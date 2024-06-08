use crate::{
    context::{Application, Selectable, UI},
    event::{
        AppEvent, CommandEvent, FormScreenEvent, MainScreenEvent, PopupCallbackAction, PopupEvent,
        PopupType, QueryboxEvent, RenderEvent, ScreenEvent,
    },
    widget::{
        popup::{Choice, Type as PopupMessageType},
        WidgetKeyHandler,
    },
    ViewMode,
};
use log::debug;
use parking_lot::Mutex;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::mpsc::Receiver;

pub struct AppEventHandler<'a> {
    app_rx: Receiver<AppEvent>,
    app_context: Arc<Mutex<Application>>,
    ui_context: Arc<Mutex<UI<'a>>>,
    should_quit: Arc<AtomicBool>,
}

impl<'a> AppEventHandler<'a> {
    pub async fn init(
        app_rx: Receiver<AppEvent>,
        context: Arc<Mutex<Application>>,
        ui_context: Arc<Mutex<UI<'a>>>,
        should_quit: Arc<AtomicBool>,
    ) {
        let mut app_router = Self {
            app_rx,
            app_context: context,
            ui_context,
            should_quit,
        };

        app_router.handle().await
    }

    async fn handle(&mut self) {
        while let Some(message) = self.app_rx.recv().await {
            match message {
                AppEvent::Run(command_event) => match command_event {
                    CommandEvent::Execute => {
                        if let Some(command) = self.ui_context.lock().selected_command() {
                            self.app_context
                                .lock()
                                .commands
                                .set_command_to_be_executed(command.to_owned());
                        }
                        self.quit();
                    }
                    CommandEvent::Insert => {
                        let mut ui = self.ui_context.lock();
                        let mut c = self.app_context.lock();
                        let command = ui.fields.collect();
                        match c.add_command(command) {
                            Ok(()) => {
                                ui.set_view(ViewMode::Main);
                                c.reload();
                            }
                            Err(error) => {
                                ui.popup.set_show_popup(true);
                                ui.popup.set_content(
                                    PopupMessageType::Error,
                                    error.to_string(),
                                    PopupCallbackAction::None,
                                );
                            }
                        }
                    }
                    CommandEvent::Edit => {
                        let mut c = self.app_context.lock();
                        let mut ui = self.ui_context.lock();

                        let edited_command = ui.fields.collect();
                        if let Some(current_command) = ui.selected_command() {
                            match c.edit_command(edited_command, current_command) {
                                Ok(()) => {
                                    ui.set_view(ViewMode::Main);
                                    c.reload()
                                }
                                Err(error) => {
                                    ui.popup.set_show_popup(true);
                                    ui.popup.set_content(
                                        PopupMessageType::Error,
                                        error.to_string(),
                                        PopupCallbackAction::None,
                                    );
                                }
                            }
                        }
                    }
                    CommandEvent::Copy => {
                        let mut ui = self.ui_context.lock();
                        if let Some(command) = ui.selected_command() {
                            if let Err(error) =
                                self.app_context.lock().copy_to_clipboard(&command.command)
                            {
                                ui.popup.set_show_popup(true);
                                ui.popup.set_content(
                                    PopupMessageType::Error,
                                    error.to_string(),
                                    PopupCallbackAction::None,
                                );
                            } else {
                                ui.clipboard_state.start()
                            }
                        }
                    }
                },
                AppEvent::Render(render_event) => match render_event {
                    RenderEvent::Main => {
                        let mut c = self.app_context.lock();
                        let mut ui = self.ui_context.lock();
                        c.reload();
                        if ui.fields.is_modified() {
                            ui.popup.set_content(
                            PopupMessageType::Warning,
                                "Wait, you didn't save your changes! Are you sure you want to quit?".to_owned(),
                                PopupCallbackAction::Render(RenderEvent::Main),
                            );
                            ui.popup.set_show_popup(true)
                        } else {
                            ui.set_view(ViewMode::Main);
                        }
                    }
                    RenderEvent::Edit => {
                        let mut ui = self.ui_context.lock();
                        if ui.selected_command().is_some() {
                            ui.set_view(ViewMode::Edit);
                            ui.fields.reset();
                            ui.fields.clear_inputs();
                            ui.set_selected_command_input();
                        }
                    }
                    RenderEvent::Insert => {
                        let mut ui = self.ui_context.lock();
                        ui.set_view(ViewMode::Insert);
                        ui.fields.clear_inputs();
                    }
                },
                AppEvent::Quit => self.quit(),
                AppEvent::Popup(popup_event) => match popup_event {
                    PopupEvent::Enable(popup_type) => {
                        let mut ui = self.ui_context.lock();
                        match popup_type {
                            PopupType::Help => {
                                ui.set_show_help(true);
                            }
                            PopupType::Dialog {
                                message,
                                callback_action,
                            } => {
                                ui.popup.set_show_popup(true);
                                ui.popup.set_content(
                                    PopupMessageType::Warning,
                                    message,
                                    callback_action,
                                );
                            }
                        }
                    }
                    PopupEvent::Answer => {
                        let mut ui = self.ui_context.lock();
                        match ui.popup.selected_choice() {
                            Choice::Ok => {
                                let mut c = self.app_context.lock();
                                match &ui.popup.content.callback {
                                    PopupCallbackAction::RemoveCommand => {
                                        if let Some(command) = ui.selected_command() {
                                            match c.remove_command(command) {
                                                Ok(()) => {
                                                    ui.popup.clear_choices();
                                                    c.reload();
                                                }
                                                Err(error) => {
                                                    ui.popup.set_show_popup(true);
                                                    ui.popup.set_content(
                                                        PopupMessageType::Error,
                                                        error.to_string(),
                                                        PopupCallbackAction::None,
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    PopupCallbackAction::None => {
                                        ui.popup.clear_choices();
                                        ui.popup.set_show_popup(false);
                                    }
                                    PopupCallbackAction::Render(screen) => match screen {
                                        RenderEvent::Main => {
                                            ui.set_view(ViewMode::Main);
                                            c.reload();
                                            ui.popup.clear_choices();
                                            ui.popup.set_show_popup(false);
                                        }
                                        RenderEvent::Edit | RenderEvent::Insert => {
                                            todo!()
                                        }
                                    },
                                }
                            }
                            Choice::Cancel => {
                                ui.popup.clear_choices();
                            }
                        };

                        // else
                        ui.popup.set_show_popup(false)
                    }
                    PopupEvent::Disable => {
                        let mut ui = self.ui_context.lock();
                        if ui.show_help() {
                            ui.set_show_help(false)
                        } else {
                            ui.popup.clear_choices();
                            ui.popup.set_show_popup(false)
                        }
                    }
                    PopupEvent::NextChoice => self.ui_context.lock().popup.next(),
                    PopupEvent::PreviousChoice => self.ui_context.lock().popup.previous(),
                },
                AppEvent::QueryBox(event) => {
                    let mut ui = self.ui_context.lock();
                    match event {
                        QueryboxEvent::Active => ui.querybox.activate_focus(),
                        QueryboxEvent::Deactive => ui.querybox.deactivate_focus(),
                        QueryboxEvent::Input(key_event) => ui.querybox.handle_input(key_event),
                    }
                }
                AppEvent::Screen(screen) => match screen {
                    ScreenEvent::Main(main_screen) => {
                        let mut c = self.app_context.lock();
                        match main_screen {
                            MainScreenEvent::NextCommand => c.commands.next(),
                            MainScreenEvent::PreviousCommand => c.commands.previous(),
                            MainScreenEvent::NextNamespace => c.namespaces.next(),
                            MainScreenEvent::PreviousNamespace => c.namespaces.previous(),
                        }
                    }
                    ScreenEvent::Form(form_screen) => {
                        let mut ui = self.ui_context.lock();
                        match form_screen {
                            FormScreenEvent::NextField => ui.fields.next(),
                            FormScreenEvent::PreviousField => ui.fields.previous(),
                            FormScreenEvent::Input(key_event) => ui.fields.handle_input(key_event),
                        }
                    }
                },
            };
        }
    }

    fn quit(&mut self) {
        debug!("quitting app");
        self.should_quit.store(true, Ordering::SeqCst)
    }
}

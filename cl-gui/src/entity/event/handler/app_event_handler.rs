use crate::{
    entity::{
        context::{ApplicationContext, Selectable, UI},
        event::{
            AppEvent, CommandEvent, FormScreenEvent, MainScreenEvent, PopupCallbackAction,
            PopupEvent, PopupType, QueryboxEvent, RenderEvent, ScreenEvent,
        },
        view_mode::ViewMode,
    },
    widget::popup::{Choice, PopupType as PopupMessageType},
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
    app_context: Arc<Mutex<ApplicationContext>>,
    ui_context: Arc<Mutex<UI<'a>>>,
    should_quit: Arc<AtomicBool>,
}

impl<'a> AppEventHandler<'a> {
    pub async fn init(
        app_rx: Receiver<AppEvent>,
        context: Arc<Mutex<ApplicationContext>>,
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
                                .set_current_command_as_callback(command);
                        }
                        self.quit();
                    }
                    CommandEvent::Insert => {
                        let mut ui = self.ui_context.lock();
                        let mut c = self.app_context.lock();
                        let command = ui.build_new_command();
                        match c.add_command(command) {
                            Ok(()) => {
                                ui.set_view_mode(ViewMode::Main);
                                ui.reset_form_field_selected_field();
                                c.reload();
                            }
                            Err(error) => {
                                ui.set_show_popup(true);
                                ui.set_popup_info(
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

                        let edited_command = ui.edit_command();
                        if let Some(current_command) = ui.selected_command() {
                            match c.add_edited_command(edited_command, current_command) {
                                Ok(()) => {
                                    ui.set_view_mode(ViewMode::Main);
                                    ui.reset_form_field_selected_field();
                                    c.reload()
                                }
                                Err(error) => {
                                    ui.set_show_popup(true);
                                    ui.set_popup_info(
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
                            if let Err(error) = self
                                .app_context
                                .lock()
                                .copy_text_to_clipboard(&command.command)
                            {
                                ui.set_show_popup(true);
                                ui.set_popup_info(
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
                        if ui.is_form_modified() {
                            ui.set_popup_info(PopupMessageType::Warning,
                                "Wait, you didn't save your changes! Are you sure you want to quit?"
                                    .to_owned(),
                                PopupCallbackAction::Render(RenderEvent::Main),
                            );
                            ui.set_show_popup(true)
                        } else {
                            ui.set_view_mode(ViewMode::Main);
                            ui.reset_form_field_selected_field();
                        }
                    }
                    RenderEvent::Edit => {
                        let mut ui = self.ui_context.lock();
                        ui.set_view_mode(ViewMode::Edit);
                        if ui.selected_command().is_some() {
                            ui.reset_form_field_selected_field();
                            ui.clear_form_fields();
                            ui.set_selected_command_input();
                        }
                    }
                    RenderEvent::Insert => {
                        let mut ui = self.ui_context.lock();
                        ui.set_view_mode(ViewMode::Insert);
                        ui.reset_form_field_selected_field();
                        ui.clear_form_fields();
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
                                ui.set_show_popup(true);
                                ui.set_popup_info(
                                    PopupMessageType::Warning,
                                    message,
                                    callback_action,
                                );
                            }
                        }
                    }
                    PopupEvent::Answer => {
                        let mut ui = self.ui_context.lock();
                        match ui.get_selected_choice() {
                            Choice::Ok => {
                                let mut c = self.app_context.lock();
                                match &ui.popup_info_mut().callback {
                                    PopupCallbackAction::DeleteCommand => {
                                        if let Some(command) = ui.selected_command() {
                                            match c.delete_selected_command(command) {
                                                Ok(()) => {
                                                    ui.clear_popup_context();
                                                    c.reload();
                                                }
                                                Err(error) => {
                                                    ui.set_show_popup(true);
                                                    ui.set_popup_info(
                                                        PopupMessageType::Error,
                                                        error.to_string(),
                                                        PopupCallbackAction::None,
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    PopupCallbackAction::None => {
                                        ui.clear_popup_context();
                                        ui.set_show_popup(false);
                                    }
                                    PopupCallbackAction::Render(screen) => match screen {
                                        RenderEvent::Main => {
                                            ui.set_view_mode(ViewMode::Main);
                                            c.reload();
                                            ui.reset_form_field_selected_field();
                                            ui.clear_popup_context();
                                            ui.set_show_popup(false);
                                        }
                                        RenderEvent::Edit | RenderEvent::Insert => {
                                            todo!()
                                        }
                                    },
                                }
                            }
                            Choice::Cancel => {
                                ui.clear_popup_context();
                            }
                        };

                        // else
                        ui.set_show_popup(false)
                    }
                    PopupEvent::Disable => {
                        let mut ui = self.ui_context.lock();
                        if ui.show_help() {
                            ui.set_show_help(false)
                        } else {
                            ui.clear_popup_context();
                            ui.set_show_popup(false)
                        }
                    }
                    PopupEvent::NextChoice => self.ui_context.lock().popup_context_mut().next(),
                    PopupEvent::PreviousChoice => {
                        self.ui_context.lock().popup_context_mut().previous()
                    }
                },
                AppEvent::QueryBox(event) => {
                    let mut ui = self.ui_context.lock();
                    match event {
                        QueryboxEvent::Active => ui.activate_querybox_focus(),
                        QueryboxEvent::Deactive => ui.deactivate_querybox_focus(),
                        QueryboxEvent::Input(key_event) => ui.handle_querybox_input(key_event),
                    }
                }
                AppEvent::Screen(screen) => match screen {
                    ScreenEvent::Main(main_screen) => {
                        let mut c = self.app_context.lock();
                        match main_screen {
                            MainScreenEvent::NextCommand => c.commands_context.next(),
                            MainScreenEvent::PreviousCommand => c.commands_context.previous(),
                            MainScreenEvent::NextNamespace => c.commands_context.namespaces.next(),
                            MainScreenEvent::PreviousNamespace => {
                                c.commands_context.namespaces.previous()
                            }
                        }
                    }
                    ScreenEvent::Form(form_screen) => {
                        let mut ui = self.ui_context.lock();
                        match form_screen {
                            FormScreenEvent::NextField => ui.next_field(),
                            FormScreenEvent::PreviousField => ui.previous_field(),
                            FormScreenEvent::Input(key_event) => ui.handle_input(key_event),
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

use super::{
    contexts::{application_context::ApplicationContext, ui_context::UIContext},
    events::app_events::{
        AppEvent, CommandEvent, FormScreenEvent, MainScreenEvent, PopupCallbackAction, PopupEvent,
        PopupType, QueryboxEvent, RenderEvent, ScreenEvent,
    },
    states::ui_state::ViewMode,
};
use crate::gui::screens::widgets::popup::Answer;
use log::debug;
use parking_lot::Mutex;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::mpsc::Receiver;

pub struct EventHandler<'a> {
    app_rx: Receiver<AppEvent>,
    app_context: Arc<Mutex<ApplicationContext>>,
    ui_context: Arc<Mutex<UIContext<'a>>>,
    should_quit: Arc<AtomicBool>,
}

impl<'a> EventHandler<'a> {
    pub async fn init(
        app_rx: Receiver<AppEvent>,
        context: Arc<Mutex<ApplicationContext>>,
        ui_context: Arc<Mutex<UIContext<'a>>>,
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
                        if let Some(command) = self.ui_context.lock().get_selected_command() {
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
                                c.reload_contexts();
                                ui.reset_form_field_selected_field();
                            }
                            Err(error) => {
                                ui.set_show_popup(true);
                                ui.set_error_popup(error.to_string());
                            }
                        }
                    }
                    CommandEvent::Edit => {
                        let mut c = self.app_context.lock();
                        let mut ui = self.ui_context.lock();

                        let edited_command = ui.edit_command();
                        if let Some(current_command) = ui.get_selected_command() {
                            match c.add_edited_command(edited_command, current_command) {
                                Ok(()) => ui.set_view_mode(ViewMode::Main),
                                Err(err) => {
                                    ui.set_show_popup(true);
                                    ui.set_error_popup(err.to_string());
                                }
                            }
                        }
                    }
                    CommandEvent::Copy => {
                        // TODO should it trigger a visual event???
                        let mut ui = self.ui_context.lock();
                        if let Some(command) = ui.get_selected_command() {
                            if let Err(error) = self
                                .app_context
                                .lock()
                                .copy_text_to_clipboard(&command.command)
                            {
                                ui.set_show_popup(true);
                                ui.set_error_popup(error.to_string())
                            }
                        }
                    }
                },
                AppEvent::Render(render_event) => match render_event {
                    RenderEvent::Main => {
                        let mut c = self.app_context.lock();
                        let mut ui = self.ui_context.lock();
                        if ui.is_form_modified() {
                            ui.set_dialog_popup(
                                "Wait, you didn't save your changes! Are you sure you want to quit?"
                                    .to_owned(),
                                PopupCallbackAction::Render(RenderEvent::Main),
                            );
                            ui.set_show_popup(true)
                        } else {
                            ui.set_view_mode(ViewMode::Main);
                            c.reload_contexts();
                            ui.reset_form_field_selected_field();
                        }
                    }
                    RenderEvent::Edit => {
                        let mut ui = self.ui_context.lock();
                        ui.set_view_mode(ViewMode::Edit);
                        if ui.get_selected_command().is_some() {
                            let screen_size = ui.screen_size();
                            ui.order_fields(screen_size);
                            ui.reset_form_field_selected_field();
                            ui.clear_form_fields();
                            ui.set_selected_command_input();
                        }
                    }
                    RenderEvent::Insert => {
                        let mut ui = self.ui_context.lock();
                        let screen_size = ui.screen_size();
                        ui.order_fields(screen_size);
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
                                ui.set_dialog_popup(message, callback_action);
                            }
                        }
                    }
                    PopupEvent::Answer => {
                        let mut ui = self.ui_context.lock();
                        if let Some(answer) = ui.get_selected_choice() {
                            match answer {
                                Answer::Ok => {
                                    let mut c = self.app_context.lock();

                                    if let Some(popup) = ui.popup() {
                                        match popup.callback() {
                                            PopupCallbackAction::DeleteCommand => {
                                                if let Some(command) = ui.get_selected_command() {
                                                    match c.delete_selected_command(command) {
                                                        Ok(()) => {
                                                            ui.clear_popup_context();
                                                            c.reload_contexts();
                                                        }
                                                        Err(error) => {
                                                            ui.set_show_popup(true);
                                                            ui.set_error_popup(error.to_string());
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
                                                    c.reload_contexts();
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
                                }
                                Answer::Cancel => {
                                    ui.clear_popup_context();
                                }
                            };
                        }
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
                    PopupEvent::NextChoice => self.ui_context.lock().next_choice(),
                    PopupEvent::PreviousChoice => self.ui_context.lock().previous_choice(),
                },
                AppEvent::QueryBox(event) => match event {
                    QueryboxEvent::Active => {
                        let mut ui = self.ui_context.lock();
                        ui.activate_querybox_focus();
                        ui.set_querybox_focus(true);
                    }
                    QueryboxEvent::Deactive => {
                        let mut ui = self.ui_context.lock();
                        ui.deactivate_querybox_focus();
                        ui.set_querybox_focus(false);
                    }
                    QueryboxEvent::Input(key_event) => {
                        self.ui_context.lock().handle_querybox_input(key_event)
                    }
                },
                AppEvent::Screen(screen) => match screen {
                    ScreenEvent::Main(main_screen) => {
                        let mut c = self.app_context.lock();
                        match main_screen {
                            MainScreenEvent::NextCommand => c.next_command(),
                            MainScreenEvent::PreviousCommand => c.previous_command(),
                            MainScreenEvent::NextNamespace => c.next_namespace(),
                            MainScreenEvent::PreviousNamespace => c.previous_namespace(),
                        }
                    }
                    ScreenEvent::Form(form_screen) => {
                        let mut ui = self.ui_context.lock();
                        match form_screen {
                            FormScreenEvent::NextField => ui.next_form_field(),
                            FormScreenEvent::PreviousField => ui.previous_form_field(),
                            FormScreenEvent::Input(key_event) => ui.handle_form_input(key_event),
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

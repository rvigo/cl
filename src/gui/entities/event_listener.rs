use super::{
    application_context::ApplicationContext,
    events::app_events::{
        AppEvents, CommandEvents, FormScreenEvent, MainScreenEvent, PopupCallbackAction,
        PopupEvent, PopupType, QueryboxEvent, RenderEvents, ScreenEvents,
    },
    ui_context::UIContext,
    ui_state::ViewMode,
};
use crate::gui::widgets::popup::Answer;
use log::debug;
use parking_lot::Mutex;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::mpsc::Receiver;

pub struct EventListener<'a> {
    app_rx: Receiver<AppEvents>,
    context: Arc<Mutex<ApplicationContext>>,
    ui_context: Arc<Mutex<UIContext<'a>>>,
    should_quit: Arc<AtomicBool>,
}

impl<'a> EventListener<'a> {
    pub async fn init(
        app_rx: Receiver<AppEvents>,
        context: Arc<Mutex<ApplicationContext>>,
        ui_state: Arc<Mutex<UIContext<'a>>>,
        should_quit: Arc<AtomicBool>,
    ) {
        let mut app_router = Self {
            app_rx,
            context,
            ui_context: ui_state,
            should_quit,
        };

        app_router.listen().await
    }

    async fn listen(&mut self) {
        while let Some(message) = self.app_rx.recv().await {
            match message {
                AppEvents::Run(command_event) => match command_event {
                    CommandEvents::Execute => {
                        if let Some(command) = self.ui_context.lock().get_selected_command() {
                            self.context.lock().set_current_command_as_callback(command);
                        }
                        self.quit();
                    }
                    CommandEvents::Insert => {
                        let mut ui = self.ui_context.lock();
                        let mut c = self.context.lock();
                        let command = ui.build_new_command();
                        match c.add_command(command) {
                            Ok(()) => self.ui_context.lock().set_view_mode(ViewMode::Main),
                            Err(error) => {
                                ui.set_show_popup(true);
                                ui.set_error_popup(error.to_string());
                            }
                        }
                    }
                    // CommandEvents::Delete => todo!(),
                    CommandEvents::Edit => {
                        let mut c = self.context.lock();
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
                },
                AppEvents::Render(render_events) => match render_events {
                    RenderEvents::Main => {
                        let mut c = self.context.lock();
                        let mut ui = self.ui_context.lock();

                        if ui.popup().is_none() || ui.get_popup_answer().is_some() {
                            ui.set_view_mode(ViewMode::Main);
                            c.reload_namespaces_state();
                            ui.reset_form_field_selected_idx();
                        }
                    }
                    RenderEvents::Edit => {
                        let mut ui = self.ui_context.lock();
                        ui.set_view_mode(ViewMode::Edit);
                        if ui.get_selected_command().is_some() {
                            ui.reset_form_field_selected_idx();
                            ui.order_fields();
                            ui.set_selected_command_input();
                        }
                    }
                    RenderEvents::Insert => {
                        let mut ui = self.ui_context.lock();
                        ui.set_view_mode(ViewMode::Insert);
                        ui.reset_form_field_selected_idx();
                        ui.order_fields();
                    }
                },
                AppEvents::Quit => self.quit(),
                AppEvents::Popup(popup_event) => match popup_event {
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
                            } // PopupType::Error { message: _ } => todo!(),
                        }
                    }
                    PopupEvent::Answer(answer) => {
                        if let Some(answer) = answer {
                            match answer {
                                Answer::Ok => {
                                    let mut ui = self.ui_context.lock();

                                    let mut c = self.context.lock();
                                    if let Some(popup) = ui.popup() {
                                        match popup.callback() {
                                            PopupCallbackAction::Delete => {
                                                if let Some(command) = ui.get_selected_command() {
                                                    match c.delete_selected_command(command) {
                                                        Ok(()) => {
                                                            ui.clear_popup_context();
                                                            c.reload_namespaces_state();
                                                        }
                                                        Err(error) => {
                                                            ui.set_show_popup(true);
                                                            ui.set_error_popup(error.to_string());
                                                        }
                                                    }
                                                }
                                            }
                                            PopupCallbackAction::None => {
                                                /* not implemented yet */
                                            }
                                        }
                                    }
                                }
                                Answer::Cancel => {
                                    self.ui_context.lock().clear_popup_context();
                                }
                            };
                        }
                        self.ui_context.lock().set_show_popup(false)
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
                },
                AppEvents::QueryBox(status) => match status {
                    QueryboxEvent::Active => {
                        let mut ui = self.ui_context.lock();
                        ui.activate_focus();
                        ui.set_querybox_focus(true);
                    }
                    QueryboxEvent::Deactive => {
                        let mut ui = self.ui_context.lock();
                        ui.deactivate_focus();
                        ui.set_querybox_focus(false);
                    }
                },
                AppEvents::Screen(screen) => match screen {
                    ScreenEvents::Main(main_screen) => {
                        let mut c = self.context.lock();
                        let ui = self.ui_context.lock();
                        match main_screen {
                            MainScreenEvent::NextCommand => c.next_command(ui.get_querybox_input()),
                            MainScreenEvent::PreviousCommand => {
                                c.previous_command(ui.get_querybox_input())
                            }
                            MainScreenEvent::NextNamespace => c.next_namespace(),
                            MainScreenEvent::PreviousNamespace => c.previous_namespace(),
                        }
                    }
                    ScreenEvents::Form(form_screen) => {
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

impl Drop for EventListener<'_> {
    fn drop(&mut self) {
        debug!("droping event listener")
    }
}

use super::{
    application_context::ApplicationContext,
    events::app_events::{
        AppEvents, CommandEvents, FormScreenEvent, MainScreenEvent, PopupCallbackAction,
        PopupEvent, PopupType, QueryboxEvent, RenderEvents, ScreenEvents,
    },
    ui_state::{UiState, ViewMode},
};
use crate::gui::widgets::popup::Answer;
use log::debug;
use parking_lot::Mutex;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::mpsc::Receiver;

pub struct AppRouter<'a> {
    app_rx: Receiver<AppEvents>,
    context: Arc<Mutex<ApplicationContext<'a>>>,
    ui_state: Arc<Mutex<UiState>>,
    should_quit: Arc<AtomicBool>,
}

impl<'a> AppRouter<'a> {
    pub async fn init(
        app_rx: Receiver<AppEvents>,
        context: Arc<Mutex<ApplicationContext<'a>>>,
        ui_state: Arc<Mutex<UiState>>,

        should_quit: Arc<AtomicBool>,
    ) {
        let mut app_router = Self {
            app_rx,
            context,
            ui_state,
            should_quit,
        };

        app_router.start().await
    }

    async fn start(&mut self) {
        while let Some(message) = self.app_rx.recv().await {
            match message {
                AppEvents::Run(command_event) => match command_event {
                    CommandEvents::Execute => {
                        self.context.lock().set_current_command_as_callback();
                        self.quit();
                    }
                    CommandEvents::Insert => {
                        let mut c = self.context.lock();
                        let command = c.build_new_command();
                        match c.add_command(command) {
                            Ok(()) => self.ui_state.lock().view_mode = ViewMode::Main,
                            Err(error) => {
                                self.ui_state
                                    .lock()
                                    .show_popup
                                    .store(true, Ordering::SeqCst);
                                c.set_error_popup(error.to_string());
                            }
                        }
                    }
                    CommandEvents::Delete => todo!(),
                    CommandEvents::Edit => {
                        let mut c = self.context.lock();
                        let edited_command = c.edit_command();
                        match c.add_edited_command(edited_command) {
                            Ok(()) => self.ui_state.lock().view_mode = ViewMode::Main,
                            Err(err) => {
                                self.ui_state
                                    .lock()
                                    .show_popup
                                    .store(true, Ordering::SeqCst);
                                c.set_error_popup(err.to_string());
                            }
                        }
                    }
                },
                AppEvents::Render(render_events) => match render_events {
                    RenderEvents::Main => {
                        let mut c = self.context.lock();
                        if c.popup().is_none() || c.get_popup_answer().is_some() {
                            self.ui_state.lock().view_mode = ViewMode::Main;
                            c.enter_main_mode();
                        }
                    }
                    RenderEvents::Edit => {
                        self.ui_state.lock().view_mode = ViewMode::Edit;
                        self.context.lock().enter_edit_mode();
                    }
                    RenderEvents::Insert => {
                        self.ui_state.lock().view_mode = ViewMode::Insert;
                        self.context.lock().enter_insert_mode();
                    }
                },
                AppEvents::Quit => self.quit(),
                AppEvents::Popup(popup_event) => match popup_event {
                    PopupEvent::Enable(popup_type) => match popup_type {
                        PopupType::Help => {
                            self.ui_state.lock().show_help.store(true, Ordering::SeqCst);
                            self.context.lock().set_show_help(true);
                        }
                        PopupType::Dialog {
                            message,
                            callback_action,
                        } => {
                            self.ui_state
                                .lock()
                                .show_popup
                                .store(true, Ordering::SeqCst);
                            self.context
                                .lock()
                                .set_dialog_popup(message, callback_action);
                        }
                        PopupType::Error { message: _ } => todo!(),
                    },
                    PopupEvent::Answer(answer) => {
                        if let Some(answer) = answer {
                            match answer {
                                Answer::Ok => {
                                    let mut c = self.context.lock();
                                    if let Some(popup) = c.popup() {
                                        match popup.callback() {
                                            PopupCallbackAction::Delete => {
                                                match c.delete_selected_command() {
                                                    Ok(()) => {
                                                        c.clear_popup_context();
                                                        c.reload_namespaces_state();
                                                    }
                                                    Err(error) => {
                                                        self.ui_state
                                                            .lock()
                                                            .show_popup
                                                            .store(true, Ordering::SeqCst);
                                                        c.set_error_popup(error.to_string());
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
                                    self.context.lock().clear_popup_context();
                                }
                            };
                        }
                        self.ui_state
                            .lock()
                            .show_popup
                            .store(false, Ordering::SeqCst)
                    }
                    PopupEvent::Disable => {
                        let mut c = self.context.lock();
                        if c.show_help() {
                            c.set_show_help(false);
                            self.ui_state
                                .lock()
                                .show_help
                                .store(false, Ordering::SeqCst)
                        } else {
                            c.clear_popup_context();
                            self.ui_state
                                .lock()
                                .show_popup
                                .store(false, Ordering::SeqCst)
                        }
                    }
                },
                AppEvents::QueryBox(status) => match status {
                    QueryboxEvent::Active => {
                        self.context.lock().activate_focus();
                        self.ui_state
                            .lock()
                            .query_box_active
                            .store(true, Ordering::SeqCst);
                    }
                    QueryboxEvent::Deactive => {
                        self.context.lock().deactivate_focus();
                        self.ui_state
                            .lock()
                            .query_box_active
                            .store(false, Ordering::SeqCst);
                    }
                },
                AppEvents::Screen(screen) => match screen {
                    ScreenEvents::Main(main_screen) => {
                        let mut c = self.context.lock();
                        match main_screen {
                            MainScreenEvent::NextCommand => c.next_command(),
                            MainScreenEvent::PreviousCommand => c.previous_command(),
                            MainScreenEvent::NextNamespace => c.next_namespace(),
                            MainScreenEvent::PreviousNamespace => c.previous_namespace(),
                        }
                    }
                    ScreenEvents::Form(form_screen) => match form_screen {
                        FormScreenEvent::NextField => self.context.lock().next_form_field(),
                        FormScreenEvent::PreviousField => self.context.lock().previous_form_field(),
                        FormScreenEvent::Input(key_event) => {
                            self.context.lock().handle_form_input(key_event)
                        }
                    },
                },
            };
        }
    }

    fn quit(&mut self) {
        debug!("quitting app");
        self.should_quit.store(true, Ordering::SeqCst)
    }
}

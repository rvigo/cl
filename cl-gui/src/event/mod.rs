mod app_event;
pub mod handler;
mod input_event;

pub use app_event::AppEvent;
pub use app_event::CommandEvent;
pub use app_event::DialogType;
pub use app_event::FormScreenEvent;
pub use app_event::MainScreenEvent;
pub use app_event::PopupCallbackAction;
pub use app_event::PopupEvent;
pub use app_event::PopupType;
pub use app_event::QueryboxEvent;
pub use app_event::RenderEvent;
pub use app_event::ScreenEvent;
pub use input_event::InputEvent;

#[macro_export]
macro_rules! impl_event {
    // needs a TX and RX in scope
    ($struct:tt) => {
        impl $struct {
            pub fn init() {
                let (tx, rx) = mpsc::unbounded_channel();
                TX.init(tx);
                RX.init(rx);
            }

            pub fn get() -> mpsc::UnboundedReceiver<Self> {
                RX.get()
            }

            pub fn emit(self) {
                TX.send(self).ok();
            }
        }
    };
}

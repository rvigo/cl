mod help;
mod info;
mod navigation_info;
mod querybox;

pub use help::Help;
pub use info::Info;
pub use navigation_info::NavigationInfo;
pub use querybox::QueryBox;

use tui::widgets::Widget;

pub trait StatusBarItem: Clone + Widget {}

macro_rules! status_bar_item {
    ($name:ty) => {
        impl StatusBarItem for $name {}
    };
}

status_bar_item!(help::Help);
status_bar_item!(info::Info);
status_bar_item!(navigation_info::NavigationInfo);
status_bar_item!(querybox::QueryBox<'_>);

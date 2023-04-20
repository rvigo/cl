pub mod base_widget;
pub mod display;
pub mod fields;
pub mod help_footer;
pub mod help_popup;
pub mod highlight;
pub mod list;
pub mod navigation_footer;
pub mod popup;
pub mod querybox;
pub mod text_field;

use self::base_widget::BaseWidget;
use super::Screen;
use tui::{
    backend::Backend,
    layout::Alignment,
    style::Style,
    widgets::{Block, BorderType, Borders, Widget},
    Frame,
};

pub trait Footer: Clone + Widget {}

pub trait ScreenExt<B>: Screen<B>
where
    B: Backend,
{
    fn render_base<F, H>(&self, frame: &mut Frame<B>, footer: Option<&F>, help_footer: H)
    where
        F: Footer,
        H: Footer,
    {
        let screen_size = self.get_screen_size();
        let base_widget = BaseWidget::new(&screen_size, footer, help_footer);
        frame.render_widget(base_widget, frame.size());
    }
}

impl<T, B> ScreenExt<B> for T
where
    T: Screen<B>,
    B: Backend,
{
}

pub trait Component {}

pub trait WidgetExt: Component {
    fn default_block<'a, S>(&self, title: S) -> Block<'a>
    where
        S: Into<String>,
    {
        let title = title.into();
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(format!(" {title} "))
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Plain)
    }
}

impl<T> WidgetExt for T where T: Component {}

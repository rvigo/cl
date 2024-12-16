use super::{statusbar::Info, Component};
use crate::state::ClipboardState;
use tui::{buffer::Buffer, layout::Rect, widgets::Widget};

pub struct ClibpoardWidget<'clipboard> {
    state: &'clipboard mut ClipboardState,
}

impl Component for ClibpoardWidget<'_> {}

impl<'clipboard> ClibpoardWidget<'clipboard> {
    pub fn new(state: &'clipboard mut ClipboardState) -> Self {
        Self { state }
    }
}

impl<'clipboard> Widget for ClibpoardWidget<'clipboard> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let yanked = self.state.yanked();
        if yanked {
            let info = Info::new("Command copied to clipboard!");

            info.render(area, buf);
        }

        self.state.check();
    }
}

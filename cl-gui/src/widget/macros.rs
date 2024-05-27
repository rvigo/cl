#[macro_export]
macro_rules! centered_rect {
    ($width: expr, $height: expr, $area: expr) => {{
        use tui::layout::{Constraint, Direction, Layout};

        let height = if $height > 100 { 100 } else { $height };
        let width = if $width > 100 { 100 } else { $width };

        let new_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage((100 - height) / 2),
                    Constraint::Percentage(height),
                    Constraint::Percentage((100 - height) / 2),
                ]
                .as_ref(),
            )
            .split($area);
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage((100 - width) / 2),
                    Constraint::Percentage(width),
                    Constraint::Percentage((100 - width) / 2),
                ]
                .as_ref(),
            )
            .split(new_area[1])[1]
    }};
}

#[macro_export]
macro_rules! default_block {
    ($title:expr) => {{
        use tui::{
            layout::Alignment,
            style::Style,
            widgets::{Block, BorderType, Borders},
        };

        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(format!(" {} ", $title))
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Plain)
    }};
}

#[macro_export]
macro_rules! register {
    ($what:ident, $($key:expr => $value:expr),+ $(,)*) => {
       $(
            $what.insert($key, $value);
        )+
    };
}

#[macro_export]
macro_rules! render {
    ($frame:ident, $({ $what:ident, $_where:expr}),* $(,)?) => {
        $(
            $frame.render_widget($what, $_where);
        )+
    };
}

#[macro_export]
macro_rules! display_widget {
    ($title:expr, $content:expr, $trim:expr, $highlight:expr) => {
        $crate::widget::display::DisplayWidget::new($content, $trim, $highlight)
            .block(default_block!($title))
    };

    ($title:expr, $content:expr, $trim:expr, $highlight:expr, $query:expr) => {
        $crate::widget::display::DisplayWidget::new($content, $trim, $highlight)
            .block(default_block!($title))
            .highlight($query)
    };
}

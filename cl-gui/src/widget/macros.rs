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
            widgets::{Block, BorderType, Borders, Padding},
        };
        use $crate::DEFAULT_TEXT_COLOR;

        Block::default()
            .borders(Borders::TOP | Borders::RIGHT)
            .style(Style::default().fg(DEFAULT_TEXT_COLOR))
            .title(format!(" {} ", $title))
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(2))
    }};

    () => {{
        use tui::{
            style::Style,
            widgets::{Block, BorderType, Borders, Padding},
        };

        Block::default()
            .borders(Borders::TOP | Borders::RIGHT)
            .style(
                Style::default()
                    .fg(DEFAULT_TEXT_COLOR)
                    .bg(DEFAULT_BACKGROUND_COLOR),
            )
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(2))
    }};
}

#[macro_export]
macro_rules! dummy_block {
    () => {{
        use tui::widgets::{Block, BorderType, Borders, Padding};

        Block::default()
            .borders(Borders::TOP | Borders::RIGHT)
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(2))
    }};
}

#[macro_export]
macro_rules! default_widget_block {
    () => {{
        use tui::{
            layout::Alignment,
            style::{Modifier, Style},
            widgets::{Block, BorderType, Borders, Padding},
        };
        use $crate::theme::{
            DEFAULT_BACKGROUND_COLOR, DEFAULT_TEXT_COLOR, DEFAULT_WIDGET_NAME_COLOR,
        };

        Block::default()
            .borders(Borders::TOP | Borders::RIGHT)
            .title_alignment(Alignment::Left)
            .title_style(
                Style::default()
                    .fg(DEFAULT_WIDGET_NAME_COLOR)
                    .add_modifier(Modifier::BOLD),
            )
            .style(
                Style::default()
                    .fg(DEFAULT_TEXT_COLOR)
                    .bg(DEFAULT_BACKGROUND_COLOR),
            )
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(2))
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
    ($frame:ident, $({ $what:expr, $_where:expr}),* $(,)?) => {
        $(
            $frame.render_widget($what, $_where);
        )+
    };
}

#[macro_export]
macro_rules! display_widget {
    ($type:expr, $content:expr, $trim:expr, $highlight:expr) => {
        $crate::widget::DisplayWidget::new($type, $content, $trim, $highlight)
            .block(default_widget_block!())
    };

    ($type:expr,  $content:expr, $trim:expr, $highlight:expr, $query:expr) => {{
        use $crate::default_widget_block;
        let mut widget = $crate::widget::DisplayWidget::new($type, $content, $trim, $highlight)
            .block(default_widget_block!());

        widget.highlight($query);

        widget
    }};
}

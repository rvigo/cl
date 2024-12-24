pub mod macros {
    #[macro_export]
    macro_rules! default_popup_block {
        ($popup_type:expr) => {{
            use tui::{
                layout::Alignment,
                style::{Color, Modifier, Style},
                widgets::{Block, BorderType, Borders, Padding},
            };
            use $crate::theme::{DEFAULT_BACKGROUND_COLOR, DEFAULT_TEXT_COLOR};
            use $crate::widget::popup::Type;

            let style = match $popup_type {
                Type::Error => Style::default()
                    .fg(Color::Rgb(243, 139, 168))
                    .add_modifier(Modifier::BOLD),

                Type::Warning => Style::default()
                    .fg(Color::Rgb(249, 226, 175))
                    .add_modifier(Modifier::BOLD),

                Type::Help => Style::default()
                    .fg(Color::Rgb(166, 227, 161))
                    .add_modifier(Modifier::BOLD),
            };
            Block::default()
                .borders(Borders::ALL)
                .title($popup_type.to_string())
                .title_alignment(Alignment::Left)
                .title_style(style)
                .style(
                    Style::default()
                        .fg(DEFAULT_TEXT_COLOR)
                        .bg(DEFAULT_BACKGROUND_COLOR),
                )
                .border_type(BorderType::Rounded)
                .padding(Padding::horizontal(2))
        }};
    }
}

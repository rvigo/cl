use tui::{
    style::Style,
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn render_helper_footer() -> Paragraph<'static> {
    let help_content =
        "Quit <Q>       New command <I>       Delete <D>       Edit command <E>       \
    Right <TAB>       Left <BACKTAB>       Up <ArrowUp>       Down <ArrowDown>";
    Paragraph::new(help_content).block(
        Block::default()
            .style(Style::default())
            .borders(Borders::ALL)
            .title(" Help ")
            .border_type(BorderType::Plain),
    )
}

pub fn render_insert_helper_footer() -> Paragraph<'static> {
    let help_content = "Return <ESC>       Right <TAB>       Left <BACKTAB>       Create <Enter>";
    Paragraph::new(help_content).block(
        Block::default()
            .style(Style::default())
            .borders(Borders::ALL)
            .title(" Help ")
            .border_type(BorderType::Plain),
    )
}

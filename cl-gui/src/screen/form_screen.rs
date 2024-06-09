use super::Screen;
use crate::{
    context::{Application, UI},
    default_widget_block, popup, render,
    terminal::TerminalSizeExt,
    view_mode::ViewMode,
    widget::{
        popup::{HelpPopup, RenderPopup},
        statusbar::{Help, Info},
        text_field::FieldType,
        TextField,
    },
    DEFAULT_BACKGROUND_COLOR, DEFAULT_HIGH_LIGHT_COLOR, DEFAULT_TEXT_COLOR,
    DEFAULT_WIDGET_NAME_COLOR,
};
use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap},
    Frame,
};

pub struct FormScreen;

impl Screen for FormScreen {
    fn render(&self, frame: &mut Frame, _: &mut Application, ui_context: &mut UI) {
        let terminal_size = frame.size().as_terminal_size();
        ui_context.fields.sort(&terminal_size);

        let view_mode = ui_context.view_mode();
        let fields = ui_context.fields.inner();
        let center = if ui_context.fields.is_modified() {
            Some(Info::new("MODIFIED"))
        } else {
            None
        };
        let right = Help::default();
        match terminal_size {
            _ => render_medium_form(frame, &view_mode, &fields, center, right),
        }

        if ui_context.show_help() {
            let help_popup = popup!(&ui_context.view_mode());
            frame.render_popup(help_popup, frame.size());
        }

        if ui_context.popup.show_popup() {
            let popup_ctx = &mut ui_context.popup;
            let content = &popup_ctx.content;
            let choices = popup_ctx.choices();
            let popup = popup!(content, choices);
            frame.render_stateful_popup(popup, frame.size(), popup_ctx);
        }
    }
}

fn render_medium_form(
    frame: &mut Frame,
    view_mode: &ViewMode,
    fields: &[TextField],
    center: Option<Info>,
    right: Help,
) {
    let drawable_area = [
        Constraint::Length(5), // drawable area
        Constraint::Max(3),    // footer
    ];

    let areas = [
        Constraint::Percentage(25), // name & preview
        Constraint::Percentage(75), // right side
    ];
    let constraints = [
        Constraint::Length(5), //Alias & Namespace
        Constraint::Min(10),   //Command
        Constraint::Max(5),    //Desc & Tags
    ];

    let drawable_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(drawable_area)
        .split(frame.size());

    let form_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(areas)
        .split(drawable_chunks[0]);

    let left_side = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(3), Constraint::Length(5)])
        .split(form_chunks[0]);
    let app_name = Paragraph::new(Text::styled(
        format!("cl - {}", view_mode),
        Style::default()
            .fg(DEFAULT_WIDGET_NAME_COLOR)
            .add_modifier(Modifier::BOLD | Modifier::ITALIC),
    ))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::TOP | Borders::RIGHT)
            .style(
                Style::default()
                    .bg(DEFAULT_BACKGROUND_COLOR)
                    .fg(DEFAULT_TEXT_COLOR),
            )
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(2)),
    );

    let right_side = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(form_chunks[1]);

    let first_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(right_side[0]);
    let second_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(right_side[1]);
    let third_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(right_side[2]);

    let map = command_preview(fields);

    let preview = Paragraph::new(map)
        .wrap(Wrap { trim: true })
        .block(default_widget_block!("Preview"));

    fields.iter().cloned().for_each(|field| {
        let area = match field.field_type() {
            FieldType::Alias => first_row[0],
            FieldType::Namespace => first_row[1],
            FieldType::Command => second_row[0],
            FieldType::Description => third_row[0],
            FieldType::Tags => third_row[1],
        };
        render! {frame, {field, area}}
    });

    //
    let footer = Block::default()
        .borders(Borders::BOTTOM | Borders::RIGHT)
        .style(
            Style::default()
                .bg(DEFAULT_BACKGROUND_COLOR)
                .fg(DEFAULT_TEXT_COLOR),
        );

    let statusbar_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(footer.inner(drawable_chunks[1]));
    render! (
        frame,
        { app_name, left_side[0]},
        { preview, left_side[1]}
    );

    render!(frame, { footer, drawable_chunks[1] });
    if let Some(center_statusbar_item) = center {
        render!(frame, {center_statusbar_item, statusbar_layout[1]}, );
    }
    render! { frame, {right, statusbar_layout[2]}};
}

fn command_preview<'a>(fields: &[TextField<'a>]) -> Vec<Line<'a>> {
    let mut alias: (FieldType, String, bool) = (FieldType::Alias, String::default(), false);
    let mut namespace: (FieldType, String, bool) = (FieldType::Namespace, String::default(), false);
    let mut command: (FieldType, String, bool) = (FieldType::Command, String::default(), false);
    let mut description: (FieldType, String, bool) =
        (FieldType::Description, String::default(), false);
    let mut tags: (FieldType, String, bool) = (FieldType::Tags, String::default(), false);

    fields.iter().for_each(|field| match field.field_type() {
        FieldType::Alias => alias = (field.field_type(), field.text(), field.in_focus),
        FieldType::Namespace => namespace = (field.field_type(), field.text(), field.in_focus),
        FieldType::Command => command = (field.field_type(), field.text(), field.in_focus),
        FieldType::Description => description = (field.field_type(), field.text(), field.in_focus),
        FieldType::Tags => tags = (field.field_type(), field.text(), field.in_focus),
    });
    [
        highlight(alias.0, alias.1, alias.2),
        highlight(namespace.0, namespace.1, namespace.2),
        highlight(command.0, command.1, command.2),
        highlight(description.0, description.1, description.2),
        highlight_tags(tags.1, tags.2),
    ]
    .iter()
    .flat_map(|v| v.iter().cloned())
    .collect::<Vec<Line<'a>>>()
}

fn highlight<'a>(field_type: FieldType, text: String, highlight: bool) -> Vec<Line<'a>> {
    let field_name = format!("{}: ", field_type);
    let space: Line = Line::from("");
    if highlight {
        vec![
            Line::styled(
                field_name,
                Style::default()
                    .fg(DEFAULT_HIGH_LIGHT_COLOR)
                    .add_modifier(Modifier::BOLD),
            ),
            Line::from(text.to_string()),
            space,
        ]
    } else {
        vec![
            Line::styled(field_name, Style::default().fg(DEFAULT_HIGH_LIGHT_COLOR)),
            Line::from(text.to_string()),
            space,
        ]
    }
}

fn highlight_tags<'a>(tags: String, highlight: bool) -> Vec<Line<'a>> {
    let content = &tags.split(',').collect::<Vec<_>>();
    let content = content
        .iter()
        .map(|s| format!(" - {}", s.trim()))
        .collect::<Vec<_>>();
    let content = content.into_iter().map(Line::from).collect::<Vec<_>>();
    let values = if highlight {
        vec![Line::styled(
            FieldType::Tags.to_string(),
            Style::default()
                .fg(DEFAULT_HIGH_LIGHT_COLOR)
                .add_modifier(Modifier::BOLD),
        )]
        .into_iter()
        .chain(content.clone())
        .collect::<Vec<_>>()
    } else {
        vec![Line::styled(
            FieldType::Tags.to_string(),
            Style::default().fg(DEFAULT_HIGH_LIGHT_COLOR),
        )]
        .into_iter()
        .chain(content.clone())
        .collect::<Vec<_>>()
    };

    values
        .into_iter()
        .chain(vec![Line::from("")])
        .collect::<Vec<_>>()
}

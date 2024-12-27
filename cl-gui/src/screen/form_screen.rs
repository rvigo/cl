use super::Screen;
use crate::{
    context::{Application, UI},
    default_widget_block, maybe_render, render,
    terminal::TerminalSizeExt,
    theme::{
        DEFAULT_BACKGROUND_COLOR, DEFAULT_HIGHLIGHT_COLOR, DEFAULT_TEXT_COLOR,
        DEFAULT_WIDGET_NAME_COLOR,
    },
    view_mode::ViewMode,
    widget::{
        statusbar::{Help, Info},
        text_field::FieldType,
        TextField,
    },
};
use itertools::Itertools;
use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap},
    Frame,
};

pub struct FormScreen;

impl Screen for FormScreen {
    fn render(&mut self, frame: &mut Frame, _: &mut Application, ui: &mut UI) {
        let terminal_size = frame.size().as_terminal_size();
        ui.fields.sort(&terminal_size);

        let view_mode = ui.view_mode();
        let fields = ui.fields.inner();
        let center = if ui.fields.is_modified() {
            Some(Info::new("MODIFIED"))
        } else {
            None
        };
        let right = Help::default();
        // temporally disable match expression
        // match terminal_size {
        //     _ => render_medium_form(frame, &view_mode, &fields, center, right),
        // }
        render_medium_form(frame, &view_mode, &fields, center, right);

        maybe_render! { frame , {ui.popup.active_popup(), frame.size(), &mut ui.popup} };
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
        Constraint::Percentage(20), // name & preview
        Constraint::Percentage(80), // right side
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
        .block(default_widget_block!().title("Preview"));

    fields.iter().cloned().for_each(|field| {
        let area = match field.field_type() {
            FieldType::Alias => first_row[0],
            FieldType::Namespace => first_row[1],
            FieldType::Command => second_row[0],
            FieldType::Description => third_row[0],
            FieldType::Tags => third_row[1],
            _ => panic!("Invalid field type"),
        };
        render! { frame, {field, area} }
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

    render! {
            frame,
            { app_name, left_side[0]},
            { preview, left_side[1]}
    };

    render! { frame, { footer, drawable_chunks[1] }};
    maybe_render! { frame, {center, statusbar_layout[1]} };
    render! { frame, {right, statusbar_layout[2]}};
}

fn command_preview<'form>(fields: &[TextField<'form>]) -> Vec<Line<'form>> {
    fn extract_field<'a, T>(
        fields: &[TextField<'a>],
        field_type: FieldType,
        default: T,
        extractor: impl Fn(&TextField<'a>) -> T,
    ) -> (FieldType, T, bool) {
        fields
            .iter()
            .find(|field| field.field_type() == field_type)
            .map(|field| (field.field_type(), extractor(field), field.in_focus))
            .unwrap_or((field_type, default, false))
    }

    let alias = extract_field(fields, FieldType::Alias, String::default(), |field| {
        field.text()
    });
    let namespace = extract_field(fields, FieldType::Namespace, String::default(), |field| {
        field.text()
    });
    let command = extract_field(fields, FieldType::Command, Vec::<String>::new(), |field| {
        field.lines()
    });
    let description = extract_field(fields, FieldType::Description, String::default(), |field| {
        field.text()
    });
    let tags = extract_field(fields, FieldType::Tags, None, |field| {
        if field.text().is_empty() {
            None
        } else {
            Some(field.text())
        }
    });

    // Collect and flatten the highlighted lines
    [
        highlight(alias.into()),
        highlight(namespace.into()),
        highlight_lines(command.into()),
        highlight(description.into()),
        highlight_tags(tags.into()),
    ]
    .iter()
    .flat_map(|v| v.iter().cloned())
    .collect()
}

fn highlight<'line>(preview: PreviewLine<String>) -> Vec<Line<'line>> {
    let field_name = format!("{}: ", preview.field_type);
    let space: Line = Line::from("");

    let style = Style::default()
        .fg(DEFAULT_HIGHLIGHT_COLOR)
        .apply_if(preview.highlight, |style| {
            style.add_modifier(Modifier::BOLD)
        });

    let styled_name = Line::styled(field_name, style);
    let content_line = Line::from(preview.content.lines().join("\n"));

    vec![styled_name, content_line, space]
}

fn highlight_lines<'a>(preview: PreviewLine<Vec<String>>) -> Vec<Line<'a>> {
    let style = Style::default()
        .fg(DEFAULT_HIGHLIGHT_COLOR)
        .apply_if(preview.highlight, |style| {
            style.add_modifier(Modifier::BOLD)
        });

    let content = preview
        .content
        .into_iter()
        .map(Line::from)
        .collect::<Vec<_>>();
    let header = Line::styled(format!("{}:", preview.field_type), style);

    let mut values = vec![header];
    values.extend(content);

    const MAX_LINES: usize = 5;
    if values.len() > MAX_LINES {
        values.truncate(MAX_LINES);
        values.push(Line::from("..."));
    }

    values.push(Line::from(""));

    values
}

fn highlight_tags<'a>(preview: PreviewLine<Option<String>>) -> Vec<Line<'a>> {
    let content = preview.content.as_ref().map_or(vec![], |tags| {
        tags.split(',')
            .map(|s| format!(" - {}", s.trim()))
            .collect::<Vec<_>>()
    });

    let style = Style::default()
        .fg(DEFAULT_HIGHLIGHT_COLOR)
        .apply_if(preview.highlight, |style| {
            style.add_modifier(Modifier::BOLD)
        });

    let line = Line::styled(preview.field_type.to_string(), style);

    let mut values = vec![line];

    values.extend(content.into_iter().map(Line::from));
    values.push(Line::from(""));

    values
}

trait StyleExt {
    fn apply_if(self, condition: bool, apply: impl Fn(Self) -> Self) -> Self
    where
        Self: Sized;
}

impl StyleExt for Style {
    fn apply_if(self, condition: bool, apply: impl Fn(Self) -> Self) -> Self {
        if condition {
            apply(self)
        } else {
            self
        }
    }
}

struct PreviewLine<C> {
    field_type: FieldType,
    content: C,
    highlight: bool,
}

impl PreviewLine<String> {
    fn new(field_type: FieldType, content: String, highlight: bool) -> Self {
        Self {
            field_type,
            content,
            highlight,
        }
    }
}

impl PreviewLine<Vec<String>> {
    fn new(field_type: FieldType, content: Vec<String>, highlight: bool) -> Self {
        Self {
            field_type,
            content,
            highlight,
        }
    }
}

impl PreviewLine<Option<String>> {
    fn new(field_type: FieldType, content: Option<String>, highlight: bool) -> Self {
        Self {
            field_type,
            content,
            highlight,
        }
    }
}

impl From<(FieldType, String, bool)> for PreviewLine<String> {
    fn from((field_type, content, highlight): (FieldType, String, bool)) -> Self {
        Self::new(field_type, content, highlight)
    }
}

impl From<(FieldType, Vec<String>, bool)> for PreviewLine<Vec<String>> {
    fn from((field_type, content, highlight): (FieldType, Vec<String>, bool)) -> Self {
        Self::new(field_type, content, highlight)
    }
}

impl From<(FieldType, Option<String>, bool)> for PreviewLine<Option<String>> {
    fn from((field_type, content, highlight): (FieldType, Option<String>, bool)) -> Self {
        Self::new(field_type, content, highlight)
    }
}

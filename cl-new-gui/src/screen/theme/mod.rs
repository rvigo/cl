use tui::style::Color as TuiColor;

const DEFAULT_TEXT_COLOR: TuiColor = TuiColor::Rgb(205, 214, 244);
const DEFAULT_WIDGET_NAME_COLOR: TuiColor = TuiColor::Rgb(203, 166, 247);
const DEFAULT_SELECTED_COLOR: TuiColor = TuiColor::Rgb(203, 166, 247);
const DEFAULT_HIGHLIGHT_COLOR: TuiColor = TuiColor::Rgb(180, 190, 254);
const DEFAULT_BACKGROUND_COLOR: TuiColor = TuiColor::Rgb(30, 30, 46);
const DEFAULT_INFO_COLOR: TuiColor = TuiColor::Rgb(148, 226, 213);
const DEFAULT_CURSOR_COLOR: TuiColor = TuiColor::Rgb(245, 224, 220);
const DEFAULT_INACTIVE_TEXTBOX_COLOR: TuiColor = TuiColor::Rgb(108, 112, 134);

#[derive(Clone)]
pub enum Color {
    Rgb(u8, u8, u8),
    AnsiValue(u8),
    Named(String),
}

impl From<TuiColor> for Color {
    fn from(value: TuiColor) -> Self {
        match value {
            TuiColor::Rgb(r, g, b) => Self::Rgb(r, g, b),
            _ => panic!("Unsupported color type"),
        }
    }
}

impl From<Color> for TuiColor {
    fn from(value: Color) -> Self {
        match value {
            Color::Rgb(r, g, b) => TuiColor::Rgb(r, g, b),
            _ => panic!("Unsupported color type"),
        }
    }
}

#[derive(Clone)]
pub struct Theme {
    pub text_color: Color,
    pub widget_name_color: Color,
    pub selected_color: Color,
    pub highlight_color: Color,
    pub background_color: Color,
    pub info_color: Color,
    pub cursor_color: Color,
    pub inactive_textbox_color: Color,
}

impl Theme {
    pub fn load() -> Self {
        Self::default()
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            text_color: DEFAULT_TEXT_COLOR.into(),
            widget_name_color: DEFAULT_WIDGET_NAME_COLOR.into(),
            selected_color: DEFAULT_SELECTED_COLOR.into(),
            highlight_color: DEFAULT_HIGHLIGHT_COLOR.into(),
            background_color: DEFAULT_BACKGROUND_COLOR.into(),
            info_color: DEFAULT_INFO_COLOR.into(),
            cursor_color: DEFAULT_CURSOR_COLOR.into(),
            inactive_textbox_color: DEFAULT_INACTIVE_TEXTBOX_COLOR.into(),
        }
    }
}

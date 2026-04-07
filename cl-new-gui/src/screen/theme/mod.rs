use tui::style::Color as TuiColor;

const DEFAULT_TEXT_COLOR: TuiColor = TuiColor::Rgb(205, 214, 244);
const DEFAULT_WIDGET_NAME_COLOR: TuiColor = TuiColor::Rgb(203, 166, 247);
const DEFAULT_SELECTED_COLOR: TuiColor = TuiColor::Rgb(203, 166, 247);
const DEFAULT_HIGHLIGHT_COLOR: TuiColor = TuiColor::Rgb(180, 190, 254);
const DEFAULT_BACKGROUND_COLOR: TuiColor = TuiColor::Rgb(30, 30, 46);
const DEFAULT_INFO_COLOR: TuiColor = TuiColor::Rgb(148, 226, 213);
const DEFAULT_CURSOR_COLOR: TuiColor = TuiColor::Rgb(245, 224, 220);
const DEFAULT_INACTIVE_TEXTBOX_COLOR: TuiColor = TuiColor::Rgb(108, 112, 134);

#[derive(Clone, Copy)]
pub enum Color {
    Rgb(u8, u8, u8),
}

impl From<TuiColor> for Color {
    fn from(value: TuiColor) -> Self {
        match value {
            TuiColor::Rgb(r, g, b) => Self::Rgb(r, g, b),
            other => {
                log::error!(
                    "unsupported TuiColor variant: {:?}, falling back to white",
                    other
                );
                Self::Rgb(255, 255, 255)
            }
        }
    }
}

impl From<Color> for TuiColor {
    fn from(value: Color) -> Self {
        let Color::Rgb(r, g, b) = value;
        TuiColor::Rgb(r, g, b)
    }
}

#[derive(Clone, Copy)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_tui_color_converts_to_color() {
        let tui_color = TuiColor::Rgb(10, 20, 30);
        let color: Color = tui_color.into();
        let Color::Rgb(r, g, b) = color;
        assert_eq!((r, g, b), (10, 20, 30));
    }

    #[test]
    fn color_converts_to_tui_color() {
        let color = Color::Rgb(10, 20, 30);
        let tui_color: TuiColor = color.into();
        assert_eq!(tui_color, TuiColor::Rgb(10, 20, 30));
    }

    #[test]
    fn unsupported_tui_color_falls_back_to_white() {
        let color: Color = TuiColor::Yellow.into();
        let Color::Rgb(r, g, b) = color;
        assert_eq!((r, g, b), (255, 255, 255));
    }

    #[test]
    fn default_theme_loads_without_panic() {
        let theme = Theme::default();
        let Color::Rgb(r, g, b) = theme.text_color;
        assert_eq!((r, g, b), (205, 214, 244));
    }
}

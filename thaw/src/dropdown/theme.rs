use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct DropdownTheme {
    pub background_color: String,
    pub item_color_hover: String,
    pub font_color_disabled: String,
}

impl ThemeMethod for DropdownTheme {
    fn light() -> Self {
        Self {
            background_color: "#fff".into(),
            item_color_hover: "#f3f5f6".into(),
            font_color_disabled: "#c2c2c2".into(),
        }
    }

    fn dark() -> Self {
        Self {
            background_color: "#48484e".into(),
            item_color_hover: "#ffffff17".into(),
            font_color_disabled: "#ffffff61".into(),
        }
    }
}

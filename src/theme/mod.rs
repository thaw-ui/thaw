mod common;

use self::common::CommonTheme;
use crate::{
    mobile::{NavBarTheme, TabbarTheme},
    utils::Provider,
    AlertTheme, AutoCompleteTheme, AvatarTheme, BreadcrumbTheme, ButtonTheme, ColorPickerTheme,
    InputTheme, MenuTheme, MessageTheme, ProgressTheme, SelectTheme, SkeletionTheme, SliderTheme,
    SwitchTheme, TableTheme, TagTheme, TypographyTheme, UploadTheme,
};
use leptos::*;

pub trait ThemeMethod {
    fn light() -> Self;
    fn dark() -> Self;
}

#[derive(Clone)]
pub struct Theme {
    pub name: String,
    pub common: CommonTheme,
    pub button: ButtonTheme,
    pub input: InputTheme,
    pub menu: MenuTheme,
    pub table: TableTheme,
    pub alert: AlertTheme,
    pub skeletion: SkeletionTheme,
    pub tag: TagTheme,
    pub avatar: AvatarTheme,
    pub message: MessageTheme,
    pub select: SelectTheme,
    pub slider: SliderTheme,
    pub switch: SwitchTheme,
    pub upload: UploadTheme,
    pub nav_bar: NavBarTheme,
    pub tabbar: TabbarTheme,
    pub auto_complete: AutoCompleteTheme,
    pub color_picker: ColorPickerTheme,
    pub breadcrumb: BreadcrumbTheme,
    pub progress: ProgressTheme,
    pub typograph: TypographyTheme,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            name: "light".into(),
            common: CommonTheme::light(),
            button: ButtonTheme::light(),
            input: InputTheme::light(),
            menu: MenuTheme::light(),
            table: TableTheme::light(),
            alert: AlertTheme::light(),
            skeletion: SkeletionTheme::light(),
            tag: TagTheme::light(),
            avatar: AvatarTheme::light(),
            message: MessageTheme::light(),
            select: SelectTheme::light(),
            slider: SliderTheme::light(),
            switch: SwitchTheme::light(),
            upload: UploadTheme::light(),
            nav_bar: NavBarTheme::light(),
            tabbar: TabbarTheme::light(),
            auto_complete: AutoCompleteTheme::light(),
            color_picker: ColorPickerTheme::light(),
            breadcrumb: BreadcrumbTheme::light(),
            progress: ProgressTheme::light(),
            typograph: TypographyTheme::light(),
        }
    }
    pub fn dark() -> Self {
        Self {
            name: "dark".into(),
            common: CommonTheme::dark(),
            button: ButtonTheme::dark(),
            input: InputTheme::dark(),
            menu: MenuTheme::dark(),
            table: TableTheme::dark(),
            alert: AlertTheme::dark(),
            skeletion: SkeletionTheme::dark(),
            tag: TagTheme::dark(),
            avatar: AvatarTheme::dark(),
            message: MessageTheme::dark(),
            select: SelectTheme::dark(),
            slider: SliderTheme::dark(),
            switch: SwitchTheme::dark(),
            upload: UploadTheme::dark(),
            nav_bar: NavBarTheme::dark(),
            tabbar: TabbarTheme::dark(),
            auto_complete: AutoCompleteTheme::dark(),
            color_picker: ColorPickerTheme::dark(),
            breadcrumb: BreadcrumbTheme::dark(),
            progress: ProgressTheme::dark(),
            typograph: TypographyTheme::dark(),
        }
    }
}

impl ThemeMethod for Theme {
    fn light() -> Self {
        Theme::light()
    }
    fn dark() -> Self {
        Theme::dark()
    }
}

#[component]
pub fn ThemeProvider(
    #[prop(optional, into)] theme: Option<RwSignal<Theme>>,
    children: Children,
) -> impl IntoView {
    let theme = if let Some(theme) = theme {
        theme
    } else {
        create_rw_signal(Theme::light())
    };

    view! { <Provider value=theme children/> }
}

pub fn use_theme(default: impl Fn() -> Theme) -> ReadSignal<Theme> {
    use_context::<RwSignal<Theme>>()
        .unwrap_or_else(|| create_rw_signal(default()))
        .split()
        .0
}

pub fn use_rw_theme() -> RwSignal<Theme> {
    expect_context::<RwSignal<Theme>>()
}

#[cfg(test)]
mod tests {
    use super::{use_theme, Theme};

    fn _t_use_theme() {
        use_theme(Theme::dark);
    }
}

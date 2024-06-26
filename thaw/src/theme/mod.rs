mod common;

use self::common::CommonTheme;
use crate::{
    mobile::{NavBarTheme, TabbarTheme},
    AlertTheme, AnchorTheme, AutoCompleteTheme, AvatarTheme, BackTopTheme, BreadcrumbTheme,
    ButtonTheme, CalendarTheme, CollapseTheme, ColorPickerTheme, DatePickerTheme, DropdownTheme,
    InputTheme, MenuTheme, MessageTheme, PopoverTheme, ProgressTheme, ScrollbarTheme, SelectTheme,
    SkeletionTheme, SliderTheme, SpinnerTheme, SwitchTheme, TableTheme, TagTheme, TimePickerTheme,
    TypographyTheme, UploadTheme,
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
    pub spinner: SpinnerTheme,
    pub upload: UploadTheme,
    pub nav_bar: NavBarTheme,
    pub tabbar: TabbarTheme,
    pub auto_complete: AutoCompleteTheme,
    pub color_picker: ColorPickerTheme,
    pub breadcrumb: BreadcrumbTheme,
    pub progress: ProgressTheme,
    pub typograph: TypographyTheme,
    pub calendar: CalendarTheme,
    pub time_picker: TimePickerTheme,
    pub date_picker: DatePickerTheme,
    pub popover: PopoverTheme,
    pub dropdown: DropdownTheme,
    pub collapse: CollapseTheme,
    pub scrollbar: ScrollbarTheme,
    pub back_top: BackTopTheme,
    pub anchor: AnchorTheme,
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
            spinner: SpinnerTheme::light(),
            upload: UploadTheme::light(),
            nav_bar: NavBarTheme::light(),
            tabbar: TabbarTheme::light(),
            auto_complete: AutoCompleteTheme::light(),
            color_picker: ColorPickerTheme::light(),
            breadcrumb: BreadcrumbTheme::light(),
            progress: ProgressTheme::light(),
            typograph: TypographyTheme::light(),
            calendar: CalendarTheme::light(),
            time_picker: TimePickerTheme::light(),
            date_picker: DatePickerTheme::light(),
            popover: PopoverTheme::light(),
            dropdown: DropdownTheme::light(),
            collapse: CollapseTheme::light(),
            scrollbar: ScrollbarTheme::light(),
            back_top: BackTopTheme::light(),
            anchor: AnchorTheme::light(),
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
            spinner: SpinnerTheme::dark(),
            upload: UploadTheme::dark(),
            nav_bar: NavBarTheme::dark(),
            tabbar: TabbarTheme::dark(),
            auto_complete: AutoCompleteTheme::dark(),
            color_picker: ColorPickerTheme::dark(),
            breadcrumb: BreadcrumbTheme::dark(),
            progress: ProgressTheme::dark(),
            typograph: TypographyTheme::dark(),
            calendar: CalendarTheme::dark(),
            time_picker: TimePickerTheme::dark(),
            date_picker: DatePickerTheme::dark(),
            popover: PopoverTheme::dark(),
            dropdown: DropdownTheme::dark(),
            collapse: CollapseTheme::dark(),
            scrollbar: ScrollbarTheme::dark(),
            back_top: BackTopTheme::dark(),
            anchor: AnchorTheme::dark(),
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

mod theme;

use crate::{
    theme::use_theme,
    utils::{class_list::class_list, mount_style, OptionalProp},
    Theme,
};
use leptos::*;
pub use theme::TableTheme;

#[component]
pub fn Table(
    #[prop(optional, into)] style: MaybeSignal<String>,
    #[prop(default=true.into(), into)] single_row: MaybeSignal<bool>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] single_column: MaybeSignal<bool>,
    children: Children,
) -> impl IntoView {
    mount_style("table", include_str!("./table.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-background-color: {};",
                theme.table.background_color
            ));
            css_vars.push_str(&format!(
                "--thaw-background-color-striped: {};",
                theme.table.background_color_striped
            ));
            css_vars.push_str(&format!(
                "--thaw-border-color: {};",
                theme.table.border_color
            ));
            css_vars.push_str(&format!(
                "--thaw-border-radius: {};",
                theme.common.border_radius
            ));
        });

        css_vars
    });

    view! {
        <table
            class=class_list![
                "thaw-table", ("thaw-table--single-row", move || single_row.get()),
                ("thaw-table--single-column", move || single_column.get()), class.map(| c | move ||
                c.get())
            ]

            style=move || format!("{}{}", css_vars.get(), style.get())
        >
            {children()}
        </table>
    }
}

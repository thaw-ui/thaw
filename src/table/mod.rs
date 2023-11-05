mod theme;

use crate::{theme::use_theme, utils::mount_style::mount_style, Theme};
use leptos::*;
pub use theme::TableTheme;

#[component]
pub fn Table(
    #[prop(optional, into)] style: MaybeSignal<String>,
    #[prop(default=true.into(), into)] single_row: MaybeSignal<bool>,
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
            class="thaw-table"
            class=("thaw-table--single-row", move || single_row.get())
            class=("thaw-table--single-column", move || single_column.get())
            style=move || format!("{}{}", css_vars.get(), style.get())
        >
            {children()}
        </table>
    }
}

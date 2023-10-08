use crate::{theme::use_theme, utils::mount_style::mount_style, Theme};
use leptos::*;

#[component]
pub fn MenuGroup(label: &'static str, children: Children) -> impl IntoView {
    mount_style("menu-group", include_str!("./menu-group.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!("--font-color: {};", theme.menu.group_color));
        });
        css_vars
    });
    view! {
        <div class="melt-menu-group" style=move || css_vars.get()>
            {label}
        </div>
        {children()}
    }
}

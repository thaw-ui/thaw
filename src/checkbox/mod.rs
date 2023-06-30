use crate::{components::*, icon::*, theme::use_theme, utils::mount_style::mount_style, Theme};
use icondata::AiIcon;
use leptos::*;
use stylers::style_sheet_str;

#[component]
pub fn Checkbox(
    cx: Scope,
    #[prop(into)] checked: RwSignal<bool>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme(cx, Theme::light);
    let class_name = mount_style("checkbox", || {
        style_sheet_str!("./src/checkbox/checkbox.css")
    });

    let css_vars = create_memo(cx, move |_| {
        let mut css_vars = String::new();
        let theme = theme.get();
        let bg_color = theme.common.color_primary;
        css_vars.push_str(&format!("--background-color-checked: {bg_color};"));
        css_vars
    });

    view! {cx, class=class_name,
        <div class:melt-checkbox=true class=("melt-checkbox--checked", move || checked.get()) style=move || css_vars.get()
            on:click=move |_| checked.set(!checked.get_untracked())>
            <input class="melt-checkbox__input" type="checkbox" />
            <div class="melt-checkbox__dot">
                <If cond=checked>
                    <Then slot>
                        <Icon icon=Icon::from(AiIcon::AiCheckOutlined) style="color: white"/>
                    </Then>
                </If>
            </div>
            <div class="melt-checkbox__label">
                { children(cx) }
            </div>
        </div>
    }
}

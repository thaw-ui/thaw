mod checkbox_group;
mod checkbox_item;

use crate::{
    components::*,
    icon::*,
    theme::use_theme,
    utils::{maybe_rw_signal::MaybeRwSignal, mount_style::mount_style},
    Theme,
};
pub use checkbox_group::CheckboxGroup;
pub use checkbox_item::CheckboxItem;
use icondata::AiIcon;
use leptos::*;

#[component]
pub fn Checkbox(
    #[prop(optional, into)] checked: MaybeRwSignal<bool>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme(Theme::light);
    mount_style("checkbox", include_str!("./checkbox.css"));

    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        let theme = theme.get();
        let bg_color = theme.common.color_primary;
        css_vars.push_str(&format!("--background-color-checked: {bg_color};"));
        css_vars
    });

    view! {
        <div class:melt-checkbox=true class=("melt-checkbox--checked", move || checked.get()) style=move || css_vars.get()
            on:click=move |_| checked.set(!checked.get_untracked())>
            <input class="melt-checkbox__input" type="checkbox" />
            <div class="melt-checkbox__dot">
                <If cond=checked.clone_into()>
                    <Then slot>
                        <Icon icon=Icon::from(AiIcon::AiCheckOutlined) style="color: white"/>
                    </Then>
                </If>
            </div>
            <div class="melt-checkbox__label">
                { children() }
            </div>
        </div>
    }
}

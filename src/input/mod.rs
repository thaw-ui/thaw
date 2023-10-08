mod theme;

use crate::{
    theme::{use_theme, Theme},
    utils::mount_style::mount_style,
};
use leptos::*;
pub use theme::InputTheme;

#[derive(Default, Clone)]
pub enum InputType {
    #[default]
    TEXT,
    PASSWORD,
}

impl InputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputType::TEXT => "text",
            InputType::PASSWORD => "password",
        }
    }
}

#[component]
pub fn Input(
    #[prop(into)] value: RwSignal<String>,
    #[prop(optional, into)] type_: MaybeSignal<InputType>,
) -> impl IntoView {
    let theme = use_theme(Theme::light);
    mount_style("input", include_str!("./input.css"));

    let input_ref = create_node_ref::<html::Input>();
    input_ref.on_load(move |input| {
        input.on(ev::input, move |ev| {
            value.set(event_target_value(&ev));
        });
    });

    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        let theme = theme.get();
        let border_color_hover = theme.common.color_primary.clone();
        css_vars.push_str(&format!("--border-color-hover: {border_color_hover};"));
        let border_radius = theme.common.border_radius.clone();
        css_vars.push_str(&format!("--border-radius: {border_radius};"));
        css_vars
    });
    view! {
        <div class:melt-input=true style=move || css_vars.get()>
            <input
                type=move || type_.get().as_str()
                prop:value=move || value.get()
                ref=input_ref
                class="melt-input__input-el"
            />
        </div>
    }
}

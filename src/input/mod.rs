mod theme;
pub use theme::InputTheme;
use crate::{utils::mount_style::mount_style, theme::{use_theme, Theme}};
use leptos::*;
use stylers::style_sheet_str;

#[component]
pub fn Input(
    cx: Scope,
    #[prop(optional, into)] value: MaybeSignal<String>,
    #[prop(optional)] on_input: Option<SignalSetter<String>>,
) -> impl IntoView {
    let theme = use_theme(cx, Theme::light);
    let class_name = mount_style("input", || style_sheet_str!("./src/input/input.css"));
    let input_ref = create_node_ref::<html::Input>(cx);
    if let Some(on_input) = on_input {
        input_ref.on_load(cx, move |input| {
            input.on(ev::input, move |ev| {
                on_input.set(event_target_value(&ev));
            });
        });
    }
    let css_vars = create_memo(cx, move |_| {
        let mut css_vars = String::new();
        let theme = theme.get();
        let border_color_hover = theme.common.color_primary.clone();
        css_vars.push_str(&format!("--border-color-hover: {border_color_hover};"));
        let border_radius = theme.common.border_radius.clone();
        css_vars.push_str(&format!("--border-radius: {border_radius};"));
        css_vars
    });
    view! {
        cx, class=class_name,
        <div class:melt-input=true style=move || css_vars.get()>
            <input type="text" prop:value=move || value.get() ref=input_ref class="melt-input__input-el"/>
        </div>
    }
}

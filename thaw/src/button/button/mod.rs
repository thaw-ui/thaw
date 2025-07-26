mod types;

pub use types::*;

use crate::{Icon, Spinner};
use leptos::{
    either::{Either, EitherOf3},
    ev, html,
    prelude::*,
};
use thaw_utils::{class_list, BoxOneCallback, ComponentRef};

#[cfg(feature = "manganis")]
const _: manganis::Asset = manganis::asset!("/src/button/button/button.css");

/// A button triggers an action or event when activated.
#[component]
pub fn Button(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// A button can have its content and borders styled for greater emphasis or to be subtle.
    #[prop(optional, into)]
    appearance: Signal<ButtonAppearance>,
    /// A button can be rounded, circular, or square.
    #[prop(optional, into)]
    shape: Signal<ButtonShape>,
    /// A button supports different sizes.
    #[prop(optional, into)]
    size: Option<Signal<ButtonSize>>,
    /// The default behavior of the button.
    #[prop(optional, into)]
    button_type: MaybeProp<ButtonType>,
    /// Whether the button is displayed as block.
    #[prop(optional, into)]
    block: Signal<bool>,
    /// The icon of the button.
    #[prop(optional, into)]
    icon: MaybeProp<icondata_core::Icon>,
    /// Whether the button is disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// When set, allows the button to be focusable even when it has been disabled.
    #[prop(optional, into)]
    disabled_focusable: Signal<bool>,
    /// Whether the button shows the loading status.
    #[prop(optional, into)]
    loading: Signal<bool>,
    #[prop(optional, into)] on_click: Option<BoxOneCallback<ev::MouseEvent>>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] comp_ref: ComponentRef<ButtonRef>,
) -> impl IntoView {
    #[cfg(not(feature = "manganis"))]
    thaw_utils::mount_style("button", include_str!("./button.css"));

    let none_children = children.is_none();
    let size_injection = ButtonSizeInjection::use_context().map(|s| s.0);
    let size = size.unwrap_or_else(|| Signal::stored(size_injection.unwrap_or_default()));
    let only_icon = Memo::new(move |_| icon.with(|i| i.is_some()) && none_children);
    let btn_disabled = Memo::new(move |_| disabled.get() || disabled_focusable.get());
    let aria_disabled = move || {
        if loading.get() || disabled_focusable.get() {
            return Some("true");
        } else {
            return None;
        }
    };

    let button_ref = NodeRef::<html::Button>::new();
    comp_ref.load(ButtonRef { button_ref });

    let on_click = move |e| {
        if btn_disabled.get_untracked() {
            return;
        }
        if loading.get_untracked() {
            return;
        }

        let Some(on_click) = on_click.as_ref() else {
            return;
        };
        on_click(e);
    };

    view! {
        <button
            class=class_list![
                "thaw-button",
                ("thaw-button--disabled", btn_disabled),
                ("thaw-button--block", move || block.get()),
                ("thaw-button--only-icon", only_icon),
                ("thaw-button--icon", move || icon.with(|i| i.is_some())),
                ("thaw-button--loading", move || loading.get()),
                move || format!("thaw-button--{}", size.get().as_str()),
                move || format!("thaw-button--{}", appearance.get().as_str()),
                move || format!("thaw-button--{}", shape.get().as_str()),
                class
            ]
            type=move || button_type.get().map(|t| t.as_str())
            disabled=move || disabled.get().then_some("")
            aria-disabled=aria_disabled
            on:click=on_click
            node_ref=button_ref
        >
            {move || {
                if loading.get() {
                    EitherOf3::A(
                        view! {
                            <span class="thaw-button__icon">
                                <Spinner size=Signal::derive(move || size.get().into()) />
                            </span>
                        },
                    )
                } else if let Some(icon) = icon.get() {
                    EitherOf3::B(view! { <Icon icon=icon class="thaw-button__icon" /> })
                } else {
                    EitherOf3::C(())
                }
            }}
            {if let Some(children) = children {
                Either::Left(children())
            } else {
                Either::Right(())
            }}
        </button>
    }
}

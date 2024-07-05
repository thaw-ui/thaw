mod button_group;

pub use button_group::ButtonGroup;

use crate::icon::Icon;
use leptos::{ev, prelude::*};
use send_wrapper::SendWrapper;
use thaw_components::OptionComp;
use thaw_utils::{class_list, mount_style, OptionalMaybeSignal, OptionalProp};

#[derive(Default, PartialEq, Clone, Copy)]
pub enum ButtonAppearance {
    #[default]
    Secondary,
    Primary,
    Subtle,
    Transparent,
}

impl ButtonAppearance {
    fn as_str(&self) -> &str {
        match self {
            ButtonAppearance::Secondary => "secondary",
            ButtonAppearance::Primary => "primary",
            ButtonAppearance::Subtle => "subtle",
            ButtonAppearance::Transparent => "transparent",
        }
    }
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum ButtonShape {
    #[default]
    Rounded,
    Circular,
    Square,
}

impl ButtonShape {
    fn as_str(&self) -> &str {
        match self {
            ButtonShape::Rounded => "rounded",
            ButtonShape::Circular => "circular",
            ButtonShape::Square => "square",
        }
    }
}

#[derive(Default, PartialEq, Clone)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ButtonSize {
    fn as_str(&self) -> &str {
        match self {
            ButtonSize::Small => "small",
            ButtonSize::Medium => "medium",
            ButtonSize::Large => "large",
        }
    }
}

#[component]
pub fn Button(
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] appearance: MaybeSignal<ButtonAppearance>,
    #[prop(optional, into)] shape: MaybeSignal<ButtonShape>,
    #[prop(optional, into)] size: MaybeSignal<ButtonSize>,
    #[prop(optional, into)] block: MaybeSignal<bool>,
    #[prop(optional, into)] icon: OptionalMaybeSignal<icondata_core::Icon>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(optional, into)] disabled_focusable: MaybeSignal<bool>,
    #[prop(optional, into)] on_click: Option<Callback<SendWrapper<ev::MouseEvent>>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    mount_style("button", include_str!("./button.css"));

    let none_children = children.is_none();
    let only_icon = Memo::new(move |_| icon.with(|i| i.is_some()) && none_children);
    let btn_disabled = Memo::new(move |_| disabled.get() || disabled_focusable.get());

    let on_click = move |e| {
        if btn_disabled.get_untracked() {
            return;
        }

        let Some(callback) = on_click.as_ref() else {
            return;
        };
        callback.call(SendWrapper::new(e));
    };

    view! {
        <button
            class=class_list![
                "thaw-button",
                ("thaw-button--disabled", btn_disabled),
                ("thaw-button--block", move || block.get()),
                ("thaw-button--only-icon", only_icon),
                move || format!("thaw-button--{}", size.get().as_str()),
                move || format!("thaw-button--{}", appearance.get().as_str()),
                move || format!("thaw-button--{}", shape.get().as_str()),
                class.map(| c | move || c.get())
            ]

            disabled=move || disabled.get().then_some("")
            aria-disabled=move || disabled_focusable.get().then_some("true")
            on:click=on_click
        >
             {

                move || {
                    let icon = icon.get();
                    view! {
                        <OptionComp value=icon let:icon>
                            <Icon icon=icon class="thaw-button__icon"/>
                        </OptionComp>
                    }
                }
            }
            <OptionComp value=children let:children>
                {children()}
            </OptionComp>
        </button>
    }
}

mod button_group;

pub use button_group::ButtonGroup;

use crate::icon::Icon;
use leptos::{either::Either, ev, prelude::*};
use thaw_utils::{class_list, mount_style, BoxOneCallback};

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

//TODO loading prop

#[component]
pub fn Button(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// A button can have its content and borders styled for greater emphasis or to be subtle.
    #[prop(optional, into)]
    appearance: MaybeSignal<ButtonAppearance>,
    /// A button can be rounded, circular, or square.
    #[prop(optional, into)]
    shape: MaybeSignal<ButtonShape>,
    /// A button supports different sizes.
    #[prop(optional, into)]
    size: MaybeSignal<ButtonSize>,
    /// The default behavior of the button.
    #[prop(optional, into)]
    button_type: MaybeProp<ButtonType>,
    /// Whether the button is displayed as block.
    #[prop(optional, into)]
    block: MaybeSignal<bool>,
    /// The icon of the button.
    #[prop(optional, into)]
    icon: MaybeProp<icondata_core::Icon>,
    /// Whether the button is disabled.
    #[prop(optional, into)]
    disabled: MaybeSignal<bool>,
    /// When set, allows the button to be focusable even when it has been disabled.
    #[prop(optional, into)]
    disabled_focusable: MaybeSignal<bool>,
    #[prop(optional, into)] on_click: Option<BoxOneCallback<ev::MouseEvent>>,
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
                move || format!("thaw-button--{}", size.get().as_str()),
                move || format!("thaw-button--{}", appearance.get().as_str()),
                move || format!("thaw-button--{}", shape.get().as_str()),
                class
            ]
            type=move || button_type.get().map(|t| t.as_str())
            disabled=move || disabled.get().then_some("")
            aria-disabled=move || disabled_focusable.get().then_some("true")
            on:click=on_click
        >
            {move || {
                let icon = icon.get();
                if let Some(icon) = icon {
                    Either::Left(view! { <Icon icon=icon class="thaw-button__icon" /> })
                } else {
                    Either::Right(())
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

#[derive(Debug, Clone)]
pub enum ButtonType {
    Submit,
    Reset,
    Button,
}

impl ButtonType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Submit => "submit",
            Self::Reset => "reset",
            Self::Button => "button",
        }
    }
}

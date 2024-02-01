mod button_group;
mod theme;

use crate::{
    components::{OptionComp, Wave, WaveRef},
    icon::Icon,
    theme::*,
    utils::{class_list::class_list, mount_style, ComponentRef, OptionalProp},
};
pub use button_group::ButtonGroup;
use leptos::*;
pub use theme::ButtonTheme;

#[derive(Default, PartialEq, Clone, Copy)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Outlined,
    Text,
    Link,
}

#[derive(Default, Clone)]
pub enum ButtonColor {
    #[default]
    Primary,
    Success,
    Warning,
    Error,
}

impl ButtonColor {
    fn theme_color(&self, theme: &Theme) -> String {
        match self {
            ButtonColor::Primary => theme.common.color_primary.clone(),
            ButtonColor::Success => theme.common.color_success.clone(),
            ButtonColor::Warning => theme.common.color_warning.clone(),
            ButtonColor::Error => theme.common.color_error.clone(),
        }
    }
    fn theme_color_hover(&self, theme: &Theme) -> String {
        match self {
            ButtonColor::Primary => theme.common.color_primary_hover.clone(),
            ButtonColor::Success => theme.common.color_success_hover.clone(),
            ButtonColor::Warning => theme.common.color_warning_hover.clone(),
            ButtonColor::Error => theme.common.color_error_hover.clone(),
        }
    }

    fn theme_color_active(&self, theme: &Theme) -> String {
        match self {
            ButtonColor::Primary => theme.common.color_primary_active.clone(),
            ButtonColor::Success => theme.common.color_success_active.clone(),
            ButtonColor::Warning => theme.common.color_warning_active.clone(),
            ButtonColor::Error => theme.common.color_error_active.clone(),
        }
    }
}

#[derive(Default, Clone)]
pub enum ButtonSize {
    Tiny,
    Small,
    #[default]
    Medium,
    Large,
}

impl ButtonSize {
    fn theme_font_size(&self, theme: &Theme) -> String {
        match self {
            ButtonSize::Tiny => theme.common.font_size_tiny.clone(),
            ButtonSize::Small => theme.common.font_size_small.clone(),
            ButtonSize::Medium => theme.common.font_size_medium.clone(),
            ButtonSize::Large => theme.common.font_size_large.clone(),
        }
    }

    fn theme_height(&self, theme: &Theme) -> String {
        match self {
            ButtonSize::Tiny => theme.common.height_tiny.clone(),
            ButtonSize::Small => theme.common.height_small.clone(),
            ButtonSize::Medium => theme.common.height_medium.clone(),
            ButtonSize::Large => theme.common.height_large.clone(),
        }
    }

    fn theme_padding(&self, theme: &Theme) -> String {
        match self {
            ButtonSize::Tiny => theme.button.padding_tiny.clone(),
            ButtonSize::Small => theme.button.padding_small.clone(),
            ButtonSize::Medium => theme.button.padding_medium.clone(),
            ButtonSize::Large => theme.button.padding_large.clone(),
        }
    }
}

#[component]
pub fn Button(
    #[prop(optional, into)] style: Option<MaybeSignal<String>>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] variant: MaybeSignal<ButtonVariant>,
    #[prop(optional, into)] color: MaybeSignal<ButtonColor>,
    #[prop(optional, into)] size: MaybeSignal<ButtonSize>,
    #[prop(optional, into)] round: MaybeSignal<bool>,
    #[prop(optional, into)] circle: MaybeSignal<bool>,
    #[prop(optional, into)] icon: Option<icondata::Icon>,
    #[prop(optional, into)] loading: MaybeSignal<bool>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(optional, into)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            let bg_color = color.get().theme_color(theme);
            css_vars.push_str(&format!(
                "--thaw-font-color-disabled: {};",
                theme.button.color_text_disabled
            ));
            css_vars.push_str(&format!(
                "--thaw-font-size: {};",
                size.get().theme_font_size(theme)
            ));
            css_vars.push_str(&format!(
                "--thaw-height: {};",
                size.get().theme_height(theme)
            ));
            css_vars.push_str(&format!(
                "--thaw-padding: {};",
                size.get().theme_padding(theme)
            ));

            match variant.get() {
                ButtonVariant::Primary => {
                    let bg_color_hover = color.get().theme_color_hover(theme);
                    let bg_color_active = color.get().theme_color_active(theme);
                    css_vars.push_str(&format!("--thaw-background-color: {bg_color};"));
                    css_vars.push_str(&format!("--thaw-background-color-hover: {bg_color_hover};"));
                    css_vars.push_str(&format!(
                        "--thaw-background-color-active: {bg_color_active};"
                    ));
                    css_vars.push_str("--thaw-font-color: #fff;");
                    css_vars.push_str(&format!("--thaw-border-color: {bg_color};"));
                    css_vars.push_str(&format!("--thaw-border-color-hover: {bg_color};"));
                    css_vars.push_str(&format!("--thaw-ripple-color: {bg_color};"));

                    css_vars.push_str(&format!(
                        "--thaw-background-color-disabled: {};",
                        theme.button.color_background_disabled
                    ));
                    css_vars.push_str(&format!(
                        "--thaw-border-color-disabled: {};",
                        theme.button.color_border_disabled
                    ));
                }
                ButtonVariant::Outlined => {
                    css_vars.push_str(&format!("--thaw-font-color-hover: {bg_color};"));
                    css_vars.push_str(&format!(
                        "--thaw-border-color: {};",
                        theme.button.border_color_outlined
                    ));
                    css_vars.push_str(&format!("--thaw-border-color-hover: {bg_color};"));
                    css_vars.push_str(&format!("--thaw-ripple-color: {bg_color};"));
                    css_vars.push_str(&format!(
                        "--thaw-border-color-disabled: {};",
                        theme.button.color_border_disabled
                    ));
                }
                ButtonVariant::Text => {
                    css_vars.push_str(&format!("--thaw-font-color-hover: {bg_color};"));
                    css_vars.push_str(&format!(
                        "--thaw-background-color-hover: {};",
                        theme.button.color_text_hover
                    ));
                    css_vars.push_str(&format!(
                        "--thaw-background-color-active: {};",
                        theme.button.color_text_active
                    ));
                    css_vars.push_str("--thaw-ripple-color: #0000;");
                }
                ButtonVariant::Link => {
                    css_vars.push_str(&format!("--thaw-font-color-hover: {bg_color};"));
                    css_vars.push_str("--thaw-ripple-color: #0000;");
                }
            }
        });

        css_vars
    });
    mount_style("button", include_str!("./button.css"));

    let icon_style = if children.is_some() {
        "margin-right: 6px"
    } else {
        ""
    };

    let disabled = create_memo(move |_| {
        if loading.get() {
            return true;
        }

        disabled.get()
    });

    let wave_ref = ComponentRef::<WaveRef>::default();

    let on_click = move |event| {
        if disabled.get() {
            return;
        }
        if let Some(wave_ref) = wave_ref.get_untracked() {
            wave_ref.play();
        }
        let Some(callback) = on_click.as_ref() else {
            return;
        };
        callback.call(event);
    };

    view! {
        <button
            class=class_list![
                "thaw-button", ("thaw-button--outlined", move || variant.get() ==
                ButtonVariant::Outlined), ("thaw-button--text", move || variant.get() ==
                ButtonVariant::Text), ("thaw-button--link", move || variant.get() ==
                ButtonVariant::Link), ("thaw-button--round", move || round.get()),
                ("thaw-button--circle", move || circle.get()), ("thaw-button--disabled", move ||
                disabled.get()), class.map(| c | move || c.get())
            ]

            style=move || {
                format!("{}{}", css_vars.get(), style.as_ref().map(|s| s.get()).unwrap_or_default())
            }
            disabled=move || disabled.get()
            on:click=on_click
        >
            <Wave comp_ref=wave_ref/>
            {move || {
                if loading.get() {
                    view! {
                        <Icon
                            icon=icondata::AiLoadingOutlined
                            style=format!(
                                "animation: thawLoadingCircle 1s infinite linear;{icon_style}",
                            )
                        />
                    }
                        .into()
                } else if let Some(icon) = icon {
                    view! { <Icon icon=icon style=icon_style/> }.into()
                } else {
                    None
                }
            }}

            <OptionComp value=children let:children>
                {children()}
            </OptionComp>
        </button>
    }
}

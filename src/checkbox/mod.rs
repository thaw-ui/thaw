use crate::{theme::use_theme, utils::mount_style::mount_style, Theme};
use leptos::*;
use stylers::style_sheet_str;
use leptos_icons::*;

#[component]
pub fn Checkbox(
    cx: Scope,
    #[prop(optional, into)] checked: MaybeSignal<bool>,
    #[prop(optional, into)] on_checked: Option<SignalSetter<bool>>,
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
    let on_click = move |_| {
        if let Some(on_checked) = on_checked {
            on_checked.set(!checked.get());
        }
    };
    view! {cx, class=class_name,
        <div class:melt-checkbox=true class=("melt-checkbox--checked", move || checked.get()) style=move || css_vars.get() on:click=on_click>
            <input class="melt-checkbox__input" type="checkbox" />
            <div class="melt-checkbox__dot">
                {
                    move || {
                        if checked.get() {
                            view! {cx,
                                <LeptosIcon icon=AiIcon::AiCheckOutlined style="color: white"/>
                            }.into()
                        } else {
                            None
                        }
                    }
                }
            </div>
            <div class="melt-checkbox__label">
                {children(cx)}
            </div>
        </div>
    }
}

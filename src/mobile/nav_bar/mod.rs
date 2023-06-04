use crate::utils::mount_style::mount_style;
use leptos::*;
use leptos_icons::*;
use stylers::style_sheet_str;

#[component]
pub fn NavBar(
    cx: Scope,
    #[prop(optional, into)] title: MaybeSignal<String>,
    #[prop(optional, into)] left_arrow: MaybeSignal<bool>,
    #[prop(optional, into)] left_text: MaybeSignal<String>,
    #[prop(optional, into)] right_text: MaybeSignal<String>,
) -> impl IntoView {
    let class_name = mount_style("nav-bar", || style_sheet_str!("./src/mobile/nav_bar/nav-bar.css"));

    view! { cx, class=class_name,
        <div class="melt-nav-bar">
            {
                move || {
                    let left_text = left_text.get();
                    if left_arrow.get() || !left_text.is_empty() {
                        view! { cx, class=class_name,
                            <div class="melt-nav-bar__left">
                                {
                                    if left_arrow.get() {
                                        view! { cx,
                                            <Icon icon=AiIcon::AiLeftOutlined/>
                                        }.into()
                                    } else {
                                        None
                                    }
                                }
                                { left_text }
                            </div>
                        }.into()
                    } else {
                        None
                    }
                }
            }
            <div class="melt-nav-bar__center">
                { move || title.get() }
            </div>
            {
                move || {
                    let right_text = right_text.get();
                    if !right_text.is_empty() {
                        view! { cx, class=class_name,
                            <div class="melt-nav-bar__right">
                                { right_text }
                            </div>
                        }.into()
                    } else {
                        None
                    }
                }
            }
            
        </div>
    }
}
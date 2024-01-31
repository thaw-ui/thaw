mod theme;

use crate::{
    components::*,
    icon::*,
    use_theme,
    utils::{class_list::class_list, mount_style, OptionalProp},
    Theme,
};
use leptos::*;
pub use theme::NavBarTheme;

#[slot]
pub struct NavBarLeft {
    #[prop(optional, into)]
    class: OptionalProp<MaybeSignal<String>>,
    children: Children,
}

#[slot]
pub struct NavBarRight {
    #[prop(optional, into)]
    class: OptionalProp<MaybeSignal<String>>,
    children: Children,
}

#[component]
pub fn NavBar(
    #[prop(optional, into)] title: MaybeSignal<String>,
    #[prop(optional, into)] left_arrow: MaybeSignal<bool>,
    #[prop(optional, into)] left_text: OptionalProp<MaybeSignal<String>>,
    #[prop(optional)] nav_bar_left: Option<NavBarLeft>,
    #[prop(optional, into)] on_click_left: Option<Callback<ev::MouseEvent>>,
    #[prop(optional, into)] right_text: OptionalProp<MaybeSignal<String>>,
    #[prop(optional)] nav_bar_right: Option<NavBarRight>,
    #[prop(optional, into)] on_click_right: Option<Callback<ev::MouseEvent>>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
) -> impl IntoView {
    mount_style("nav-bar", include_str!("./nav-bar.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        theme.with(|theme| {
            format!(
                "--thaw-background-color: {};",
                theme.nav_bar.background_color
            )
        })
    });

    let on_click_left = move |ev| {
        if let Some(click_left) = on_click_left.as_ref() {
            click_left.call(ev);
        }
    };

    let on_click_right = move |ev| {
        if let Some(click_right) = on_click_right.as_ref() {
            click_right.call(ev);
        }
    };

    view! {
        <div class=class_list!["thaw-nav-bar", class.map(|c| move || c.get())] style=move || css_vars.get()>
            {
                if let Some(NavBarLeft { class, children }) = nav_bar_left {
                    view! {
                        <div class=class_list!["thaw-nav-bar__left", class.map(|c| move || c.get())] on:click=on_click_left>
                            {children()}
                        </div>
                    }.into_view()
                } else if let Some(left_text) = left_text.into_option() {
                    view! {
                        <div class="thaw-nav-bar__left" on:click=on_click_left>
                            <If cond=left_arrow>
                                <Then slot>
                                    <Icon icon=icondata::AiLeftOutlined/>
                                </Then>
                            </If>
                            {move || left_text.get()}
                        </div>
                    }.into_view()
                } else {
                    (move || {
                        if left_arrow.get() {
                            view! {
                                <div class="thaw-nav-bar__left" on:click=on_click_left>
                                    <Icon icon=icondata::AiLeftOutlined/>
                                </div>
                            }.into()
                        } else {
                            None
                        }
                    }).into_view()
                }
            }
            <div class="thaw-nav-bar__center">{move || title.get()}</div>
            {
                if let Some(NavBarRight { class, children }) = nav_bar_right {
                    view! {
                        <div class=class_list!["thaw-nav-bar__right", class.map(|c| move || c.get())] on:click=on_click_right>
                            {children()}
                        </div>
                    }.into()
                } else if let Some(right_text) = right_text.into_option() {
                    view! {
                        <div class="thaw-nav-bar__right" on:click=on_click_right>
                            {move || right_text.get()}
                        </div>
                    }.into()
                } else {
                    None
                }
            }
        </div>
    }
}

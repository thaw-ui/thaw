mod theme;

pub use theme::BackTopTheme;

use crate::{use_theme, Icon, Theme};
use leptos::{html::ToHtmlElement, *};
use thaw_components::{CSSTransition, Teleport};
use thaw_utils::{add_event_listener, get_scroll_parent, mount_style};

#[component]
pub fn BackTop(
    #[prop(default=40.into(), into)] right: MaybeSignal<i32>,
    #[prop(default=40.into(), into)] bottom: MaybeSignal<i32>,
    #[prop(default=180.into(), into)] visibility_height: MaybeSignal<i32>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    mount_style("back-top", include_str!("./back-top.css"));
    let theme = use_theme(Theme::light);
    let style = Memo::new(move |_| {
        let mut style = String::new();
        style.push_str(&format!("right: {}px;", right.get_untracked()));
        style.push_str(&format!("bottom: {}px;", bottom.get_untracked()));
        theme.with(|theme| {
            style.push_str(&format!(
                "--thaw-icon-color-hover: {};",
                theme.common.color_primary_hover
            ));
            style.push_str(&format!(
                "--thaw-icon-color-active: {};",
                theme.common.color_primary_active
            ));
            style.push_str(&format!(
                "--thaw-background-color: {};",
                theme.back_top.background_color
            ));
        });
        style
    });
    let placeholder_ref = NodeRef::<html::Div>::new();
    let back_top_ref = NodeRef::new();
    let is_show_back_top = RwSignal::new(false);
    let scroll_top = RwSignal::new(0);

    let _ = watch(
        move || scroll_top.get(),
        move |scroll_top, _, _| {
            is_show_back_top.set(scroll_top > &visibility_height.get());
        },
        false,
    );

    let scroll_to_top = StoredValue::new(None::<Callback<()>>);

    placeholder_ref.on_load(move |el| {
        let scroll_el = get_scroll_parent(&el.into_any())
            .unwrap_or_else(|| document().document_element().unwrap().to_leptos_element());

        {
            let scroll_el = scroll_el.clone();
            scroll_to_top.set_value(Some(Callback::new(move |_| {
                scroll_el.scroll_by_with_scroll_to_options(
                    web_sys::ScrollToOptions::new()
                        .top(0.0)
                        .behavior(web_sys::ScrollBehavior::Smooth),
                );
            })));
        }

        let handle = add_event_listener(scroll_el.clone(), ev::scroll, move |_| {
            scroll_top.set(scroll_el.scroll_top());
        });

        on_cleanup(move || {
            handle.remove();
        });
    });

    let on_click = move |_| {
        scroll_to_top.with_value(|scroll_to_top| {
            if let Some(scroll_to_top) = scroll_to_top {
                scroll_to_top.call(());
            }
        });
    };

    view! {
        <div style="display: none" class="thaw-back-top-placeholder" ref=placeholder_ref>
            <Teleport immediate=is_show_back_top>
                <CSSTransition
                    node_ref=back_top_ref
                    name="fade-in-scale-up-transition"
                    appear=is_show_back_top.get_untracked()
                    show=is_show_back_top
                    let:display
                >
                    <div class="thaw-back-top" ref=back_top_ref style=move || display.get().map(|d| d.to_string()).unwrap_or_else(|| style.get()) on:click=on_click>
                        <Icon icon=icondata_ai::AiVerticalAlignTopOutlined/>
                    </div>
                </CSSTransition>
            </Teleport>
        </div>
    }
}

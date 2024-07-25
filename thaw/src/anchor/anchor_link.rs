use super::AnchorInjection;
use leptos::{html, prelude::*};
use thaw_components::OptionComp;
use thaw_utils::{class_list, StoredMaybeSignal};

#[component]
pub fn AnchorLink(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(into)] title: MaybeSignal<String>,
    #[prop(into)] href: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let anchor = AnchorInjection::expect_context();

    let title: StoredMaybeSignal<_> = title.into();
    let title_ref = NodeRef::<html::A>::new();
    let href_id = StoredValue::new(None::<String>);
    let is_active = Memo::new(move |_| {
        href_id.with_value(|href_id| {
            if href_id.is_none() {
                false
            } else {
                anchor.active_id.with(|active_id| active_id == href_id)
            }
        })
    });

    if !href.is_empty() {
        if href.starts_with('#') {
            let id = href[1..].to_string();
            href_id.set_value(Some(id.clone()));
            anchor.append_id(id);

            on_cleanup(move || {
                href_id.with_value(|id| {
                    if let Some(id) = id {
                        anchor.remove_id(id);
                    }
                });
            });

            Effect::new(move |_| {
                let Some(title_el) = title_ref.get() else {
                    return;
                };

                if is_active.get() {
                    let title_rect = title_el.get_bounding_client_rect();
                    anchor.update_background_position(title_rect);
                }
            });
        }
    }
    let on_click = move |_| {
        href_id.with_value(move |href_id| {
            if let Some(href_id) = href_id {
                anchor.scroll_into_view(href_id);
            }
        });
    };

    view! {
        <div class=class_list![
            "thaw-anchor-link", ("thaw-anchor-link--active", move || is_active.get()), class]>
            <a
                href=href
                class="thaw-anchor-link__title"
                on:click=on_click
                node_ref=title_ref
                title=move || title.get()
            >
                {move || title.get()}
            </a>
            <OptionComp value=children let:children>
                {children()}
            </OptionComp>
        </div>
    }
}

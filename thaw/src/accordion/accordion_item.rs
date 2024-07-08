use crate::AccordionInjection;
use leptos::{prelude::*, html};
use thaw_components::CSSTransition;
use thaw_utils::{mount_style, update, with, StoredMaybeSignal};

#[component]
pub fn AccordionItem(
    #[prop(into)] value: MaybeSignal<String>,
    accordion_header: AccordionHeader,
    children: Children,
) -> impl IntoView {
    mount_style("accordion-item", include_str!("./accordion-item.css"));
    let AccordionInjection {
        open_items,
        multiple,
        collapsible,
    } = AccordionInjection::use_();
    let panel_ref = NodeRef::<html::Div>::new();
    let value: StoredMaybeSignal<_> = value.into();

    let is_show_panel = Memo::new(move |_| with!(|open_items, value| open_items.contains(value)));

    let on_click = move |_| {
        let is_show_panel = is_show_panel.get_untracked();
        update!(move |open_items| {
            if is_show_panel {
                if collapsible {
                    with!(|value| open_items.remove(value));
                } else if multiple {
                    with!(|value| open_items.remove(value));
                }
            } else {
                if !multiple {
                    open_items.clear();
                }
                open_items.insert(value.get_untracked());
            }
        });
    };

    view! {
        <div class="thaw-accordion-item">
            <div class="thaw-accordion-header">
                <button
                    class="thaw-accordion-header__button"
                    // aria_expanded=move || is_show_panel.get().to_string() #TODO
                    type="button"
                    on:click=on_click
                >
                    {(accordion_header.children)()}
                </button>
            </div>
            <CSSTransition
                node_ref=panel_ref
                show=is_show_panel
                name="thaw-accordion-panel"
                let:display
            >
                <div
                    class="thaw-accordion-panel"
                    node_ref=panel_ref
                    style=move || display.get().unwrap_or_default()
                >
                    {children()}
                </div>
            </CSSTransition>
        </div>
    }
}

#[slot]
pub struct AccordionHeader {
    children: Children,
}

#[slot]
pub struct AccordionPanel {
    children: Children,
}

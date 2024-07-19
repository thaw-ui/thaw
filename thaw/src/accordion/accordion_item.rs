use crate::AccordionInjection;
use leptos::{html, prelude::*};
use thaw_components::CSSTransition;
use thaw_utils::{mount_style, update, with, StoredMaybeSignal};

#[component]
pub fn AccordionItem(
    /// Required value that identifies this item inside an Accordion component.
    #[prop(into)]
    value: MaybeSignal<String>,
    accordion_header: AccordionHeader,
    children: Children,
) -> impl IntoView {
    mount_style("accordion-item", include_str!("./accordion-item.css"));
    let AccordionInjection {
        open_items,
        multiple,
        collapsible,
    } = AccordionInjection::expect_context();
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
                    aria-expanded=move || is_show_panel.get().to_string()
                    type="button"
                    on:click=on_click
                >
                    <span
                        class="thaw-accordion-header__expand-icon"
                        aria-hidden="true"
                    >
                        <svg
                            fill="currentColor"
                            aria-hidden="true"
                            width="1em"
                            height="1em"
                            viewBox="0 0 20 20"
                            style=move || if is_show_panel.get() {
                                "transform: rotate(90deg)"
                            } else {
                                "transform: rotate(0deg)"
                            }
                        >
                            <path d="M7.65 4.15c.2-.2.5-.2.7 0l5.49 5.46c.21.22.21.57 0 .78l-5.49 5.46a.5.5 0 0 1-.7-.7L12.8 10 7.65 4.85a.5.5 0 0 1 0-.7Z" fill="currentColor"></path>
                        </svg>
                    </span>
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

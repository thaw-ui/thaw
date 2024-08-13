use super::{DrawerPosition, DrawerSize};
use crate::ConfigInjection;
use leptos::{ev, html, prelude::*};
use thaw_components::{CSSTransition, FocusTrap, Teleport};
use thaw_utils::{class_list, mount_style, use_lock_html_scroll, Model};

#[component]
pub fn OverlayDrawer(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Controls the open state of the Drawer.
    #[prop(into)]
    open: Model<bool>,
    /// Whether to emit hide event when click mask.
    #[prop(default = true.into(), into)]
    mask_closeable: MaybeSignal<bool>,
    /// Whether to close drawer on Esc is pressed.
    #[prop(optional, into)]
    close_on_esc: bool,
    /// Position of the drawer.
    #[prop(optional, into)]
    position: MaybeSignal<DrawerPosition>,
    /// Size of the drawer.
    #[prop(optional, into)]
    size: MaybeSignal<DrawerSize>,
    children: Children,
) -> impl IntoView {
    mount_style("drawer", include_str!("./drawer.css"));
    mount_style("overlay-drawer", include_str!("./overlay-drawer.css"));

    let config_provider = ConfigInjection::expect_context();
    let drawer_ref = NodeRef::<html::Div>::new();
    let open_drawer: RwSignal<bool> = RwSignal::new(open.get_untracked());
    let is_lock = RwSignal::new(open.get_untracked());
    Effect::new(move |_| {
        let is_open = open.get();
        if is_open {
            is_lock.set(true);
        }
        open_drawer.set(is_open);
    });
    use_lock_html_scroll(is_lock.into());
    let on_after_leave = move || {
        is_lock.set(false);
    };

    let mask_ref = NodeRef::<html::Div>::new();
    let on_mask_click = move |_| {
        if mask_closeable.get_untracked() {
            open.set(false);
        }
    };
    let on_esc = move |_: ev::KeyboardEvent| {
        open.set(false);
    };

    view! {
        <Teleport immediate=open.signal()>
            <FocusTrap disabled=!close_on_esc active=open.signal() on_esc>
                <div
                    class=class_list!["thaw-config-provider thaw-overlay-drawer-container", class]
                    data-thaw-id=config_provider.id().clone()
                >
                    <CSSTransition
                        node_ref=mask_ref
                        appear=open.get_untracked()
                        show=open.signal()
                        name="fade-in-transition"
                        let:display
                    >
                        <div
                            class="thaw-overlay-drawer__backdrop"
                            style=move || display.get().unwrap_or_default()
                            on:click=on_mask_click
                            node_ref=mask_ref
                        ></div>
                    </CSSTransition>
                    <CSSTransition
                        node_ref=drawer_ref
                        appear=open_drawer.get_untracked()
                        show=open_drawer
                        name=Memo::new(move |_| {
                            format!("slide-in-from-{}-transition", position.get().as_str())
                        })

                        on_after_leave
                        let:display
                    >
                        <div
                            class=class_list![
                                "thaw-overlay-drawer",
                                move || format!("thaw-overlay-drawer--position-{}", position.get().as_str())
                            ]

                            style=move || {
                                let size = move || {
                                    format!(
                                        "--thaw-drawer--size: {}",
                                        size.get().as_size_str(position),
                                    )
                                };
                                display.get().map_or_else(size, |d| d.to_string())
                            }
                            node_ref=drawer_ref
                            role="dialog"
                            aria-modal="true"
                        >
                            {children()}
                        </div>
                    </CSSTransition>
                </div>
            </FocusTrap>
        </Teleport>
    }
}

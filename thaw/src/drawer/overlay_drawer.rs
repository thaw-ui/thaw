use crate::ConfigInjection;
use leptos::*;
use thaw_components::{CSSTransition, FocusTrap, Teleport};
use thaw_utils::{class_list, mount_style, use_lock_html_scroll, Model};

#[component]
fn OverlayDrawer(
    open: Model<bool>,
    mask_closeable: MaybeSignal<bool>,
    close_on_esc: bool,
    // placement: MaybeSignal<DrawerPlacement>,
    children: Children,
) -> impl IntoView {
    mount_style("overlay-drawer", include_str!("./overlay-drawer.css"));
    let config_provider = ConfigInjection::use_();
    let drawer_ref = NodeRef::<html::Div>::new();

    let placement = Memo::new(move |prev| {
        // let placement:  = placement.get().as_str();
        // let Some(prev) = prev else {
        //     return placement;
        // };

        // if is_css_transition.get() {
        //     prev
        // } else {
        //     placement
        // }
        "left"
    });
    let is_css_transition = RwSignal::new(false);
    let on_after_enter = move |_| {
        is_css_transition.set(false);
    };

    let is_lock = RwSignal::new(open.get_untracked());
    Effect::new(move |_| {
        let is_show = open.get();
        if is_show {
            is_lock.set(true);
            is_css_transition.set(true);
        }
    });
    use_lock_html_scroll(is_lock.into());
    let on_after_leave = move |_| {
        is_lock.set(false);
        is_css_transition.set(false);
    };

    let mask_ref = NodeRef::<html::Div>::new();
    let on_mask_click = move |_| {
        if mask_closeable.get_untracked() {
            open.set(false);
        }
    };
    let on_esc = Callback::new(move |_: ev::KeyboardEvent| {
        open.set(false);
    });

    view! {
        <Teleport immediate=open.signal()>
            <FocusTrap disabled=!close_on_esc active=open.signal() on_esc>
                <div class="thaw-config-provider thaw-overlay-drawer-container" data-thaw-id=config_provider.id().clone()>
                    <CSSTransition
                        node_ref=mask_ref
                        appear=open.get_untracked()
                        show=open.signal()
                        name="fade-in-transition"
                        let:display
                    >
                        <div
                            class="thaw-overlay-drawer__backdrop"
                            style=move || display.get()
                            on:click=on_mask_click
                            ref=mask_ref
                        ></div>
                    </CSSTransition>
                    <CSSTransition
                        node_ref=drawer_ref
                        appear=open.get_untracked()
                        show=open.signal()
                        name=Memo::new(move |_| {
                            format!("slide-in-from-{}-transition", placement.get())
                        })

                        on_after_enter
                        on_after_leave
                        let:display
                    >
                        <div
                            class=class_list![
                                "thaw-overlay-drawer", move || format!("thaw-drawer--placement-{}",
                                placement.get())
                            ]

                            style=move || display.get()
                            ref=drawer_ref
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

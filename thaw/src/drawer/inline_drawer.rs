use leptos::*;
use thaw_components::CSSTransition;
use thaw_utils::{class_list, mount_style, Model};

#[component]
fn InlineDrawer(
    open: Model<bool>,
    mask_closeable: MaybeSignal<bool>,
    close_on_esc: bool,
    // placement: MaybeSignal<DrawerPlacement>,
    children: Children,
) -> impl IntoView {
    mount_style("overlay-drawer", include_str!("./overlay-drawer.css"));
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

    view! {
        <CSSTransition
            node_ref=drawer_ref
            appear=open.get_untracked()
            show=open.signal()
            name=Memo::new(move |_| {
                format!("slide-in-from-{}-transition", placement.get())
            })

            let:display
        >
            <div
                class=class_list![
                    "thaw-overlay-drawer", move || format!("thaw-drawer--placement-{}",
                    placement.get())
                ]
                ref=drawer_ref
            >
                {children()}
            </div>
        </CSSTransition>
    }
}

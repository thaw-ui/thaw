use super::{DrawerPosition, DrawerSize};
use leptos::{html, prelude::*};
use thaw_components::CSSTransition;
use thaw_utils::{class_list, mount_style, Model};

#[component]
pub fn InlineDrawer(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Controls the open state of the Drawer.
    #[prop(into)]
    open: Model<bool>,
    /// Position of the drawer.
    #[prop(optional, into)]
    position: Signal<DrawerPosition>,
    /// Size of the drawer.
    #[prop(optional, into)]
    size: Signal<DrawerSize>,
    children: Children,
) -> impl IntoView {
    mount_style("drawer", include_str!("./drawer.css"));
    mount_style("inline-drawer", include_str!("./inline-drawer.css"));
    let drawer_ref = NodeRef::<html::Div>::new();
    let open_drawer: RwSignal<bool> = RwSignal::new(open.get_untracked());

    Effect::new(move |_| {
        let is_open = open.get();
        open_drawer.set(is_open);
    });

    view! {
        <CSSTransition
            node_ref=drawer_ref
            appear=open_drawer.get_untracked()
            show=open_drawer
            name=Memo::new(move |_| {
                format!("slide-in-from-{}-transition", position.get().as_str())
            })
            let:display
        >
            <div
                class=class_list![
                    "thaw-inline-drawer",
                    move || format!("thaw-inline-drawer--position-{}", position.get().as_str()),
                    class
                ]
                style=move || {
                    let size = move || {
                        format!("--thaw-drawer--size: {}", size.get().as_size_str(position))
                    };
                    display.get().map_or_else(size, |d| d.to_string())
                }
                node_ref=drawer_ref
            >
                {children()}
            </div>
        </CSSTransition>
    }
}

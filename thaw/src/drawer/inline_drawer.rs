use super::{DrawerPosition, DrawerSize};
use leptos::*;
use thaw_components::CSSTransition;
use thaw_utils::{class_list, mount_style, Model};

#[component]
pub fn InlineDrawer(
    #[prop(into)] open: Model<bool>,
    #[prop(optional, into)] position: MaybeSignal<DrawerPosition>,
    #[prop(optional, into)] size: MaybeSignal<DrawerSize>,
    children: Children,
) -> impl IntoView {
    mount_style("drawer", include_str!("./drawer.css"));
    mount_style("inline-drawer", include_str!("./inline-drawer.css"));
    let drawer_ref = NodeRef::<html::Div>::new();
    let is_css_transition = RwSignal::new(false);
    let on_after_enter = move |_| {
        is_css_transition.set(false);
    };
    let lazy_position = Memo::new(move |prev| {
        let position = position.get().as_str();
        let Some(prev) = prev else {
            return position;
        };

        if is_css_transition.get() {
            prev
        } else {
            position
        }
    });
    Effect::new(move |_| {
        let is_open = open.get();
        if is_open {
            is_css_transition.set(true);
        }
    });
    let on_after_leave = move |_| {
        is_css_transition.set(false);
    };

    view! {
        <CSSTransition
            node_ref=drawer_ref
            appear=open.get_untracked()
            show=open.signal()
            name=Memo::new(move |_| {
                format!("slide-in-from-{}-transition", lazy_position.get())
            })
            on_after_enter
            on_after_leave
            let:display
        >
            <div
                class=class_list![
                    "thaw-inline-drawer",
                    move || format!("thaw-inline-drawer--position-{}", lazy_position.get())
                ]
                style=move || {
                    let size = move || {format!("--thaw-drawer--size: {}", size.get().as_size_str(position))};
                    display
                        .get()
                        .map_or_else(size, |d| d.to_string())
                }
                ref=drawer_ref
            >
                {children()}
            </div>
        </CSSTransition>
    }
}

use leptos::{html::ElementDescriptor, *};
use std::ops::Deref;

#[component]
pub fn CSSTransition<T: ElementDescriptor + Clone + 'static>(
    node_ref: NodeRef<T>,
    #[prop(into)] show: MaybeSignal<bool>,
    #[prop(into)] name: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let display = store_value(None::<String>);
    let remove_class_name = store_value(None::<RemoveClassName>);

    node_ref.on_load(move |node_el| {
        let el = node_el.clone().into_any();
        let el = el.deref();
        let style = el.style();

        if !show.get_untracked() {
            let display_value = style.get_property_value("display").ok();
            display.set_value(display_value);
            let _ = style.set_property("display", "none");
        }

        let class_list = el.class_list();
        let remove_class = Callback::new(move |_| {
            remove_class_name.update_value(|class| {
                if let Some(class) = class.take() {
                    match class {
                        RemoveClassName::Enter(active, to) => {
                            let _ = class_list.remove_2(&active, &to);
                        }
                        RemoveClassName::Leave(active, to) => {
                            let _ = class_list.remove_2(&active, &to);
                            let _ = style.set_property("display", "none");
                        }
                    }
                }
            });
        });

        let _ = node_el
            .on(ev::transitionend, move |_| {
                remove_class.call(());
            })
            .on(ev::animationend, move |_| {
                remove_class.call(());
            });
    });

    create_render_effect(move |prev: Option<bool>| {
        let show = show.get();
        if let Some(node_el) = node_ref.get_untracked() {
            if let Some(prev) = prev {
                let name = name.get_untracked();

                let el = node_el.into_any();
                let el = el.deref();
                let style = el.style();
                let class_list = el.class_list();

                if show && !prev {
                    let enter_from = format!("{name}-enter-from");
                    let enter_active = format!("{name}-enter-active");
                    let enter_to = format!("{name}-enter-to");

                    let _ = class_list.add_2(&enter_from, &enter_active);
                    display.update_value(|display| {
                        if let Some(display) = display.take() {
                            let _ = style.set_property("display", &display);
                        }
                    });
                    request_animation_frame(move || {
                        let _ = class_list.remove_1(&enter_from);
                        let _ = class_list.add_1(&enter_to);
                        remove_class_name
                            .set_value(Some(RemoveClassName::Enter(enter_active, enter_to)));
                    });
                } else if !show && prev {
                    let display_value = style.get_property_value("display").ok();
                    display.set_value(display_value);

                    let leave_from = format!("{name}-leave-from");
                    let leave_active = format!("{name}-leave-active");
                    let leave_to = format!("{name}-leave-to");

                    let _ = class_list.add_2(&leave_from, &leave_active);
                    request_animation_frame(move || {
                        let _ = class_list.remove_1(&leave_from);
                        let _ = class_list.add_1(&leave_to);
                        remove_class_name
                            .set_value(Some(RemoveClassName::Leave(leave_active, leave_to)));
                    });
                }
            }
        }
        show
    });

    children()
}

enum RemoveClassName {
    Enter(String, String),
    Leave(String, String),
}

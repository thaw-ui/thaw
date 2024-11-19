use leptos::{either::Either, ev, html, prelude::*};
use thaw_utils::{class_list, mount_style};

#[component]
pub fn Table(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    mount_style("table", include_str!("./table.css"));

    view! { <table class=class_list!["thaw-table", class]>{children()}</table> }
}

#[component]
pub fn TableHeader(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! { <thead class=class_list!["thaw-table-header", class]>{children()}</thead> }
}

#[component]
pub fn TableHeaderCell(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] resizable: bool,
    #[prop(optional, into)] min_width: Option<f64>,
    #[prop(optional, into)] max_width: Option<f64>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let th_ref = NodeRef::<html::Th>::new();
    let mouse_down_x = RwSignal::new(0);
    let mouse_down_th_width = RwSignal::new(0.0);
    let mouse = StoredValue::new(Vec::<WindowListenerHandle>::new());

    let on_mouse_down = move |e: ev::MouseEvent| {
        mouse_down_x.set(e.x());
        let Some(th_el) = th_ref.get_untracked() else {
            return;
        };
        let Ok(Some(css)) = window().get_computed_style(&th_el) else{
           return;
        };
        if let Ok(width) = css.get_property_value("width") {
            let width = web_sys::js_sys::Number::parse_float(&width);
            mouse_down_th_width.set(width);
        }

        let on_mouse_move = window_event_listener(ev::mousemove, move |e: ev::MouseEvent| {
            let mouse_x = e.x();
            let mut new_width = mouse_down_th_width.get_untracked()
                + f64::from(mouse_x - mouse_down_x.get_untracked());

            if let Some(max_width) = max_width {
                if new_width > max_width {
                    new_width = max_width;
                }
            }
            if let Some(min_width) = min_width {
                if new_width < min_width {
                    new_width = min_width;
                }
            }
            if new_width < 0.0 {
                new_width = 0.0;
            }

            if let Some(th_el) = th_ref.get_untracked() {
                let mut style = format!("width: {new_width:.2}px");
                if let Some(max_width) = max_width {
                    style.push_str(&format!(";max-width: {max_width:.2}px"));
                }
                if let Some(min_width) = min_width {
                    style.push_str(&format!(";min-width: {min_width:.2}px"));
                }
                let _ = th_el.set_attribute("style", &style);
            }
        });
        let on_mouse_up = window_event_listener(ev::mouseup, move |_| {
            mouse.update_value(|value| {
                for handle in value.drain(..) {
                    handle.remove();
                }
            });
        });
        mouse.update_value(|value| {
            value.push(on_mouse_move);
            value.push(on_mouse_up);
        });
    };
    on_cleanup(move || {
        mouse.update_value(|value| {
            for handle in value.drain(..) {
                handle.remove();
            }
        });
    });
    view! {
        <th class=class_list!["thaw-table-header-cell", class] node_ref=th_ref>
            <button class="thaw-table-header-cell__button" role="presentation">
                {if let Some(children) = children {
                    Either::Left(children())
                } else {
                    Either::Right(())
                }}
            </button>
            {if resizable {
                Either::Left(view! {
                    <span class="thaw-table-header-cell__aside" on:mousedown=on_mouse_down>
                        <div class="thaw-table-resize-handle" role="separator" aria-hidden="true">
                        </div>
                    </span>
                })
            } else {
                Either::Right(())
            }}
        </th>
    }
}

#[component]
pub fn TableBody(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! { <tbody class=class_list!["thaw-table-body", class]>{children()}</tbody> }
}

#[component]
pub fn TableRow(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! { <tr class=class_list!["thaw-table-row", class]>{children()}</tr> }
}

#[component]
pub fn TableCell(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <td class=class_list![
            "thaw-table-cell", class
        ]>
            {if let Some(children) = children {
                Either::Left(children())
            } else {
                Either::Right(())
            }}
        </td>
    }
}

#[component]
pub fn TableCellLayout(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] truncate: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=class_list![
            "thaw-table-cell-layout",
            ("thaw-table-cell-layout--truncate", move || truncate.get()),
            class
        ]>
            <div class="thaw-table-cell-layout__content">
                <span class="thaw-table-cell-layout__main">{children()}</span>
            </div>
        </div>
    }
}

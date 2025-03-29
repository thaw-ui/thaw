use leptos::prelude::*;
use leptos_transition_group::CSSTransition;
use thaw_utils::mount_style;
use web_sys::HtmlElement;

#[component]
pub fn CollapseTransition<T>(
    #[prop(into)] show: Signal<bool>,
    children: TypedChildren<T>,
) -> impl IntoView
where
    T: AddAnyAttr + IntoView + Send + 'static,
{
    mount_style("motion-collapse", include_str!("./index.css"));

    let on_enter = |el: HtmlElement| {
        let style = el.style();
        let memorized_height = el.offset_height();
        let _ = style.set_property("max-height", "0");
        el.offset_width();
        let _ = style.set_property("transition", "");
        let _ = el
            .style()
            .set_property("max-height", &format!("{}px", memorized_height));
        el.offset_width();
    };

    let on_after_enter = |el: HtmlElement| {
        let _ = el.style().set_property("max-height", "");
    };

    let on_before_leave = |el: HtmlElement| {
        let _ = el
            .style()
            .set_property("max-height", &format!("{}px", el.offset_height()));
        el.offset_width();
    };

    let on_leave = |el: HtmlElement| {
        let _ = el.style().set_property("max-height", "0");
        el.offset_width();
    };

    let on_after_leave = |el: HtmlElement| {
        let _ = el.style().set_property("max-height", "");
    };

    view! {
        <CSSTransition
            show
            name="thaw-motion-collapse"
            children
            on_enter
            on_after_enter
            on_before_leave
            on_leave
            on_after_leave
        />
    }
}

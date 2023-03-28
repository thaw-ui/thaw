use leptos::*;
use stylers::style_sheet;

#[component]
pub fn Table(cx: Scope, children: Children) -> impl IntoView {
    let class_name = style_sheet!("./src/table/table.css");
    view! {cx, class=class_name,
        <table class="melt-table">
            {children(cx)}
        </table>
    }
}
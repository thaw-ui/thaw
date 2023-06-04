use leptos::*;
use melt_ui::mobile::NavBar;

#[component]
pub fn NavBarPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div style="height: 100vh; background: #f5f5f5">
            <NavBar title="Home" left_arrow=true left_text="back" right_text="add"/>
        </div>
    }
}
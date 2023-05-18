use leptos::*;
use melt_ui::*;

#[component]
pub fn MenuPage(cx: Scope) -> impl IntoView {
    let selected = create_rw_signal(cx, String::from("o"));
    view! { cx,
        <Menu selected>
            <MenuItem key="a" label="and"/>
            <MenuItem key="o" label="or"/>
        </Menu>
    }
}

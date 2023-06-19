use leptos::*;
use melt_ui::*;

#[component]
pub fn MenuPage(cx: Scope) -> impl IntoView {
    let selected = create_rw_signal(cx, String::from("o"));
    view! { cx,
        <div>
            { move || selected.get() }
            <Menu selected>
                <MenuItem key="a" label="and"/>
                <MenuItem key="o" label="or"/>
            </Menu>
        </div>
    }
}

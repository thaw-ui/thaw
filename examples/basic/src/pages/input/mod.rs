use leptos::*;
use melt_ui::*;

#[component]
pub fn InputPage(cx: Scope) -> impl IntoView {
    let value = create_rw_signal(cx, String::from("o"));
    view! { cx,
        <>
            {move || value.get()}
            <Input value/>
            <Input value type_="password" />
        </>
    }
}
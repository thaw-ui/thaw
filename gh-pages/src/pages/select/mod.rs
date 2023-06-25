use leptos::*;
use melt_ui::*;

#[component]
pub fn SelectPage(cx: Scope) -> impl IntoView {
    let selected_value = create_rw_signal(cx, Some(String::from("apple")));

    let options = vec![SelectOption {
        label: String::from("apple"),
        value: String::from("apple"),
    }];
    view! { cx,
        <Select value=selected_value options/>
    }
}

use leptos::*;
use melt_ui::*;

#[component]
pub fn SelectPage() -> impl IntoView {
    let selected_value = create_rw_signal(Some(String::from("apple")));

    let options = vec![SelectOption {
        label: String::from("apple"),
        value: String::from("apple"),
    }];
    view! {
        <Select value=selected_value options/>
    }
}

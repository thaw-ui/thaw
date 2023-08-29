use leptos::*;
use melt_ui::*;

#[component]
pub fn InputPage() -> impl IntoView {
    let value = create_rw_signal(String::from("o"));
    view! {
        <>
            {move || value.get()}
            <Input value/>
            <Input value type_=InputType::PASSWORD />
        </>
    }
}

use leptos::*;
use melt_ui::*;

#[component]
pub fn CheckboxPage() -> impl IntoView {
    let checked = create_rw_signal(false);
    view! {
        <div>
            <Checkbox checked>
                "Click"
            </Checkbox>
        </div>
    }
}

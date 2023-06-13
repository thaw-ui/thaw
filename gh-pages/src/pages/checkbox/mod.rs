use leptos::*;
use melt_ui::*;

#[component]
pub fn CheckboxPage(cx: Scope) -> impl IntoView {
    let checked = create_rw_signal(cx, false);
    view! {cx,
        <div>
            <Checkbox checked>
                "Click"
            </Checkbox>
        </div>
    }
}

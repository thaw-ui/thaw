use leptos::*;
use melt_ui::*;

#[component]
pub fn TabsPage(cx: Scope) -> impl IntoView {
    let active_key = create_rw_signal(cx, "apple");
    view! { cx,
        <Tabs active_key>
            <Tab key="apple" label="Apple">
                "apple"
            </Tab>
            <Tab key="pear" label="Pear">
                "pear"
            </Tab>
        </Tabs>
    }
}

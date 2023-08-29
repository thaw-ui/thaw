use leptos::*;
use melt_ui::*;

#[component]
pub fn TabsPage() -> impl IntoView {
    let active_key = create_rw_signal("apple");
    view! {
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

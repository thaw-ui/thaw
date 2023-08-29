use leptos::*;
use melt_ui::mobile::*;

#[component]
pub fn TabbarPage() -> impl IntoView {
    let selected = create_rw_signal(String::from("o"));
    view! {
        <div style="height: 100vh; background: #f5f5f5">
            { move || selected.get() }
            <Tabbar selected>
                <TabbarItem name="a">
                    "and"
                </TabbarItem>
                <TabbarItem name="i">
                    "if"
                </TabbarItem>
                <TabbarItem name="o" icon=icondata::AiIcon::AiCloseOutlined>
                    "or"
                </TabbarItem>
            </Tabbar>
        </div>
    }
}

use leptos::*;
use melt_ui::mobile::*;

#[component]
pub fn TabbarPage(cx: Scope) -> impl IntoView {
    let selected = create_rw_signal(cx, String::from("o"));
    view! { cx,
        <div style="height: 100vh; background: #f5f5f5">
            { move || selected.get() }
            <Tabbar selected>
                <TabbarItem name="a">
                    "and"
                </TabbarItem>
                <TabbarItem name="i">
                    "if"
                </TabbarItem> 
                <TabbarItem name="o" icon=leptos_icons::AiIcon::AiCloseOutlined>
                    "or"
                </TabbarItem> 
            </Tabbar>
        </div>
    }
}

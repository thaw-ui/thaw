use crate::components::{Demo, DemoCode};
use leptos::*;
use melt_ui::*;
use prisms::highlight_str;

#[component]
pub fn MenuPage() -> impl IntoView {
    let value = create_rw_signal(String::from("o"));
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Menu"</h1>
            <Demo>
                <Menu value>
                    <MenuItem key="a" label="and"/>
                    <MenuItem key="o" label="or"/>
                </Menu>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    let value = create_rw_signal(String::from("o"));
                            
                    <Menu value>
                        <MenuItem key="a" label="and"/>
                        <MenuItem key="o" label="or"/>
                    </Menu>
                "#,
                        "rust"
                    )
                >

                    ""
                </DemoCode>
            </Demo>
        </div>
    }
}

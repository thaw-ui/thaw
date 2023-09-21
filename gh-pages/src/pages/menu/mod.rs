use crate::components::{Demo, DemoCode};
use indoc::indoc;
use leptos::*;
use melt_ui::*;

#[component]
pub fn MenuPage() -> impl IntoView {
    let selected = create_rw_signal(String::from("o"));
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Menu"</h1>
            <Demo>
                <Menu selected>
                    <MenuItem key="a" label="and"/>
                    <MenuItem key="o" label="or"/>
                </Menu>
                <DemoCode slot>
                    {
                        indoc!(r#"
                        let selected = create_rw_signal(String::from("o"));
                        
                        <Menu selected>
                            <MenuItem key="a" label="and"/>
                            <MenuItem key="o" label="or"/>
                        </Menu>
                        "#)
                    }
                </DemoCode>
            </Demo>
        </div>
    }
}

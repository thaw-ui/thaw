use crate::components::{Demo, DemoCode};
use indoc::indoc;
use leptos::*;
use melt_ui::*;

#[component]
pub fn InputPage() -> impl IntoView {
    let value = create_rw_signal(String::from("o"));
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Input"</h1>
            <Demo>
                <Input value/>
                <Input value type_=InputType::PASSWORD />
                <DemoCode slot>
                    {
                        indoc! {r#"
                            let value = create_rw_signal(String::from("o"));

                            <Input value/>
                            <Input value type_=InputType::PASSWORD />
                        "#}
                    }
                </DemoCode>
            </Demo>
        </div>
    }
}

use crate::components::{Demo, DemoCode};
use indoc::indoc;
use leptos::*;
use melt_ui::*;

#[component]
pub fn SelectPage() -> impl IntoView {
    let selected_value = create_rw_signal(Some(String::from("apple")));

    let options = vec![SelectOption {
        label: String::from("apple"),
        value: String::from("apple"),
    }];
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Select"</h1>
            <Demo>
                <Select value=selected_value options/>
                <DemoCode slot>
                    {
                        indoc! {r#"
                        let selected_value = create_rw_signal(Some(String::from("apple")));
                        let options = vec![SelectOption {
                            label: String::from("apple"),
                            value: String::from("apple"),
                        }];

                        <Select value=selected_value options/>
                        "#}
                    }
                </DemoCode>
            </Demo>
        </div>
    }
}

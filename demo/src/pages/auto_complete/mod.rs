use crate::components::{Demo, DemoCode};
use leptos::*;
use thaw::*;
use prisms::highlight_str;

#[component]
pub fn AutoCompletePage() -> impl IntoView {
    let value = create_rw_signal(String::new());
    let options = create_memo(move |_| {
        let prefix = value
            .get()
            .split_once('@')
            .map_or(value.get(), |v| v.0.to_string());
        vec!["@gmail.com", "@163.com"]
            .into_iter()
            .map(|suffix| AutoCompleteOption {
                label: format!("{prefix}{suffix}"),
                value: format!("{prefix}{suffix}"),
            })
            .collect()
    });

    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Auto Complete"</h1>
            <Demo>
                <AutoComplete value options placeholder="Email"/>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                        let value = create_rw_signal(String::new());
                        let options =create_memo(|_| {
                            let prefix = value
                                .get()
                                .split_once('@')
                                .map_or(value.get(), |v| v.0.to_string());
                            vec!["@gmail.com", "@163.com"]
                                .into_iter()
                                .map(|suffix| AutoCompleteOption {
                                    label: format!("{prefix}{suffix}"),
                                    value: format!("{prefix}{suffix}"),
                                })
                                .collect()
                        });
                        view! {
                            <AutoComplete value options placeholder="Email"/>
                        }
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

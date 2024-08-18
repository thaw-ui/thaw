use leptos::{context::Provider, prelude::*};
use thaw_components::OptionComp;
use thaw_utils::{class_list, mount_style};
use uuid::Uuid;

#[component]
pub fn Field(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] orientation: MaybeSignal<FieldOrientation>,
    children: Children,
) -> impl IntoView {
    mount_style("field", include_str!("./field.css"));
    let id = StoredValue::new(Uuid::new_v4().to_string());

    view! {
        <div class=class_list![
            "thaw-field",
            move || {
                format!("thaw-field--{}", orientation.get().as_str())
            },
            class
        ]>
            {move || {
                view! {
                    <OptionComp value=label.get() let:label>
                        <label class="thaw-field__label" for=id.get_value()>
                            {label}
                        </label>
                    </OptionComp>
                }
            }}
            <Provider value=FieldInjection { id, name }>{children()}</Provider>
        </div>
    }
}

#[derive(Clone)]
pub(crate) struct FieldInjection {
    id: StoredValue<String>,
    name: MaybeProp<String>,
}

impl FieldInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }
}

#[derive(Debug, Default, Clone)]
pub enum FieldOrientation {
    Horizontal,
    #[default]
    Vertical,
}

impl FieldOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

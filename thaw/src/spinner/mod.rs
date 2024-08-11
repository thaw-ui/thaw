use leptos::prelude::*;
use thaw_utils::{class_list, mount_style};

#[derive(Default, Clone)]
pub enum SpinnerSize {
    ExtraTiny,
    Tiny,
    ExtraSmall,
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
    Huge,
}

impl SpinnerSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            SpinnerSize::ExtraTiny => "extra-tiny",
            SpinnerSize::Tiny => "tiny",
            SpinnerSize::ExtraSmall => "extra-small",
            SpinnerSize::Small => "small",
            SpinnerSize::Medium => "medium",
            SpinnerSize::Large => "large",
            SpinnerSize::ExtraLarge => "extra-large",
            SpinnerSize::Huge => "huge",
        }
    }
}

#[component]
pub fn Spinner(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// An optional label for the Spinner.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// The size of the spinner.
    #[prop(optional, into)]
    size: MaybeSignal<SpinnerSize>,
) -> impl IntoView {
    mount_style("spinner", include_str!("./spinner.css"));
    let id = StoredValue::new(uuid::Uuid::new_v4().to_string());

    let spinner_label = label.clone();
    let labelledby = move || {
        spinner_label.with(|label| {
            if label.is_some() {
                Some(id.get_value())
            } else {
                None
            }
        })
    };

    view! {
        <div
            class=class_list!["thaw-spinner", move || format!("thaw-spinner--{}", size.get().as_str()), class]
            role="progressbar"
            aria-labelledby=labelledby
        >
            <span class="thaw-spinner__spinner">
                <span class="thaw-spinner__spinner-tail"></span>
            </span>
            {
                move || {
                    if let Some(label) = label.get() {
                        view! {
                            <label class="thaw-spinner__label" id=id.get_value()>
                                {label}
                            </label>
                        }.into()
                    } else {
                        None
                    }
                }
            }
        </div>
    }
}

use leptos::prelude::*;
use thaw_utils::class_list;

#[cfg(feature = "manganis")]
const _: manganis::Asset = manganis::asset!("/src/spinner/spinner.css");

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
    size: Signal<SpinnerSize>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    #[cfg(not(feature = "manganis"))]
    thaw_utils::mount_style("spinner", include_str!("./spinner.css"));

    let id = StoredValue::new(uuid::Uuid::new_v4().to_string());

    let spinner_label = label.clone();
    let children_flag = children.is_some();
    let labelledby = move || {
        spinner_label.with(|label| {
            if label.is_some() || children_flag {
                Some(id.get_value())
            } else {
                None
            }
        })
    };

    view! {
        <div
            class=class_list![
                "thaw-spinner",
                move || format!("thaw-spinner--{}", size.get().as_str()),
                class
            ]
            role="progressbar"
            aria-labelledby=labelledby
        >
            <span class="thaw-spinner__spinner">
                <span class="thaw-spinner__spinner-tail"></span>
            </span>
            {if let Some(children) = children {
                view! {
                    <label class="thaw-spinner__label" id=id.get_value()>
                        {children()}
                    </label>
                }
                    .into_any()
            } else {
                {
                    move || {
                        if let Some(label) = label.get() {
                            view! {
                                <label class="thaw-spinner__label" id=id.get_value()>
                                    {label}
                                </label>
                            }
                                .into()
                        } else {
                            None
                        }
                    }
                }
                    .into_any()
            }}
        </div>
    }
}

use leptos::{context::Provider, either::EitherOf3, prelude::*};
use thaw_components::OptionComp;
use thaw_utils::{class_list, mount_style};
use uuid::Uuid;

#[component]
pub fn Field(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// The label associated with the field.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    /// The orientation of the label relative to the field component.
    #[prop(optional, into)]
    orientation: MaybeSignal<FieldOrientation>,
    children: Children,
) -> impl IntoView {
    mount_style("field", include_str!("./field.css"));
    let id = StoredValue::new(Uuid::new_v4().to_string());
    let validation_state = RwSignal::new(None::<FieldValidationState>);

    view! {
        <div class=class_list![
            "thaw-field",
            move || {
                format!("thaw-field--{}", orientation.get().as_str())
            },
            class
        ]>
            {
                let label = label.clone();
                move || {
                    view! {
                        <OptionComp value=label.get() let:label>
                            <label class="thaw-field__label" for=id.get_value()>
                                {label}
                            </label>
                        </OptionComp>
                    }
                }
            }
            <Provider value=FieldInjection { id, name, label, validation_state }>{children()}</Provider>
            {move || {
                view! {
                    <OptionComp value=validation_state.get() let:validation_state>
                        {
                            match validation_state {
                                FieldValidationState::Error(message) => EitherOf3::A(view! {
                                    <div class="thaw-field__validation-message">
                                        <span class="thaw-field__validation-message-icon thaw-field__validation-message-icon--error">
                                            <svg
                                                fill="currentColor"
                                                aria-hidden="true"
                                                width="12"
                                                height="12"
                                                viewBox="0 0 12 12">
                                                <path
                                                    d="M6 11A5 5 0 1 0 6 1a5 5 0 0 0 0 10Zm-.75-2.75a.75.75 0 1 1 1.5 0 .75.75 0 0 1-1.5 0Zm.26-4.84a.5.5 0 0 1 .98 0l.01.09v2.59a.5.5 0 0 1-1 0V3.41Z"
                                                    fill="currentColor">
                                                </path>
                                            </svg>
                                        </span>
                                        {message}
                                    </div>
                                }),
                                FieldValidationState::Success(message) => EitherOf3::B(view! {
                                    <div class="thaw-field__validation-message">
                                        <span class="thaw-field__validation-message-icon thaw-field__validation-message-icon--success">
                                            <svg
                                                fill="currentColor"
                                                aria-hidden="true"
                                                width="12"
                                                height="12"
                                                viewBox="0 0 12 12">
                                                <path
                                                    d="M1 6a5 5 0 1 1 10 0A5 5 0 0 1 1 6Zm7.35-.9a.5.5 0 1 0-.7-.7L5.5 6.54 4.35 5.4a.5.5 0 1 0-.7.7l1.5 1.5c.2.2.5.2.7 0l2.5-2.5Z"
                                                    fill="currentColor">
                                                </path>
                                            </svg>
                                        </span>
                                        {message}
                                    </div>
                                }),
                                FieldValidationState::Warning(message) => EitherOf3::C(view! {
                                    <div class="thaw-field__validation-message">
                                        <span class="thaw-field__validation-message-icon thaw-field__validation-message-icon--warning">
                                            <svg
                                                fill="currentColor"
                                                aria-hidden="true"
                                                width="12"
                                                height="12"
                                                viewBox="0 0 12 12">
                                                <path
                                                    d="M5.21 1.46a.9.9 0 0 1 1.58 0l4.09 7.17a.92.92 0 0 1-.79 1.37H1.91a.92.92 0 0 1-.79-1.37l4.1-7.17ZM5.5 4.5v1a.5.5 0 0 0 1 0v-1a.5.5 0 0 0-1 0ZM6 6.75a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5Z"
                                                    fill="currentColor">
                                                </path>
                                            </svg>
                                        </span>
                                        {message}
                                    </div>
                                })
                            }
                        }

                    </OptionComp>
                }
            }}
        </div>
    }
}

#[derive(Clone)]
pub(crate) struct FieldInjection {
    id: StoredValue<String>,
    name: MaybeProp<String>,
    label: MaybeProp<String>,
    validation_state: RwSignal<Option<FieldValidationState>>,
}

impl FieldInjection {
    pub fn use_context() -> Option<Self> {
        use_context()
    }

    pub fn id(&self) -> Option<String> {
        if self.label.with(|l| l.is_some()) {
            Some(self.id.get_value())
        } else {
            None
        }
    }

    pub fn name(&self) -> Option<String> {
        self.name.get()
    }

    pub fn use_id_and_name(
        id: MaybeProp<String>,
        name: MaybeProp<String>,
    ) -> (Signal<Option<String>>, Signal<Option<String>>) {
        let field_injection = Self::use_context();
        let id = Signal::derive(move || {
            if let Some(id) = id.get() {
                return Some(id);
            }

            let Some(field_injection) = field_injection.as_ref() else {
                return None;
            };

            field_injection.id()
        });

        let field_injection = Self::use_context();
        let name = Signal::derive(move || {
            if let Some(name) = name.get() {
                return Some(name);
            }

            let Some(field_injection) = field_injection.as_ref() else {
                return None;
            };

            field_injection.name()
        });

        (id, name)
    }

    pub fn update_validation_state(&self, state: Option<FieldValidationState>) {
        self.validation_state.try_maybe_update(|validation_state| {
            if validation_state == &state {
                (false, ())
            } else {
                *validation_state = state;
                (true, ())
            }
        });
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

#[derive(Debug, Clone, PartialEq)]
pub enum FieldValidationState {
    Error(String),
    Success(String),
    Warning(String),
}

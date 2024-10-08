use leptos::{context::Provider, prelude::*};
use slotmap::{DefaultKey, SlotMap};

#[component]
pub fn FieldContextProvider(children: Children) -> impl IntoView {
    view! { <Provider value=FieldContextInjection::new()>{children()}</Provider> }
}

#[derive(Clone)]
pub struct FieldContextInjection(
    StoredValue<SlotMap<DefaultKey, (Signal<Option<String>>, Callback<(), bool>)>>,
);

impl FieldContextInjection {
    fn new() -> Self {
        Self(StoredValue::new(SlotMap::new()))
    }

    pub fn expect_context() -> Self {
        expect_context()
    }

    pub fn use_context() -> Option<Self> {
        use_context()
    }

    pub(crate) fn register_field(
        &self,
        name: Signal<Option<String>>,
        validate: impl Fn() -> bool + Send + Sync + 'static,
    ) {
        let mut key = None;
        let validate: Callback<(), bool> = Callback::from(move || validate());
        self.0.update_value(|map| {
            key = Some(map.insert((name, validate)));
            ()
        });

        let map = self.0.clone();
        Owner::on_cleanup(move || {
            map.update_value(|map| { map.remove(key.unwrap()); });
        });
    }

    pub fn validate(&self) -> bool {
        self.0.with_value(|map| {
            let mut rt = true;
            for (_, (_, validate)) in map.iter() {
                if !validate.run(()) {
                    rt = false;
                }
            }
            rt
        })
    }

    pub fn validate_field(&self, name: String) -> bool {
        self.0.with_value(|map| {
            for (_, (n, validate)) in map.iter() {
                if n.get_untracked().as_ref() == Some(&name) && !validate.run(()) {
                    return false;
                }
            }
            true
        })
    }
}

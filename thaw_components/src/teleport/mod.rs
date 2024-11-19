use leptos::prelude::*;
use std::cell::Cell;

#[component]
pub fn Teleport(
    #[prop(default = true.into(), into)] immediate: Signal<bool>,
    #[prop(into, optional)] mount: Option<web_sys::Element>,
    children: Children,
) -> impl IntoView {
    let mount_fn: Cell<Option<Box<dyn FnOnce() -> ()>>> = Cell::new(Some(Box::new(move || {
        let mount = if let Some(el) = mount.as_ref() {
            el
        } else {
            use leptos::wasm_bindgen::JsCast;
            &document()
                .body()
                .expect("body element to exist")
                .unchecked_into()
        };

        let mountable = {
            let view = children().into_view();
            let mut mountable = view.build();
            mountable.mount(mount, None);
            mountable
        };

        on_cleanup({
            let mut mountable = send_wrapper::SendWrapper::new(mountable);
            move || {
                mountable.unmount();
            }
        });
    })));

    let owner = Owner::new();
    Effect::new(move |_| {
        if immediate.get() {
            let Some(f) = mount_fn.take() else {
                return;
            };

            owner.with(|| {
                f();
            });
        }
    });
}

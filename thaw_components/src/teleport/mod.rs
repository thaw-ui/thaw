use std::cell::Cell;

use cfg_if::cfg_if;
use leptos::prelude::*;

/// https://github.com/solidjs/solid/blob/main/packages/solid/web/src/index.ts#L56
#[component]
pub fn Teleport(
    #[prop(default = true.into(), into)] immediate: MaybeSignal<bool>,
    #[prop(into, optional)] mount: Option<web_sys::Element>,
    children: Children,
) -> impl IntoView {
    // cfg_if! { if #[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))] {
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

    let owner = Owner::current().unwrap();
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
    // } else {
    //     let _ = mount;
    //     let _ = immediate;
    //     let _ = element;
    //     let _ = children;
    // }}
}

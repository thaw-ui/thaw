use cfg_if::cfg_if;
use leptos::{children, prelude::*};
use tachys::view::any_view::AnyView;

/// https://github.com/solidjs/solid/blob/main/packages/solid/web/src/index.ts#L56
#[component]
pub fn Teleport(
    #[prop(default = true.into(), into)] immediate: MaybeSignal<bool>,
    #[prop(into, optional)] mount: Option<web_sys::Element>,
    #[prop(optional, into)] element: Option<AnyView<Dom>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // cfg_if! { if #[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))] {
    let mount_fn = StoredValue::new(None::<Box<dyn FnOnce() + Send + Sync>>);
    let mount = send_wrapper::SendWrapper::new(mount);
    let element = send_wrapper::SendWrapper::new(element);
    let children = send_wrapper::SendWrapper::new(children);
    mount_fn.set_value(Some(Box::new(move || {
        let mount = if let Some(el) = mount.take() {
            el
        } else {
            use leptos::wasm_bindgen::JsCast;
            document()
                .body()
                .expect("body element to exist")
                .unchecked_into()
        };

        if let Some(element) = element.take() {
            let render_root = element;
            let mut mountable = render_root.build();
            mountable.mount(&mount, None);
            // TODO
            // on_cleanup(move || {
            //     mountable.unmount();
            // });
        } else if let Some(children) = children.take() {
            let container = document()
                .create_element("div")
                .expect("element creation to work");

            thaw_utils::with_hydration_off(|| {
                // use leptos::leptos_dom::Mountable;
                // let _ = container.append_child(&children().into_view().get_mountable_node());
                let view = children().into_view();
                let mut mountable = view.build();
                mountable.mount(&container, None);
            });

            let render_root = container;
            let _ = mount.append_child(&render_root);
            // on_cleanup(move || {
            //     let _ = mount.remove_child(&render_root);
            // });
        }
    })));

    let owner = Owner::current().unwrap();
    Effect::new(move |_| {
        if immediate.get() {
            let Some(f) = mount_fn
                .try_update_value(|mount_fn| mount_fn.take())
                .flatten()
            else {
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

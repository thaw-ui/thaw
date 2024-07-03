use super::{ToastPosition, ToasterReceiver};
use leptos::*;
use thaw_components::Teleport;
use thaw_utils::{class_list, mount_style};

#[component]
pub fn Toaster(
    receiver: ToasterReceiver,
    #[prop(optional)] position: ToastPosition,
) -> impl IntoView {
    mount_style("toaster", include_str!("./toaster.css"));
    let toast_list = RwSignal::new(vec![]);

    Effect::new(move |_| {
        for view in receiver.try_recv() {
            toast_list.update(move |list| {
                list.push(view);
            });
        }
    });
    view! {
        <Teleport>
            <div class="thaw-config-provider thaw-toaster-container">
                <For
                    each=move || toast_list.get()
                    key=|toast| toast.1.id
                    let:toast
                >
                    <div class=class_list!["thaw-toaster", "thaw-toaster"]>
                        {toast.0}
                    </div>
                </For>
            </div>
        </Teleport>
    }
}

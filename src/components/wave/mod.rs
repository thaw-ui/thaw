use crate::utils::{mount_style, ComponentRef};
use leptos::{leptos_dom::helpers::TimeoutHandle, *};
use std::time::Duration;

#[derive(Clone)]
pub struct WaveRef {
    play: Callback<()>,
}

impl WaveRef {
    pub fn play(&self) {
        self.play.call(());
    }
}

#[component]
pub fn Wave(#[prop(optional)] comp_ref: ComponentRef<WaveRef>) -> impl IntoView {
    mount_style("wave", include_str!("./wave.css"));
    let wave_ref = create_node_ref::<html::Div>();
    let animation_timeout_handle = create_rw_signal(None::<TimeoutHandle>);
    let play = Callback::new(move |_: ()| {
        if let Some(handle) = animation_timeout_handle.get() {
            handle.clear();
            animation_timeout_handle.set(None);
        }
        if let Some(wave_ref) = wave_ref.get() {
            _ = wave_ref.offset_height();
        }
        let handle = set_timeout_with_handle(
            move || {
                animation_timeout_handle.set(None);
            },
            Duration::from_secs(1),
        );
        if let Ok(handle) = handle {
            animation_timeout_handle.set(Some(handle))
        }
    });
    comp_ref.load(WaveRef { play });
    on_cleanup(move || {
        if let Some(handle) = animation_timeout_handle.get() {
            handle.clear();
            animation_timeout_handle.set(None);
        }
    });
    view! {
        <div
            class="thaw-wave"
            class=(
                "thaw-wave--active",
                move || animation_timeout_handle.with(|handle| handle.is_some()),
            )

            ref=wave_ref
        ></div>
    }
}

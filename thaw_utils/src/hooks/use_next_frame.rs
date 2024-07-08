// use leptos::{
//     leptos_dom::helpers::AnimationFrameRequestHandle, on_cleanup,
//     request_animation_frame_with_handle, StoredValue,
// };

use leptos::{
    prelude::{request_animation_frame_with_handle, AnimationFrameRequestHandle},
    reactive_graph::owner::{on_cleanup, StoredValue},
};

#[derive(Clone)]
pub struct NextFrame(StoredValue<Option<AnimationFrameRequestHandle>>);

impl Default for NextFrame {
    fn default() -> Self {
        Self(StoredValue::new(None))
    }
}

impl Copy for NextFrame {}

impl NextFrame {
    pub fn new() -> Self {
        let next_frame = NextFrame::default();

        on_cleanup(move || {
            next_frame.cancel();
        });

        next_frame
    }

    pub fn run(&self, cb: impl FnOnce() + 'static) {
        self.cancel();

        let next_frame_hadnle = self.0.clone();
        let handle = request_animation_frame_with_handle(move || {
            let handle = request_animation_frame_with_handle(cb).unwrap();
            next_frame_hadnle.set_value(Some(handle));
        })
        .unwrap();
        self.0.set_value(Some(handle));
    }

    pub fn cancel(&self) {
        self.0.update_value(|value| {
            if let Some(value) = value.take() {
                value.cancel();
            }
        });
    }
}

use leptos::{
    leptos_dom::helpers::AnimationFrameRequestHandle, on_cleanup,
    request_animation_frame_with_handle, StoredValue,
};

pub fn use_next_frame() -> NextFrame {
    let next_frame = NextFrame::default();

    on_cleanup(move || {
        next_frame.cancel();
    });

    next_frame
}

#[derive(Default, Clone)]
pub struct NextFrame(StoredValue<Option<AnimationFrameRequestHandle>>);

impl Copy for NextFrame {}

impl NextFrame {
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

// use leptos::{
//     leptos_dom::helpers::AnimationFrameRequestHandle, on_cleanup,
//     request_animation_frame_with_handle, StoredValue,
// };

use leptos::{
    prelude::window,
    reactive_graph::owner::{on_cleanup, StoredValue},
};
use wasm_bindgen::{closure::Closure, JsCast, JsValue, UnwrapThrowExt};

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
            let handle = request_animation_frame_with_handle(cb);
            next_frame_hadnle.set_value(Some(handle));
        });
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnimationFrameRequestHandle(i32);

impl AnimationFrameRequestHandle {
    pub fn cancel(&self) {
        let _ = window().cancel_animation_frame(self.0);
    }
}

fn request_animation_frame_with_handle(cb: impl FnOnce() + 'static) -> AnimationFrameRequestHandle {
    #[inline(never)]
    fn raf(cb: JsValue) -> Result<AnimationFrameRequestHandle, JsValue> {
        window()
            .request_animation_frame(cb.as_ref().unchecked_ref())
            .map(AnimationFrameRequestHandle)
    }

    raf(closure_once(cb)).expect_throw("request_animation_frame_with_handle")
}

fn closure_once(cb: impl FnOnce() + 'static) -> JsValue {
    let mut wrapped_cb: Option<Box<dyn FnOnce()>> = Some(Box::new(cb));
    let closure = Closure::new(move || {
        if let Some(cb) = wrapped_cb.take() {
            cb()
        }
    });
    closure.into_js_value()
}

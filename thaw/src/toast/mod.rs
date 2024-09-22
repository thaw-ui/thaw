mod toast;
mod toast_body;
mod toast_footer;
mod toast_title;
mod toaster;
mod toaster_provider;

pub use toast::*;
pub use toast_body::*;
pub use toast_footer::*;
pub use toast_title::*;
pub use toaster_provider::*;

use leptos::prelude::*;
use std::sync::mpsc::{channel, Receiver, Sender, TryIter};
use wasm_bindgen::UnwrapThrowExt;

#[derive(Clone, Copy)]
pub struct ToasterInjection {
    sender: StoredValue<Sender<(Children, ToastOptions)>>,
    trigger: StoredValue<ArcTrigger>,
}

impl ToasterInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }

    pub fn channel() -> (Self, ToasterReceiver) {
        let (sender, receiver) = channel::<(Children, ToastOptions)>();
        let trigger = ArcTrigger::new();

        (
            Self {
                sender: StoredValue::new(sender),
                trigger: StoredValue::new(trigger.clone()),
            },
            ToasterReceiver::new(receiver, trigger),
        )
    }

    pub fn dispatch_toast<C, IV>(&self, children: C, options: ToastOptions)
    where
        C: FnOnce() -> IV + Send + 'static,
        IV: IntoView + 'static,
    {
        self.sender.with_value(|sender| {
            sender
                .send((Box::new(move || children().into_any()), options))
                .unwrap_throw()
        });
        self.trigger.with_value(|trigger| trigger.notify());
    }
}

pub struct ToasterReceiver {
    receiver: Receiver<(Children, ToastOptions)>,
    trigger: ArcTrigger,
}

impl ToasterReceiver {
    pub fn new(receiver: Receiver<(Children, ToastOptions)>, trigger: ArcTrigger) -> Self {
        Self { receiver, trigger }
    }

    pub fn try_recv(&self) -> TryIter<'_, (Children, ToastOptions)> {
        self.trigger.track();
        self.receiver.try_iter()
    }
}

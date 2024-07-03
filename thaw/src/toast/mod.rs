mod toast;
mod toast_body;
mod toast_footer;
mod toast_title;
mod toaster;
mod toaster_provider;

pub use toast::*;
pub use toast_title::*;
pub use toaster_provider::*;

use leptos::{html::AnyElement, *};
use std::sync::mpsc::{channel, Receiver, Sender, TryIter};

#[derive(Clone)]
pub struct ToasterInjection {
    sender: Sender<(HtmlElement<AnyElement>, ToastOptions)>,
    trigger: Trigger,
}

impl ToasterInjection {
    pub fn use_() -> Self {
        expect_context()
    }

    pub fn channel() -> (Self, ToasterReceiver) {
        let (sender, receiver) = channel::<(HtmlElement<AnyElement>, ToastOptions)>();
        let trigger = Trigger::new();

        (
            Self { sender, trigger },
            ToasterReceiver::new(receiver, trigger),
        )
    }

    pub fn dispatch_toast(&self, any_element: HtmlElement<AnyElement>, options: ToastOptions) {
        self.sender.send((any_element, options)).unwrap();
        self.trigger.notify();
    }
}

pub struct ToasterReceiver {
    receiver: Receiver<(HtmlElement<AnyElement>, ToastOptions)>,
    trigger: Trigger,
}

impl ToasterReceiver {
    pub fn new(
        receiver: Receiver<(HtmlElement<AnyElement>, ToastOptions)>,
        trigger: Trigger,
    ) -> Self {
        Self { receiver, trigger }
    }

    pub fn try_recv(&self) -> TryIter<'_, (HtmlElement<AnyElement>, ToastOptions)> {
        self.trigger.track();
        self.receiver.try_iter()
    }
}

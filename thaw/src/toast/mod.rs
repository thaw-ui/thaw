mod toast;
mod toast_body;
mod toast_footer;
mod toast_title;
mod toaster;
mod toaster_provider;

use tachys::view::any_view::AnyView;
pub use toast::*;
pub use toast_title::*;
pub use toaster_provider::*;

use leptos::prelude::*;
use std::sync::mpsc::{channel, Receiver, Sender, TryIter};

#[derive(Clone)]
pub struct ToasterInjection {
    sender: Sender<(AnyView<Dom>, ToastOptions)>,
    trigger: ArcTrigger,
}

impl ToasterInjection {
    pub fn use_() -> Self {
        expect_context()
    }

    pub fn channel() -> (Self, ToasterReceiver) {
        let (sender, receiver) = channel::<(AnyView<Dom>, ToastOptions)>();
        let trigger = ArcTrigger::new();

        (
            Self { sender, trigger },
            ToasterReceiver::new(receiver, trigger),
        )
    }

    pub fn dispatch_toast(&self, any_view: AnyView<Dom>, options: ToastOptions) {
        self.sender.send((any_view, options)).unwrap();
        self.trigger.trigger();
    }
}

pub struct ToasterReceiver {
    receiver: Receiver<(AnyView<Dom>, ToastOptions)>,
    trigger: ArcTrigger,
}

impl ToasterReceiver {
    pub fn new(receiver: Receiver<(AnyView<Dom>, ToastOptions)>, trigger: ArcTrigger) -> Self {
        Self { receiver, trigger }
    }

    pub fn try_recv(&self) -> TryIter<'_, (AnyView<Dom>, ToastOptions)> {
        self.trigger.track();
        self.receiver.try_iter()
    }
}

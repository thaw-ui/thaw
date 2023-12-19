use cfg_if::cfg_if;
use leptos::{html::AnyElement, HtmlElement, MaybeSignal};

pub fn dyn_classes(el: HtmlElement<AnyElement>, classes_signal: MaybeSignal<String>) {
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            let _ = el;
            let _ = classes_signal;
        } else {
            use leptos::SignalGet;
            let _ = el.dyn_classes(move || {
                let classes = classes_signal.get();
                if classes.is_empty() {
                    return vec![];
                }
                classes
                    .split_ascii_whitespace()
                    .map(|class| class.to_string())
                    .collect::<Vec<String>>()
            });
        }
    };
}

pub fn ssr_class(class: &MaybeSignal<String>) -> String {
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            use leptos::SignalGetUntracked;
            class.get_untracked()
        } else {
            let _ = class;
            String::new()
        }
    }
}

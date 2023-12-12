use leptos::{html::AnyElement, HtmlElement, MaybeSignal, SignalGet};

pub fn dyn_classes(el: HtmlElement<AnyElement>, classes_signal: MaybeSignal<String>) {
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

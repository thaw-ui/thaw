use crate::focus::apply_focus_visible_polyfill;
use leptos::{html, prelude::*};
use web_sys::Document;

pub fn use_focus_visible(target_document: Option<Document>) -> NodeRef<html::Div> {
    let scope_ref = NodeRef::<html::Div>::new();

    Effect::new(move || {
        let Some(target_document) = target_document.clone() else {
            return;
        };
        let Some(default_view) = target_document.default_view() else {
            return;
        };

        if let Some(scope) = scope_ref.get() {
            let rt = apply_focus_visible_polyfill(scope.into(), default_view);
            Owner::on_cleanup(rt);
        }
    });

    scope_ref
}

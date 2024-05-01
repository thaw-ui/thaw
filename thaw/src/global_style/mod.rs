use crate::{use_theme, Theme};
use leptos::*;

#[component]
pub fn GlobalStyle() -> impl IntoView {
    let theme = use_theme(Theme::light);
    create_effect(move |_| {
        theme.with(|theme| {
            if let Some(body) = document().body() {
                _ = body
                    .style()
                    .set_property("background-color", &theme.common.background_color);
                _ = body.style().set_property("color", &theme.common.font_color);
                _ = body
                    .style()
                    .set_property("font-size", &theme.common.font_size);
                _ = body
                    .style()
                    .set_property("color-scheme", &theme.common.color_scheme);
                _ = body.style().set_property("margin", "0");
            }
        });
    });
}

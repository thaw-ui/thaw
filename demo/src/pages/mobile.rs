use leptos::*;
use melt_ui::{use_theme, Theme};

#[component]
pub fn MobilePage(path: &'static str) -> impl IntoView {
    let theme = use_theme(Theme::light);
    let src = create_memo(move |_| theme.with(|theme| format!("{path}&theme={}", theme.name)));
    let style = create_memo(move |_| {
        theme.with(|theme| {
            let mut style = String::from("margin-top: 5vh; width: 350px; height: 680px; border-radius: 16px; box-shadow: 0 6px 16px -9px rgba(0, 0, 0, .08), 0 9px 28px 0 rgba(0, 0, 0, .05), 0 12px 48px 16px rgba(0, 0, 0, .03);");
            style.push_str(&format!("border: 1px solid {}; ", theme.common.border_color));
            style
        })
    });
    view! {
        <div style="height: 100vh; width: 400px; text-align: center">
            <iframe
                src=move || src.get()
                style=move || style.get()
            ></iframe>
        </div>
    }
}

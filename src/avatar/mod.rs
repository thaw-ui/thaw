use crate::mount_style;
use leptos::*;

#[component]
pub fn Avatar(
    #[prop(optional, into)] src: MaybeSignal<String>,
    #[prop(optional, into)] circle: MaybeSignal<bool>,
    #[prop(default = MaybeSignal::Static(30), into)] size: MaybeSignal<i32>,
) -> impl IntoView {
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        css_vars.push_str(&format!("--size: {}px;", size.get()));
        css_vars.push_str(&format!(
            "--border-radius: {};",
            if circle.get() { "50%" } else { "3px" }
        ));
        css_vars
    });
    mount_style("avatar", include_str!("./avatar.css"));
    view! {
        <span class="melt-avatar" style=move || css_vars.get()>
            {move || {
                let src = src.get();
                (!src.is_empty()).then(|| view! {
                    <img src=src />
                })
            }}
        </span>
    }
}

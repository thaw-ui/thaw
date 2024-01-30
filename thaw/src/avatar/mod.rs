mod theme;

use crate::{
    components::OptionComp,
    use_theme,
    utils::{class_list::class_list, mount_style, OptionalProp},
    Theme,
};
use leptos::*;
pub use theme::AvatarTheme;

#[component]
pub fn Avatar(
    #[prop(optional, into)] src: Option<MaybeSignal<String>>,
    #[prop(optional, into)] round: MaybeSignal<bool>,
    #[prop(default = MaybeSignal::Static(30), into)] size: MaybeSignal<u16>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
) -> impl IntoView {
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        css_vars.push_str(&format!("--thaw-size: {}px;", size.get()));
        css_vars.push_str(&format!(
            "--thaw-border-radius: {};",
            if round.get() { "50%" } else { "3px" }
        ));
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-background-color: {}",
                theme.avatar.background_color
            ));
        });
        css_vars
    });
    mount_style("avatar", include_str!("./avatar.css"));

    view! {
        <span
            class=class_list!["thaw-avatar", class.map(| c | move || c.get())]
            style=move || css_vars.get()
        >
            <OptionComp value=src let:src>
                <img src=move || src.get()/>
            </OptionComp>
        </span>
    }
}

use crate::{use_theme, utils::mount_style::mount_style, Theme};
use leptos::*;

#[component]
pub fn UploadDragger(children: Children) -> impl IntoView {
    mount_style("upload-dragger", include_str!("./upload-dragger.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            let border_color_hover = theme.common.color_primary.clone();
            css_vars.push_str(&format!("--border-color-hover: {border_color_hover};"));
        });
        css_vars
    });
    view! {
        <div class="melt-upload-dragger" style=css_vars>
            {children()}
        </div>
    }
}

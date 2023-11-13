use crate::{use_theme, utils::mount_style, Theme};
use leptos::*;

#[component]
pub fn UploadDragger(children: Children) -> impl IntoView {
    mount_style("upload-dragger", include_str!("./upload-dragger.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-background-color: {};",
                theme.upload.dragger_background_color
            ));
            css_vars.push_str(&format!(
                "--thaw-border-color: {};",
                theme.upload.dragger_border_color
            ));
            let border_color_hover = theme.common.color_primary.clone();
            css_vars.push_str(&format!("--thaw-border-color-hover: {border_color_hover};"));
        });
        css_vars
    });
    view! {
        <div class="thaw-upload-dragger" style=css_vars>
            {children()}
        </div>
    }
}

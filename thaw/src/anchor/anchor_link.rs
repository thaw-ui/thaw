use crate::use_anchor;
use leptos::*;
use thaw_components::OptionComp;

#[component]
pub fn AnchorLink(
    #[prop(into)] title: MaybeSignal<String>,
    #[prop(into)] href: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    if !href.is_empty() {
        let anchor = use_anchor();
        let mut href_id = None::<String>;
        anchor.0.update(|ids| {
            if href.starts_with('#') {
                let id = href[1..].to_string();
                href_id = Some(id.clone());
                ids.push(id);
            }
        });

        on_cleanup(move || {
            anchor.0.update(|ids| {
                if let Some(index) = ids.iter().position(|id| Some(id) == href_id.as_ref()) {
                    ids.remove(index);
                }
            });
        });
    }
    view! {
        <div class="thaw-anchor-link">
            <a href=href class="thaw-anchor-link__title">
                {move || title.get()}
            </a>
            <OptionComp value=children let:children>
                {children()}
            </OptionComp>
        </div>
    }
}

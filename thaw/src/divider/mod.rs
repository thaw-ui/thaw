use leptos::prelude::*;
use thaw_components::OptionComp;
use thaw_utils::class_list;

#[cfg(feature = "manganis")]
const _: manganis::Asset = manganis::asset!("/src/divider/divider.css");

#[component]
pub fn Divider(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// A divider can be horizontal (default) or vertical.
    #[prop(optional, into)]
    vertical: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    #[cfg(not(feature = "manganis"))]
    thaw_utils::mount_style("divider", include_str!("./divider.css"));

    view! {
        <div
            class=class_list![
                "thaw-divider",
                ("thaw-divider--vertical", move || vertical.get()),
                class
            ]
            aria-orientation=move || if vertical.get() { "vertical" } else { "horizontal" }
            role="separator"
        >
            <OptionComp value=children let:children>
                <div class="thaw-divider__wrapper">{children()}</div>
            </OptionComp>
        </div>
    }
}

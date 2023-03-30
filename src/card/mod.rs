use crate::utils::mount_style::mount_style;
use leptos::*;
use stylers::style_sheet_str;

#[component]
pub fn Card(
    cx: Scope,
    #[prop(default = None)] title: Option<String>,
    #[prop(default = None)] header: Option<Children>,
    #[prop(default = None)] header_extra: Option<Children>,
    children: Children,
    #[prop(default = None)] footer: Option<Children>,
) -> impl IntoView {
    let class_name = mount_style("card", || style_sheet_str!("./src/card/card.css"));
    view! {
        cx, class=class_name,
        <div class="melt-card">
            {
                if header.is_some() || title.is_some() {
                    view! {
                        cx, class=class_name,
                        <div class="melt-card__header">
                            <div class="melt-card__header-content">
                                {
                                    if let Some(header) = header {
                                        view! {
                                            cx,
                                            <>
                                                { header(cx) }
                                            </>
                                        }
                                    } else {
                                        view! {
                                            cx,
                                            <>
                                                { title }
                                            </>
                                        }
                                    }
                                }
                            </div>
                            {
                                if let Some(header_extra) = header_extra {
                                    view! {
                                        cx, class=class_name,
                                        <div class="melt-card__header-extra">
                                            { header_extra(cx)}
                                        </div>
                                    }.into()
                                } else {
                                    None
                                }
                            }
                        </div>
                    }.into()
                } else {
                    None
                }
            }
            <div class="melt-card__content">
                { children(cx) }
            </div>
            {
                if let Some(footer) = footer {
                    view! {
                        cx, class=class_name,
                        <div class="melt-card__footer">
                            { footer(cx) }
                        </div>
                    }.into()
                } else {
                    None
                }
            }
         </div>
    }
}

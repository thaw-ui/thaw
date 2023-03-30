use crate::card::*;
use crate::teleport::*;
use crate::utils::mount_style::mount_style;
use leptos::*;
use stylers::style_sheet_str;

#[component]
pub fn Modal(
    cx: Scope,
    #[prop(default = None)] title: Option<String>,
    children: Children,
    #[prop(default = None)] footer: Option<Children>,
    open: ReadSignal<bool>,
    #[prop(default = None)] on_cancel: Option<Box<dyn Fn() + 'static>>,
) -> impl IntoView {
    let class_name = mount_style("modal", || style_sheet_str!("./src/modal/modal.css"));
    let header = move |cx| {
        view! {
            cx, class=class_name,
            <>
                <span class="melt-model-title">
                    {title}
                </span>
            </>
        }
    };
    let header_extra = |cx| {
        view! {
            cx,
            <>
                <span style="cursor: pointer;" on:click=move |_| if let Some(on_cancel) = &on_cancel { on_cancel()}>
                    { "x" }
                </span>
            </>
        }
    };
    view! {
        cx, class=class_name,
        <Teleport>
            <div class="melt-modal-container" style=move || if open.get() { "" } else { "display: none" }>
                <div class="melt-modal-mask"></div>
                <div class="melt-modal-body">
                    <Card header=Some(Box::new(header)) header_extra=Some(Box::new(header_extra)) footer=footer>
                        {children(cx)}
                    </Card>
                </div>
            </div>
        </Teleport>
    }
}

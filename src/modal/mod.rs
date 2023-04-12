use crate::card::*;
use crate::teleport::*;
use crate::utils::mount_style::mount_style;
use leptos::*;
use stylers::style_sheet_str;
use leptos_icons::*;

#[component]
pub fn Modal(
    cx: Scope,
    #[prop(optional, into)] title: Option<MaybeSignal<String>>,
    children: Children,
    #[prop(optional)] footer: Option<Children>,
    #[prop(optional, into)] open: MaybeSignal<bool>,
    #[prop(optional)] on_cancel: Option<SignalSetter<()>>,
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

    let header_extra = move |cx| {
        view! {
            cx,
            <>
                <span style="cursor: pointer;" on:click=move |_| if let Some(on_cancel) = &on_cancel { on_cancel.set(())}>
                    <LeptosIcon icon=AiIcon::AiCloseOutlined/>
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

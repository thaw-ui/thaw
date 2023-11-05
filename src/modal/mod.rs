use crate::card::*;
use crate::components::OptionComp;
use crate::icon::*;
use crate::teleport::*;
use crate::utils::mount_style::mount_style;
use crate::utils::StoredMaybeSignal;
use leptos::*;

#[slot]
pub struct ModalFooter {
    children: ChildrenFn,
}

#[component]
pub fn Modal(
    #[prop(into)] show: RwSignal<bool>,
    #[prop(optional, into)] title: MaybeSignal<String>,
    children: Children,
    #[prop(optional)] modal_footer: Option<ModalFooter>,
) -> impl IntoView {
    mount_style("modal", include_str!("./modal.css"));
    let title: StoredMaybeSignal<_> = title.into();

    view! {
        <Teleport>
            <div
                class="thaw-modal-container"
                style=move || if show.get() { "" } else { "display: none" }
            >
                <div class="thaw-modal-mask"></div>
                <div class="thaw-modal-body">
                    <Card>
                        <CardHeader slot>
                            <span class="thaw-model-title">{move || title.get()}</span>
                        </CardHeader>
                        <CardHeaderExtra slot>
                            <span style="cursor: pointer;" on:click=move |_| show.set(false)>
                                <Icon icon=Icon::from(AiIcon::AiCloseOutlined)/>
                            </span>
                        </CardHeaderExtra>
                        {children()}
                        <CardFooter slot if_=modal_footer.is_some()>
                            <OptionComp value=modal_footer.as_ref() let:footer>
                                {(footer.children)()}
                            </OptionComp>
                        </CardFooter>
                    </Card>
                </div>
            </div>
        </Teleport>
    }
}

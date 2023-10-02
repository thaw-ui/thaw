use crate::card::*;
use crate::components::OptionComp;
use crate::icon::*;
use crate::teleport::*;
use crate::utils::mount_style::mount_style;
use leptos::*;
use stylers::style_sheet_str;

#[slot]
pub struct ModalFooter {
    children: ChildrenFn,
}

#[component]
pub fn Modal(
    #[prop(into)] show: RwSignal<bool>,
    #[prop(optional, into)] title: MaybeSignal<&'static str>,
    children: Children,
    #[prop(optional)] modal_footer: Option<ModalFooter>,
) -> impl IntoView {
    let class_name = mount_style("modal", || style_sheet_str!("./src/modal/modal.css"));

    view! {
         class=class_name,
        <Teleport>
            <div class="melt-modal-container" style=move || if show.get() { "" } else { "display: none" }>
                <div class="melt-modal-mask"></div>
                <div class="melt-modal-body">
                    <Card>
                        <CardHeader slot>
                            <span class="melt-model-title">
                                { title.get() }
                            </span>
                        </CardHeader>
                        <CardHeaderExtra slot>
                            <span style="cursor: pointer;" on:click=move |_| show.set(false)>
                                <Icon icon=Icon::from(AiIcon::AiCloseOutlined)/>
                            </span>
                        </CardHeaderExtra>
                        { children() }
                        <CardFooter slot if_=modal_footer.is_some()>
                            <OptionComp value=modal_footer.as_ref() let:footer>
                                { (footer.children)() }
                            </OptionComp>
                        </CardFooter>
                    </Card>
                </div>
            </div>
        </Teleport>
    }
}

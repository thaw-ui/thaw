use crate::{
    components::{OptionComp, Teleport},
    utils::{class_list::class_list, mount_style, StoredMaybeSignal},
    Card, CardFooter, CardHeader,
};
use leptos::*;

#[slot]
pub struct DrawerFooter {
    children: ChildrenFn,
}

#[component]
pub fn Drawer(
    #[prop(into)] show: RwSignal<bool>,
    #[prop(optional, into)] title: MaybeSignal<String>,
    #[prop(into)] placement: MaybeSignal<DrawerPlacement>,
    children: Children,
    #[prop(optional)] drawer_footer: Option<DrawerFooter>,
) -> impl IntoView {
    mount_style("drawer", include_str!("./drawer.css"));
    let title: StoredMaybeSignal<_> = title.into();

    view! {
        <Teleport>
            <div
                class="thaw-drawer-container"
                style=move || if show.get() { "" } else { "display: none" }
            >
                <div class="thaw-drawer-mask"></div>
                <div class=class_list!["thaw-drawer", move || format!("thaw-drawer--placement-{}", placement.get().as_str())]>
                    <Card>
                        <CardHeader slot>
                            <span class="thaw-drawer-title">{move || title.get()}</span>
                        </CardHeader>
                        {children()}
                        <CardFooter slot if_=drawer_footer.is_some()>
                            <OptionComp value=drawer_footer.as_ref() let:footer>
                                {(footer.children)()}
                            </OptionComp>
                        </CardFooter>
                    </Card>
                </div>
            </div>
        </Teleport>
    }
}

#[derive(Clone)]
pub enum DrawerPlacement {
    Top,
    Bottom,
    Left,
    Right,
}

impl DrawerPlacement {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

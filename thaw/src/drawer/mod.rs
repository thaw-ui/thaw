use crate::{
    components::Teleport,
    utils::{class_list::class_list, mount_style, Model, OptionalProp},
    Card,
};
use leptos::*;

#[component]
pub fn Drawer(
    #[prop(into)] show: Model<bool>,
    #[prop(optional, into)] title: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] placement: MaybeSignal<DrawerPlacement>,
    #[prop(default = MaybeSignal::Static("520px".to_string()), into)] width: MaybeSignal<String>,
    #[prop(default = MaybeSignal::Static("260px".to_string()), into)] height: MaybeSignal<String>,
    #[prop(default = 2000.into(), into)] z_index: MaybeSignal<i16>,
    #[prop(optional, into)] mount: DrawerMount,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    children: Children,
) -> impl IntoView {
    mount_style("drawer", include_str!("./drawer.css"));
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        css_vars.push_str(&format!("--thaw-width: {};", width.get()));
        css_vars.push_str(&format!("--thaw-height: {};", height.get()));
        css_vars
    });

    #[component]
    fn DrawerInnr(
        show: Model<bool>,
        title: OptionalProp<MaybeSignal<String>>,
        placement: MaybeSignal<DrawerPlacement>,
        z_index: MaybeSignal<i16>,
        class: OptionalProp<MaybeSignal<String>>,
        css_vars: Memo<String>,
        children: Children,
    ) -> impl IntoView {
        view! {
            <div
                class="thaw-drawer-container"
                style=move || if show.get() { format!("z-index: {}", z_index.get()) } else { "display: none".to_string() }
            >
                <div class="thaw-drawer-mask" on:click=move |_| show.set(false)></div>
                <div
                    class=class_list![
                        "thaw-drawer", move || format!("thaw-drawer--placement-{}", placement.get()
                        .as_str()), class.map(| c | move || c.get())
                    ]

                    style=move || css_vars.get()
                >
                    <Card title>{children()}</Card>
                </div>
            </div>
        }
    }

    match mount {
        DrawerMount::None => view! {
            <DrawerInnr show title placement z_index class css_vars children />
        },
        DrawerMount::Body => view! {
            <Teleport>
                <DrawerInnr show title placement z_index class css_vars children />
            </Teleport>
        },
    }
}

#[derive(Clone, Default)]
pub enum DrawerPlacement {
    Top,
    Bottom,
    Left,
    #[default]
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

#[derive(Default)]
pub enum DrawerMount {
    None,
    #[default]
    Body,
}

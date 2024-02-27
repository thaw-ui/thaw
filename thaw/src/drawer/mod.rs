use crate::{
    components::{CSSTransition, Teleport},
    utils::{class_list::class_list, mount_style, use_lock_html_scroll, Model, OptionalProp},
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
    let style = create_memo(move |_| {
        let mut style = String::new();

        style.push_str(&format!("--thaw-width: {};", width.get()));
        style.push_str(&format!("--thaw-height: {};", height.get()));

        style.push_str(&format!("z-index: {};", z_index.get()));
        style
    });

    #[component]
    fn DrawerInnr(
        show: Model<bool>,
        title: OptionalProp<MaybeSignal<String>>,
        placement: MaybeSignal<DrawerPlacement>,
        class: OptionalProp<MaybeSignal<String>>,
        style: Memo<String>,
        children: Children,
    ) -> impl IntoView {
        let mask_ref = NodeRef::<html::Div>::new();
        let drawer_ref = NodeRef::<html::Div>::new();

        let is_css_transition = RwSignal::new(false);
        let placement = Memo::new(move |prev| {
            let placement = placement.get().as_str();
            let Some(prev) = prev else {
                return placement;
            };

            if is_css_transition.get() {
                prev
            } else {
                placement
            }
        });
        let on_after_enter = move |_| {
            is_css_transition.set(false);
        };

        let is_lock = RwSignal::new(show.get_untracked());
        Effect::new(move |_| {
            if show.get() {
                is_lock.set(true);
                is_css_transition.set(true);
            }
        });
        use_lock_html_scroll(is_lock.into());
        let on_after_leave = move |_| {
            is_lock.set(false);
            is_css_transition.set(false);
        };

        view! {
            <div class="thaw-drawer-container" style=move || style.get()>
                <CSSTransition
                    node_ref=mask_ref
                    show=show.signal()
                    name="fade-in-transition"
                    let:display
                >
                    <div
                        class="thaw-drawer-mask"
                        style=move || display.get()
                        on:click=move |_| show.set(false)
                        ref=mask_ref
                    ></div>
                </CSSTransition>
                <CSSTransition
                    node_ref=drawer_ref
                    show=show.signal()
                    name=Memo::new(move |_| {
                        format!("slide-in-from-{}-transition", placement.get())
                    })

                    on_after_enter
                    on_after_leave
                    let:display
                >
                    <div
                        class=class_list![
                            "thaw-drawer", move || format!("thaw-drawer--placement-{}", placement
                            .get()), class.map(| c | move || c.get())
                        ]

                        style=move || display.get()
                        ref=drawer_ref
                        role="dialog"
                        aria-modal="true"
                    >
                        <Card title>{children()}</Card>
                    </div>
                </CSSTransition>
            </div>
        }
    }

    match mount {
        DrawerMount::None => view! { <DrawerInnr show title placement class style children/> },
        DrawerMount::Body => view! {
            <Teleport>
                <DrawerInnr show title placement class style children/>
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

impl Copy for DrawerPlacement {}

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

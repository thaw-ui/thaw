use crate::ConfigInjection;
use leptos::{context::Provider, prelude::*, tachys::html::node_ref::node_ref};
use leptos_transition_group::CSSTransition;
use thaw_components::{use_binder, Follower, FollowerInjection, Teleport, UseBinder};
use thaw_utils::BoxCallback;

#[component]
pub fn Binder<T, FT>(
    #[prop(optional, into)] on_css_transition_after_leave: Option<BoxCallback>,
    follower: Follower<FT>,
    children: TypedChildren<T>,
) -> impl IntoView
where
    T: AddAnyAttr + IntoView + Send + 'static,
    FT: AddAnyAttr + IntoView + Send + 'static,
{
    let config_provider = ConfigInjection::expect_context();

    let Follower {
        show: follower_show,
        width: follower_width,
        placement: follower_placement,
        children: follower_children,
        auto_height,
        arrow,
    } = follower;

    let UseBinder {
        target_ref,
        content_ref,
        follower_ref,
        placement,
        sync_position,
        ensure_listener,
        remove_listener,
    } = use_binder(follower_width, follower_placement, auto_height, arrow);

    let follower_injection = FollowerInjection::new({
        let sync_position = sync_position.clone();
        move || sync_position()
    });

    let on_before_enter = {
        let sync_position = sync_position.clone();
        move |_| {
            sync_position();
        }
    };

    Effect::new(move |_| {
        if target_ref.get().is_none() {
            return;
        }
        if content_ref.get().is_none() {
            return;
        }
        if follower_show.get() {
            sync_position();

            remove_listener();
            ensure_listener();
        } else {
            remove_listener();
        }
    });

    let on_after_leave = move |_| {
        if let Some(on_css_transition_after_leave) = &on_css_transition_after_leave {
            on_css_transition_after_leave();
        }
    };

    view! {
        {children.into_inner()().into_inner().add_any_attr(node_ref(target_ref))}
        <Teleport immediate=follower_show>
            <div
                class="thaw-config-provider thaw-binder-follower"
                node_ref=follower_ref
                data-thaw-placement=move || placement.get().as_str()
                data-thaw-id=config_provider.id()
            >
                <Provider value=follower_injection>
                    <CSSTransition
                        name="thaw-fade-in-scale-up-transition"
                        show=follower_show
                        on_before_enter
                        on_after_leave
                    >
                        {follower_children
                            .into_inner()()
                            .into_inner()
                            .add_any_attr(node_ref(content_ref))}
                    </CSSTransition>
                </Provider>
            </div>
        </Teleport>
    }
}

use super::{message_provider::MessageType, Message};
use leptos::*;
use uuid::Uuid;

#[component]
pub fn MessageEnvironment(
    message: MessageType,
    #[prop(into)] on_internal_after_leave: Callback<Uuid>,
) -> impl IntoView {
    let (id, content, variant, options) = message;
    set_timeout(
        move || {
            on_internal_after_leave.call(id);
        },
        options.duration,
    );

    view! { <Message content variant/> }
}

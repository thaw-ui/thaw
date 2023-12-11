use std::time::Duration;

use super::{message_environment::MessageEnvironment, MessageVariant};
use crate::{components::Teleport, utils::mount_style};
use leptos::*;
use uuid::Uuid;

#[component]
pub fn MessageProvider(children: Children) -> impl IntoView {
    mount_style("message", include_str!("./message.css"));

    let message_list = create_rw_signal::<Vec<MessageType>>(vec![]);

    let handle_after_leave = move |id| {
        message_list.update(move |message_list| {
            let Some(index) = message_list.iter().position(|message| id == message.0) else {
                return;
            };
            message_list.remove(index);
        });
    };

    view! {
        <Provider value=MessageInjection::new(
            message_list,
        )>
            {children()} <Teleport>
                <div class="thaw-message-container">
                    <For
                        each=move || message_list.get()
                        key=|message| message.0
                        children=move |message| {
                            view! {
                                <MessageEnvironment
                                    message
                                    on_internal_after_leave=handle_after_leave
                                />
                            }
                        }
                    />

                </div>
            </Teleport>
        </Provider>
    }
}

pub(crate) type MessageType = (Uuid, String, MessageVariant, MessageOptions);

#[derive(Clone)]
pub struct MessageOptions {
    pub duration: Duration,
}

impl Default for MessageOptions {
    fn default() -> Self {
        Self {
            duration: Duration::from_secs(3),
        }
    }
}

#[derive(Clone)]
pub struct MessageInjection {
    message_list: RwSignal<Vec<MessageType>>,
}
impl Copy for MessageInjection {}

impl MessageInjection {
    fn new(message_list: RwSignal<Vec<MessageType>>) -> Self {
        Self { message_list }
    }

    pub fn create(&self, content: String, variant: MessageVariant, options: MessageOptions) {
        self.message_list.update(move |message_list| {
            let id = uuid::Uuid::new_v4();
            message_list.push((id, content, variant, options));
        });
    }
}

pub fn use_message() -> MessageInjection {
    expect_context::<MessageInjection>()
}

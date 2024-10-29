use super::ToastMdPage;
use crate::pages::MobilePage;
use leptos::*;
use std::time::Duration;
use thaw::mobile::*;
use thaw::*;

#[component]
pub fn ToastPage() -> impl IntoView {
    view! {
        <div style="display: flex">
            <ToastMdPage />
            <div>
                <MobilePage path="/?path=/mobile/toast" />
            </div>
        </div>
    }
}

#[component]
pub fn ToastDemoPage() -> impl IntoView {
    let count = create_rw_signal(0u32);
    let onclick = move |_| {
        show_toast(ToastOptions {
            message: format!("Hello {}", count.get_untracked()),
            duration: Duration::from_millis(2000),
        });
        count.set(count.get_untracked() + 1);
    };
    view! {
        <div style="margin: 20px">
            <Button on_click=onclick>"hi"</Button>
        </div>
    }
}

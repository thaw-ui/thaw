use leptos::*;
use melt_ui::mobile::*;
use melt_ui::*;
use std::time::Duration;

#[component]
pub fn ToastPage() -> impl IntoView {
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
            <Button on:click=onclick>"hi"</Button>
        </div>
    }
}

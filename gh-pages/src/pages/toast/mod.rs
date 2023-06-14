use std::time::Duration;
use leptos::*;
use melt_ui::mobile::*;
use melt_ui::*;

#[component]
pub fn ToastPage(cx: Scope) -> impl IntoView {
    let count = create_rw_signal(cx, 0u32);
    let onclick = move |_| {
        show_toast(
            cx,
            ToastOptions {
                message: format!("Hello {}", count.get_untracked()),
                duration: Duration::from_millis(2000),
            },
        );
        count.set(count.get_untracked() + 1);
    };
    view! {cx,
        <div style="margin: 20px">
            <Button on:click=onclick>"hi"</Button>
        </div>
    }
}

use leptos::*;

#[component]
pub fn MobilePage(path: &'static str) -> impl IntoView {
    view! {
        <div style="height: 100vh; width: 400px; text-align: center">
            <iframe src=path style="margin-top: 5vh; width: 350px; height: 680px; border-radius: 16px; border: 1px solid #eee; box-shadow: #ebedf0 0 4px 12px;"/>
        </div>
    }
}

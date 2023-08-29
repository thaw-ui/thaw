use leptos::*;

#[component]
pub fn MobilePage(path: &'static str) -> impl IntoView {
    view! {
        <div style="height: 100vh; display: flex; align-items: center; justify-content: center; background: #eff2f5">
            <iframe src=path style="width: 360px; height: 640px; background-color: #fff; border: none; border-radius: 16px; border: 1px solid #e1e1e1"/>
        </div>
    }
}

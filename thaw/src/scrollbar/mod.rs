use leptos::*;

#[component]
pub fn Scrollbar(children: Children) -> impl IntoView {
    view! {
        <div class="thaw-scrollbar">
            <div class="thaw-scrollbar__container">
                <div class="thaw-scrollbar__content">
                    {children()}
                </div>
            </div>
            <div class="jo-scrollbar__track--vertical">
                <div class="jo-scrollabr__thumb"></div>
            </div>
            <div class="jo-scrollbar__track--horizontal">
                <div class="jo-scrollabr__thumb"></div>
            </div>
        </div>
    }
}

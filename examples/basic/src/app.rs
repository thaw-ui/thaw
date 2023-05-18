use crate::pages::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <Routes>
                <Route path="/" view=move |cx| view! {cx,
                    <Home />
                } />
                <Route path="/slider" view=move |cx| view! {cx,
                    <SliderPage />
                } />
            </Routes>
        </Router>
    }
}

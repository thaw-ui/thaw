use crate::components::*;
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
                <Route path="/menu" view=move |cx| view! {cx,
                    <MenuPage />
                } />
                <Route path="/slider" view=move |cx| view! {cx,
                    <SliderPage />
                } />
                <Route path="/components" view=move |cx| view! {cx,
                    <ComponentsPage />
                } >
                    <Route path="/menu" view=move |cx| view! {cx,
                        <MenuPage />
                    } />
                    <Route path="/slider" view=move |cx| view! {cx,
                        <SliderPage />
                    } />
                </Route>
            </Routes>
        </Router>
    }
}

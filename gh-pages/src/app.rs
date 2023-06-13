use crate::pages::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router base="/melt-ui">
            <Routes base="/melt-ui".to_string() >
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
                    <Route path="/tabbar" view=move |cx| view! {cx,
                        <MobilePage path="/melt-ui?path=/mobile/tabbar" />
                    } />
                    <Route path="/nav-bar" view=move |cx| view! {cx,
                        <MobilePage path="/melt-ui?path=/mobile/nav-bar" />
                    } />
                    <Route path="/input" view=move |cx| view! {cx,
                        <InputPage />
                    } />
                    <Route path="/image" view=move |cx| view! {cx,
                        <ImagePage />
                    } />
                    <Route path="/modal" view=move |cx| view! {cx,
                        <ModalPage />
                    } />
                    <Route path="/button" view=move |cx| view! {cx,
                        <ButtonPage />
                    } />
                    <Route path="/checkbox" view=move |cx| view! {cx,
                        <CheckboxPage />
                    } />
                </Route>
            </Routes>
            <Routes base="/melt-ui/mobile".to_string()>
                <Route path="/tabbar" view=move |cx| view! {cx,
                    <TabbarPage />
                } />
                <Route path="/nav-bar" view=move |cx| view! {cx,
                    <NavBarPage />
                } />
            </Routes>
        </Router>
    }
}

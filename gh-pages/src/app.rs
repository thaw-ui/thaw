use crate::pages::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router base="/melt-ui">
            <Routes base="/melt-ui".to_string() >
                <Route path="/" view=Home />
                <Route path="/components" view=ComponentsPage>
                    <Route path="/menu" view=MenuPage />
                    <Route path="/slider" view=SliderPage />
                    <Route path="/tabbar" view=TabbarPage />
                    <Route path="/nav-bar" view=|| view! {
                        <MobilePage path="/melt-ui?path=/mobile/nav-bar" />
                    } />
                    <Route path="/input" view=InputPage />
                    <Route path="/image" view=ImagePage />
                    <Route path="/modal" view=ModalPage />
                    <Route path="/button" view=ButtonPage />
                    <Route path="/checkbox" view=CheckboxPage />
                    <Route path="/toast" view=|| view! {
                        <MobilePage path="/melt-ui?path=/mobile/toast" />
                    } />
                    <Route path="/tabs" view=TabsPage />
                    <Route path="/select" view=SelectPage />
                </Route>
                <Route path="/mobile/tabbar" view=TabbarDemoPage />
                <Route path="/mobile/nav-bar" view=NavBarPage />
                <Route path="/mobile/toast" view=ToastPage />
            </Routes>
        </Router>
    }
}

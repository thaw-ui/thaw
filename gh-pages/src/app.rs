use crate::pages::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router base="/melt-ui">
            <Routes base="/melt-ui".to_string() >
                <Route path="/" view=move || view! {
                    <Home />
                } />
                <Route path="/menu" view=move || view! {
                    <MenuPage />
                } />
                <Route path="/slider" view=move || view! {
                    <SliderPage />
                } />
                <Route path="/components" view=move || view! {
                    <ComponentsPage />
                } >
                    <Route path="/menu" view=move || view! {
                        <MenuPage />
                    } />
                    <Route path="/slider" view=move || view! {
                        <SliderPage />
                    } />
                    <Route path="/tabbar" view=move || view! {
                        <MobilePage path="/melt-ui?path=/mobile/tabbar" />
                    } />
                    <Route path="/nav-bar" view=move || view! {
                        <MobilePage path="/melt-ui?path=/mobile/nav-bar" />
                    } />
                    <Route path="/input" view=move || view! {
                        <InputPage />
                    } />
                    <Route path="/image" view=move || view! {
                        <ImagePage />
                    } />
                    <Route path="/modal" view=move || view! {
                        <ModalPage />
                    } />
                    <Route path="/button" view=move || view! {
                        <ButtonPage />
                    } />
                    <Route path="/checkbox" view=move || view! {
                        <CheckboxPage />
                    } />
                    <Route path="/toast" view=move || view! {
                        <MobilePage path="/melt-ui?path=/mobile/toast" />
                    } />
                    <Route path="/tabs" view=move || view! {
                        <TabsPage />
                    } />
                    <Route path="/select" view=move || view! {
                        <SelectPage />
                    } />
                </Route>
            </Routes>
            <Routes base="/melt-ui/mobile".to_string()>
                <Route path="/tabbar" view=move || view! {
                    <TabbarPage />
                } />
                <Route path="/nav-bar" view=move || view! {
                    <NavBarPage />
                } />
                <Route path="/toast" view=move || view! {
                    <ToastPage />
                } />
            </Routes>
        </Router>
    }
}

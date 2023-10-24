use crate::pages::*;
use leptos::*;
use leptos_router::*;
use melt_ui::*;

#[component]
pub fn App() -> impl IntoView {
    let theme = create_rw_signal(Theme::light());
    provide_context(theme);
    view! {
        <Provider theme=theme.split().0>
            <Router base="/melt-ui">
                <Routes base="/melt-ui".to_string()>
                    <Route path="/" view=Home/>
                    <Route path="/components" view=ComponentsPage>
                        <Route path="/menu" view=MenuPage/>
                        <Route path="/slider" view=SliderPage/>
                        <Route path="/tabbar" view=TabbarPage/>
                        <Route path="/nav-bar" view=NavBarPage/>
                        <Route path="/input" view=InputPage/>
                        <Route path="/image" view=ImagePage/>
                        <Route path="/modal" view=ModalPage/>
                        <Route path="/button" view=ButtonPage/>
                        <Route path="/checkbox" view=CheckboxPage/>
                        <Route path="/toast" view=ToastPage/>
                        <Route path="/tabs" view=TabsPage/>
                        <Route path="/select" view=SelectPage/>
                        <Route path="/space" view=SpacePage/>
                        <Route path="/table" view=TablePage/>
                        <Route path="/color-picker" view=ColorPickerPage/>
                        <Route path="/alert" view=AlertPage/>
                        <Route path="/grid" view=GridPage/>
                        <Route path="/auto-complete" view=AutoCompletePage/>
                        <Route path="/avatar" view=AvatarPage/>
                        <Route path="/badge" view=BadgePage/>
                        <Route path="/card" view=CardPage/>
                        <Route path="/divider" view=DividerPage/>
                        <Route path="/input-number" view=InputNumberPage/>
                        <Route path="/icon" view=IconPage/>
                        <Route path="/message" view=MessagePage/>
                        <Route path="/radio" view=RadioPage/>
                        <Route path="/skeleton" view=SkeletonPage/>
                        <Route path="/switch" view=SwitchPage/>
                        <Route path="/tag" view=TagPage/>
                        <Route path="/upload" view=UploadPage/>
                    </Route>
                    <Route path="/mobile/tabbar" view=TabbarDemoPage/>
                    <Route path="/mobile/nav-bar" view=NavBarDemoPage/>
                    <Route path="/mobile/toast" view=ToastDemoPage/>
                </Routes>
            </Router>
        </Provider>
    }
}

#[component]
fn Provider(theme: ReadSignal<Theme>, children: Children) -> impl IntoView {
    view! {
        <ThemeProvider theme>
            <MessageProvider>
                {children()}
            </MessageProvider>
        </ThemeProvider>
    }
}

pub fn use_rw_theme() -> RwSignal<Theme> {
    expect_context::<RwSignal<Theme>>()
}

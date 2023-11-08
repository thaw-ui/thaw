use crate::pages::*;
use leptos::*;
use leptos_router::*;
use thaw::*;

#[component]
pub fn App() -> impl IntoView {
    fn use_query_value(key: &str) -> Option<String> {
        let href = window().location().href().ok()?;
        let url = Url::try_from(href.as_str()).ok()?;
        url.search_params.get(key).cloned()
    }
    let theme = use_query_value("theme").map_or_else(Theme::light, |name| {
        if name == "light" {
            Theme::light()
        } else if name == "dark" {
            Theme::dark()
        } else {
            Theme::light()
        }
    });
    let theme = create_rw_signal(theme);

    provide_context(theme);
    view! {
        <Provider theme>
            <TheRouter />
        </Provider>
    }
}

#[component]
fn TheRouter() -> impl IntoView {
    let loading_bar = use_loading_bar();
    let set_is_routing = SignalSetter::map(move |is_routing| {
        if is_routing {
            loading_bar.start();
        } else {
            loading_bar.finish();
        }
    });
    view! {
        <Router base="/thaw" set_is_routing>
            <Routes base="/thaw".to_string()>
                <Route path="/" view=Home/>
                <Route path="/guide" view=GuidePage>
                    <Route path="/installation" view=InstallationPage/>
                    <Route path="/usage" view=UsagePage/>
                </Route>
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
                    <Route path="/loading-bar" view=LoadingBarPage/>
                    <Route path="/breadcrumb" view=BreadcrumbPage/>
                </Route>
                <Route path="/mobile/tabbar" view=TabbarDemoPage/>
                <Route path="/mobile/nav-bar" view=NavBarDemoPage/>
                <Route path="/mobile/toast" view=ToastDemoPage/>
            </Routes>
        </Router>
    }
}

#[component]
fn Provider(theme: RwSignal<Theme>, children: Children) -> impl IntoView {
    view! {
        <ThemeProvider theme>
            <GlobalStyle />
            <MessageProvider>
                <LoadingBarProvider>
                    {children()}
                </LoadingBarProvider>
            </MessageProvider>
        </ThemeProvider>
    }
}

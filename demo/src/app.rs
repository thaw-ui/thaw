use crate::pages::*;
use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_router::*;
use thaw::*;

#[component]
pub fn App() -> impl IntoView {
    let is_routing = create_rw_signal(false);
    let set_is_routing = SignalSetter::map(move |is_routing_data| {
        is_routing.set(is_routing_data);
    });
    provide_meta_context();

    view! {
        <Router base="/thaw" set_is_routing>
            <TheProvider>
                <TheRouter is_routing/>
            </TheProvider>
        </Router>
    }
}

#[component]
fn TheRouter(is_routing: RwSignal<bool>) -> impl IntoView {
    let loading_bar = use_loading_bar();
    _ = is_routing.watch(move |is_routing| {
        if *is_routing {
            loading_bar.start();
        } else {
            loading_bar.finish();
        }
    });

    view! {
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
                <Route path="/spinner" view=SpinnerPage/>
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
                <Route path="/layout" view=LayoutPage/>
                <Route path="/progress" view=ProgressPage/>
                <Route path="/theme" view=ThemePage/>
                <Route path="/typography" view=TypographyPage/>
                <Route path="/calendar" view=CalendarPage/>
                <Route path="/time-picker" view=TimePickerPage/>
            </Route>
            <Route path="/mobile/tabbar" view=TabbarDemoPage/>
            <Route path="/mobile/nav-bar" view=NavBarDemoPage/>
            <Route path="/mobile/toast" view=ToastDemoPage/>
        </Routes>
    }
}

#[component]
fn TheProvider(children: Children) -> impl IntoView {
    fn use_query_value(key: &str) -> Option<String> {
        let query_map = use_query_map();
        query_map.with_untracked(|query| query.get(key).cloned())
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

    view! {
        <ThemeProvider theme>
            <GlobalStyle/>
            <MessageProvider>
                <LoadingBarProvider>{children()}</LoadingBarProvider>
            </MessageProvider>
        </ThemeProvider>
    }
}

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
                <Route path="/server-sider-rendering" view=ServerSiderRenderingPage/>
            </Route>
            <Route path="/components" view=ComponentsPage>
                <Route path="/tabbar" view=TabbarPage/>
                <Route path="/nav-bar" view=NavBarPage/>
                <Route path="/image" view=ImagePage/>
                <Route path="/button" view=ButtonPage/>
                <Route path="/checkbox" view=CheckboxPage/>
                <Route path="/toast" view=ToastPage/>
                <Route path="/color-picker" view=ColorPickerPage/>
                <Route path="/alert" view=AlertPage/>
                <Route path="/grid" view=GridPage/>
                <Route path="/auto-complete" view=AutoCompletePage/>
                <Route path="/avatar" view=AvatarPage/>
                <Route path="/badge" view=BadgePage/>
                <Route path="/card" view=CardPage/>
                <Route path="/divider" view=DividerPage/>
                <Route path="/icon" view=IconPage/>
                <Route path="/breadcrumb" view=BreadcrumbPage/>
                <Route path="/calendar" view=CalendarPage/>
                <Route path="/date-picker" view=DatePickerPage/>
                <Route path="/input" view=InputMdPage/>
                <Route path="/input-number" view=InputNumberMdPage/>
                <Route path="/layout" view=LayoutMdPage/>
                <Route path="/loading-bar" view=LoadingBarMdPage/>
                <Route path="/menu" view=MenuMdPage/>
                <Route path="/message" view=MessageMdPage/>
                <Route path="/modal" view=ModalMdPage/>
                <Route path="/popover" view=PopoverMdPage/>
                <Route path="/progress" view=ProgressMdPage/>
                <Route path="/radio" view=RadioMdPage/>
                <Route path="/select" view=SelectMdPage/>
                <Route path="/skeleton" view=SkeletonMdPage/>
                <Route path="/slider" view=SliderMdPage/>
                <Route path="/space" view=SpaceMdPage/>
                <Route path="/spinner" view=SpinnerMdPage/>
                <Route path="/switch" view=SwitchMdPage/>
                <Route path="/table" view=TableMdPage/>
                <Route path="/tabs" view=TabsMdPage/>
                <Route path="/tag" view=TagMdPage/>
                <Route path="/theme" view=ThemeMdPage/>
                <Route path="/time-picker" view=TimePickerMdPage/>
                <Route path="/typography" view=TypographyMdPage/>
                <Route path="/upload" view=UploadMdPage/>
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

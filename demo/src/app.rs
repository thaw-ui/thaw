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
        <Router set_is_routing>
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
        <Routes>
            <Route path="/" view=Home/>
            <Route path="/guide" view=ComponentsPage>
                <Route path="/installation" view=InstallationMdPage/>
                <Route path="/usage" view=UsageMdPage/>
                <Route path="/server-sider-rendering" view=ServerSiderRenderingMdPage/>
                <Route path="/development/guide" view=DevelopmentGuideMdPage/>
                <Route path="/development/components" view=DevelopmentComponentsMdPage/>
            </Route>
            <Route path="/components" view=ComponentsPage>
                // <Route path="/tabbar" view=TabbarPage/>
                // <Route path="/nav-bar" view=NavBarPage/>
                // <Route path="/toast" view=ToastPage/>
                <Route path="/accordion" view=AccordionMdPage/>
                <Route path="/anchor" view=AnchorMdPage/>
                <Route path="/auto-complete" view=AutoCompleteMdPage/>
                <Route path="/avatar" view=AvatarMdPage/>
                <Route path="/back-top" view=BackTopMdPage/>
                <Route path="/badge" view=BadgeMdPage/>
                <Route path="/breadcrumb" view=BreadcrumbMdPage/>
                <Route path="/button" view=ButtonMdPage/>
                <Route path="/calendar" view=CalendarMdPage/>
                <Route path="/card" view=CardMdPage/>
                <Route path="/checkbox" view=CheckboxMdPage/>
                <Route path="/color-picker" view=ColorPickerMdPage/>
                <Route path="/combobox" view=ComboboxMdPage/>
                <Route path="/config-provider" view=ConfigProviderMdPage/>
                <Route path="/date-picker" view=DatePickerMdPage/>
                <Route path="/divider" view=DividerMdPage/>
                <Route path="/drawer" view=DrawerMdPage/>
                <Route path="/grid" view=GridMdPage/>
                <Route path="/icon" view=IconMdPage/>
                <Route path="/image" view=ImageMdPage/>
                <Route path="/input" view=InputMdPage/>
                <Route path="/layout" view=LayoutMdPage/>
                <Route path="/loading-bar" view=LoadingBarMdPage/>
                // <Route path="/message" view=MessageMdPage/>
                <Route path="/message-bar" view=MessageBarMdPage/>
                <Route path="/modal" view=ModalMdPage/>
                <Route path="/popover" view=PopoverMdPage/>
                <Route path="/progress" view=ProgressMdPage/>
                <Route path="/radio" view=RadioMdPage/>
                <Route path="/scrollbar" view=ScrollbarMdPage/>
                // <Route path="/select" view=SelectMdPage/>
                <Route path="/skeleton" view=SkeletonMdPage/>
                <Route path="/slider" view=SliderMdPage/>
                <Route path="/space" view=SpaceMdPage/>
                <Route path="/spin-button" view=SpinButtonMdPage/>
                <Route path="/spinner" view=SpinnerMdPage/>
                <Route path="/switch" view=SwitchMdPage/>
                <Route path="/tab-list" view=TabListMdPage/>
                <Route path="/table" view=TableMdPage/>
                <Route path="/tag" view=TagMdPage/>
                <Route path="/text" view=TextMdPage/>
                <Route path="/textarea" view=TextareaMdPage/>
                <Route path="/time-picker" view=TimePickerMdPage/>
                <Route path="/upload" view=UploadMdPage/>
            </Route>
            // <Route path="/mobile/tabbar" view=TabbarDemoPage/>
            // <Route path="/mobile/nav-bar" view=NavBarDemoPage/>
            // <Route path="/mobile/toast" view=ToastDemoPage/>
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
        <ConfigProvider theme>
            <ThemeProvider theme>
                // <MessageProvider>
                    <LoadingBarProvider>{children()}</LoadingBarProvider>
                // </MessageProvider>
            </ThemeProvider>
        </ConfigProvider>
    }
}

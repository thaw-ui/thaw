use crate::pages::*;
use leptos::{prelude::*, reactive_graph::wrappers::write::SignalSetter};
use leptos_meta::provide_meta_context;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    StaticSegment,
};
// use leptos_use::{
//     storage::use_local_storage,
//     utils::{FromToStringCodec, StringCodec},
// };
use thaw::*;

#[component]
pub fn App() -> impl IntoView {
    let is_routing = RwSignal::new(false);
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
    let loading_bar = LoadingBarInjection::expect_use();
    _ = is_routing.watch(move |is_routing| {
        if *is_routing {
            loading_bar.start();
        } else {
            loading_bar.finish();
        }
    });

    view! {
        <Routes fallback=|| "404">
            <Route path=StaticSegment("") view=Home/>
            <ParentRoute path=StaticSegment("guide") view=ComponentsPage>
                <Route path=StaticSegment("installation") view=InstallationMdPage/>
                <Route path=StaticSegment("server-sider-rendering") view=ServerSiderRenderingMdPage/>
                <Route path=(StaticSegment("development"), StaticSegment("components")) view=DevelopmentComponentsMdPage/>
            </ParentRoute>
            <ParentRoute path=StaticSegment("components") view=ComponentsPage>

            //     // <Route path="/tabbar" view=TabbarPage/>
            //     // <Route path="/nav-bar" view=NavBarPage/>
            //     // <Route path="/toast" view=ToastPage/>
                {
                    view! {
                        <Route path=StaticSegment("accordion") view=AccordionMdPage/>
                        <Route path=StaticSegment("anchor") view=AnchorMdPage/>
                        <Route path=StaticSegment("auto-complete") view=AutoCompleteMdPage/>
                        <Route path=StaticSegment("avatar") view=AvatarMdPage/>
                        <Route path=StaticSegment("back-top") view=BackTopMdPage/>
                        <Route path=StaticSegment("badge") view=BadgeMdPage/>
                        <Route path=StaticSegment("breadcrumb") view=BreadcrumbMdPage/>
                        <Route path=StaticSegment("button") view=ButtonMdPage/>
                        <Route path=StaticSegment("calendar") view=CalendarMdPage/>
                        <Route path=StaticSegment("card") view=CardMdPage/>
                        <Route path=StaticSegment("checkbox") view=CheckboxMdPage/>
                        <Route path=StaticSegment("color-picker") view=ColorPickerMdPage/>
                        <Route path=StaticSegment("combobox") view=ComboboxMdPage/>
                        <Route path=StaticSegment("config-provider") view=ConfigProviderMdPage/>
                    }
                }
                {
                    view! {
                        <Route path=StaticSegment("date-picker") view=DatePickerMdPage/>
                        <Route path=StaticSegment("dialog") view=DialogMdPage/>
                        <Route path=StaticSegment("divider") view=DividerMdPage/>
                        <Route path=StaticSegment("drawer") view=DrawerMdPage/>
                        <Route path=StaticSegment("menu") view=MenuMdPage/>
                        <Route path=StaticSegment("grid") view=GridMdPage/>
                        <Route path=StaticSegment("icon") view=IconMdPage/>
                        <Route path=StaticSegment("image") view=ImageMdPage/>
                        <Route path=StaticSegment("input") view=InputMdPage/>
                        <Route path=StaticSegment("layout") view=LayoutMdPage/>
                        <Route path=StaticSegment("loading-bar") view=LoadingBarMdPage/>
                        <Route path=StaticSegment("message-bar") view=MessageBarMdPage/>
                        <Route path=StaticSegment("popover") view=PopoverMdPage/>
                        <Route path=StaticSegment("progress-bar") view=ProgressBarMdPage/>
                        <Route path=StaticSegment("radio") view=RadioMdPage/>
                    }
                }
                {
                    view! {
                        <Route path=StaticSegment("scrollbar") view=ScrollbarMdPage/>
                        <Route path=StaticSegment("skeleton") view=SkeletonMdPage/>
                        <Route path=StaticSegment("slider") view=SliderMdPage/>
                        <Route path=StaticSegment("space") view=SpaceMdPage/>
                        <Route path=StaticSegment("spin-button") view=SpinButtonMdPage/>
                        <Route path=StaticSegment("spinner") view=SpinnerMdPage/>
                        <Route path=StaticSegment("switch") view=SwitchMdPage/>
                    }
                }
                <Route path=StaticSegment("tab-list") view=TabListMdPage/>
                <Route path=StaticSegment("table") view=TableMdPage/>
                <Route path=StaticSegment("tag") view=TagMdPage/>
                <Route path=StaticSegment("text") view=TextMdPage/>
                <Route path=StaticSegment("textarea") view=TextareaMdPage/>
                <Route path=StaticSegment("time-picker") view=TimePickerMdPage/>
                <Route path=StaticSegment("toast") view=ToastMdPage />
                <Route path=StaticSegment("upload") view=UploadMdPage/>
            </ParentRoute>
            // <Route path="/mobile/tabbar" view=TabbarDemoPage/>
            // <Route path="/mobile/nav-bar" view=NavBarDemoPage/>
            // <Route path="/mobile/toast" view=ToastDemoPage/>
        </Routes>
    }
}

#[component]
fn TheProvider(children: Children) -> impl IntoView {
    // let (read_theme, _, _) = use_local_storage::<String, FromToStringCodec>("theme");
    // let theme = RwSignal::new(Theme::from(read_theme.get_untracked()));

    view! {
        <ConfigProvider>
            <ToasterProvider>
                <LoadingBarProvider>
                    {children()}
                </LoadingBarProvider>
            </ToasterProvider>
        </ConfigProvider>
    }
}

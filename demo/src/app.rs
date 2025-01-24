use crate::pages::*;
use leptos::{prelude::*, reactive::wrappers::write::SignalSetter};
use leptos_meta::provide_meta_context;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path,
};
// use leptos_use::{
//     storage::use_local_storage,
//     utils::{FromToStringCodec, StringCodec},
// };
use thaw::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    // let (read_theme, _, _) = use_local_storage::<String, FromToStringCodec>("theme");
    // let theme = RwSignal::new(Theme::from(read_theme.get_untracked()));
    let dir = RwSignal::new(ConfigDirection::Ltr);

    view! {
        <ConfigProvider dir=dir>
            <ToasterProvider>
                <LoadingBarProvider>
                    <TheRouter />
                </LoadingBarProvider>
            </ToasterProvider>
        </ConfigProvider>
    }
}

#[component]
fn TheRouter() -> impl IntoView {
    let loading_bar = LoadingBarInjection::expect_context();
    let is_routing = RwSignal::new(false);
    let set_is_routing = SignalSetter::map(move |is_routing_data| {
        is_routing.set(is_routing_data);
    });

    Effect::watch(
        move || is_routing.get(),
        move |is_routing, _, _| {
            if *is_routing {
                loading_bar.start();
            } else {
                loading_bar.finish();
            }
        },
        false,
    );

    view! {
        <Router set_is_routing>
            <Routes fallback=|| "404">
                <Route path=path!("/") view=Home />
                <ParentRoute path=path!("/guide") view=ComponentsPage>
                    <Route path=path!("/installation") view=InstallationMdPage />
                    <Route path=path!("/server-sider-rendering") view=ServerSiderRenderingMdPage />
                    <Route path=path!("/development/components") view=DevelopmentComponentsMdPage />
                    <Route path=path!("/customize-theme") view=CustomizeThemeMdPage />
                    <Route path=path!("/FAQs") view=FAQsMdPage />
                </ParentRoute>
                <ParentRoute path=path!("/components") view=ComponentsPage>
                    <Route path=path!("/accordion") view=AccordionMdPage />
                    <Route path=path!("/anchor") view=AnchorMdPage />
                    <Route path=path!("/auto-complete") view=AutoCompleteMdPage />
                    <Route path=path!("/avatar") view=AvatarMdPage />
                    <Route path=path!("/back-top") view=BackTopMdPage />
                    <Route path=path!("/badge") view=BadgeMdPage />
                    <Route path=path!("/breadcrumb") view=BreadcrumbMdPage />
                    <Route path=path!("/button") view=ButtonMdPage />
                    <Route path=path!("/calendar") view=CalendarMdPage />
                    <Route path=path!("/card") view=CardMdPage />
                    <Route path=path!("/card-header") view=CardHeaderMdPage />
                    <Route path=path!("/checkbox") view=CheckboxMdPage />
                    <Route path=path!("/color-picker") view=ColorPickerMdPage />
                    <Route path=path!("/combobox") view=ComboboxMdPage />
                    <Route path=path!("/config-provider") view=ConfigProviderMdPage />
                    <Route path=path!("/date-picker") view=DatePickerMdPage />
                    <Route path=path!("/dialog") view=DialogMdPage />
                    <Route path=path!("/divider") view=DividerMdPage />
                    <Route path=path!("/drawer") view=DrawerMdPage />
                    <Route path=path!("/field") view=FieldMdPage />
                    <Route path=path!("/flex") view=FlexMdPage />
                    <Route path=path!("/menu") view=MenuMdPage />
                    <Route path=path!("/grid") view=GridMdPage />
                    <Route path=path!("/icon") view=IconMdPage />
                    <Route path=path!("/image") view=ImageMdPage />
                    <Route path=path!("/info-label") view=InfoLabelMdPage />
                    <Route path=path!("/input") view=InputMdPage />
                    <Route path=path!("/label") view=LabelMdPage />
                    <Route path=path!("/layout") view=LayoutMdPage />
                    <Route path=path!("/link") view=LinkMdPage />
                    <Route path=path!("/loading-bar") view=LoadingBarMdPage />
                    <Route path=path!("/message-bar") view=MessageBarMdPage />
                    <Route path=path!("/nav") view=NavMdPage />
                    <Route path=path!("/pagination") view=PaginationMdPage />
                    <Route path=path!("/popover") view=PopoverMdPage />
                    <Route path=path!("/progress-bar") view=ProgressBarMdPage />
                    <Route path=path!("/radio") view=RadioMdPage />
                    <Route path=path!("/rating") view=RatingMdPage />
                    <Route path=path!("/scrollbar") view=ScrollbarMdPage />
                    <Route path=path!("/select") view=SelectMdPage />
                    <Route path=path!("/skeleton") view=SkeletonMdPage />
                    <Route path=path!("/slider") view=SliderMdPage />
                    <Route path=path!("/range-slider") view=RangeSliderMdPage />
                    <Route path=path!("/space") view=SpaceMdPage />
                    <Route path=path!("/spin-button") view=SpinButtonMdPage />
                    <Route path=path!("/spinner") view=SpinnerMdPage />
                    <Route path=path!("/switch") view=SwitchMdPage />
                    <Route path=path!("/tab-list") view=TabListMdPage />
                    <Route path=path!("/table") view=TableMdPage />
                    <Route path=path!("/tag") view=TagMdPage />
                    <Route path=path!("/tag-group") view=TagGroupMdPage />
                    <Route path=path!("/tag-picker") view=TagPickerMdPage />
                    <Route path=path!("/text") view=TextMdPage />
                    <Route path=path!("/textarea") view=TextareaMdPage />
                    <Route path=path!("/time-picker") view=TimePickerMdPage />
                    <Route path=path!("/toast") view=ToastMdPage />
                    <Route path=path!("/tooltip") view=TooltipMdPage />
                    <Route path=path!("/upload") view=UploadMdPage />
                </ParentRoute>
            </Routes>
        </Router>
    }
}

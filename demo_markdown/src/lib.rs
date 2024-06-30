mod markdown;

use crate::markdown::parse_markdown;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::ItemFn;

macro_rules! file_path {
    ($($key:expr => $value:expr),*) => {
        {
            vec![
                $(
                    ($key, include_str!($value)),
                )*
            ]
        }
    }
}

#[proc_macro]
pub fn include_md(_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let file_list = file_path! {
        "DevelopmentComponentsMdPage" => "../docs/_guide/development/components.md",
        "DevelopmentGuideMdPage" => "../docs/_guide/development/guide.md",
        "InstallationMdPage" => "../docs/_guide/installation.md",
        "ServerSiderRenderingMdPage" => "../docs/_guide/server_sider_rendering.md",
        "UsageMdPage" => "../docs/_guide/usage.md",
        // "NavBarMdPage" => "../docs/_mobile/nav_bar/mod.md",
        // "TabbarMdPage" => "../docs/_mobile/tabbar/mod.md",
        // "ToastMdPage" => "../docs/_mobile/toast/mod.md",
        "AccordionMdPage" => "../docs/accordion/mod.md",
        // "AlertMdPage" => "../docs/alert/mod.md",
        "AnchorMdPage" => "../docs/anchor/mod.md",
        "AutoCompleteMdPage" => "../docs/auto_complete/mod.md",
        "AvatarMdPage" => "../docs/avatar/mod.md",
        "BackTopMdPage" => "../docs/back_top/mod.md",
        "BadgeMdPage" => "../docs/badge/mod.md",
        "BreadcrumbMdPage" => "../docs/breadcrumb/mod.md",
        "ButtonMdPage" => "../docs/button/mod.md",
        "CalendarMdPage" => "../docs/calendar/mod.md",
        "CardMdPage" => "../docs/card/mod.md",
        "CheckboxMdPage" => "../docs/checkbox/mod.md",
        "ColorPickerMdPage" => "../docs/color_picker/mod.md",
        "ComboboxMdPage" => "../docs/combobox/mod.md",
        "ConfigProviderMdPage" => "../docs/config_provider/mod.md",
        "DatePickerMdPage" => "../docs/date_picker/mod.md",
        "DialogMdPage" => "../docs/dialog/mod.md",
        "DividerMdPage" => "../docs/divider/mod.md",
        "DrawerMdPage" => "../docs/drawer/mod.md",
        "GridMdPage" => "../docs/grid/mod.md",
        "IconMdPage" => "../docs/icon/mod.md",
        "ImageMdPage" => "../docs/image/mod.md",
        "InputMdPage" => "../docs/input/mod.md",
        "LayoutMdPage" => "../docs/layout/mod.md",
        "LoadingBarMdPage" => "../docs/loading_bar/mod.md",
        // "MessageMdPage" => "../docs/message/mod.md",
        "MessageBarMdPage" => "../docs/message_bar/mod.md",
        "PopoverMdPage" => "../docs/popover/mod.md",
        "ProgressBarMdPage" => "../docs/progress_bar/mod.md",
        "RadioMdPage" => "../docs/radio/mod.md",
        "ScrollbarMdPage" => "../docs/scrollbar/mod.md",
        // "SelectMdPage" => "../docs/select/mod.md",
        "SkeletonMdPage" => "../docs/skeleton/mod.md",
        "SliderMdPage" => "../docs/slider/mod.md",
        "SpaceMdPage" => "../docs/space/mod.md",
        "SpinButtonMdPage" => "../docs/spin_button/mod.md",
        "SpinnerMdPage" => "../docs/spinner/mod.md",
        "SwitchMdPage" => "../docs/switch/mod.md",
        "TabListMdPage" => "../docs/tab_list/mod.md",
        "TableMdPage" => "../docs/table/mod.md",
        "TagMdPage" => "../docs/tag/mod.md",
        "TextareaMdPage" => "../docs/textarea/mod.md",
        "TimePickerMdPage" => "../docs/time_picker/mod.md",
        "TextMdPage" => "../docs/text/mod.md",
        "UploadMdPage" => "../docs/upload/mod.md"
    };

    let mut fn_list = vec![];

    for (fn_name, file_str) in file_list {
        let fn_name = Ident::new(fn_name, Span::call_site());

        let (body, demos, toc) = match parse_markdown(file_str) {
            Ok(body) => body,
            Err(err) => {
                return quote!(compile_error!(#err)).into();
            }
        };

        let toc = {
            let links = toc
                .into_iter()
                .map(|h| format!(r##"<AnchorLink href="#{}" title="{}" />"##, h.0, h.1))
                .collect::<Vec<_>>()
                .join(" ");
            let toc = format!(
                r##"#[component] fn Toc() -> impl IntoView {{ view! {{ <Anchor offset_target=".doc-content">{}</Anchor> }} }}"##,
                links
            );
            syn::parse_str::<ItemFn>(&toc)
                .expect(&format!("Cannot be resolved as a function: \n {toc}"))
        };

        let demos: Vec<ItemFn> = demos
            .into_iter()
            .enumerate()
            .map(|(index, demo)| {
                format!(
                    "#[component] fn Demo{}() -> impl IntoView {{ {} }}",
                    index + 1,
                    demo
                )
            })
            .map(|demo| {
                syn::parse_str::<ItemFn>(&demo)
                    .unwrap_or_else(|_| panic!("Cannot be resolved as a function: \n {demo}"))
            })
            .collect();

        fn_list.push(quote! {
            #[component]
            pub fn #fn_name() -> impl IntoView {
                #(#demos)*

                #toc

                view! {
                    <div class="demo-components__component">
                        #body
                    </div>
                    <div class="demo-components__toc">
                        <Toc />
                    </div>
                }
            }
        });
    }

    quote! {
        #(#fn_list)*
    }
    .into()
}

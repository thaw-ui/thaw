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
        "InstallationMdPage" => "../docs/_guide/installation.md",
        "ServerSiderRenderingMdPage" => "../docs/_guide/server_sider_rendering.md",
        "AccordionMdPage" => "../../thaw/src/accordion/docs/mod.md",
        "AnchorMdPage" => "../../thaw/src/anchor/docs/mod.md",
        "AutoCompleteMdPage" => "../../thaw/src/auto_complete/docs/mod.md",
        "AvatarMdPage" => "../../thaw/src/avatar/docs/mod.md",
        "BackTopMdPage" => "../../thaw/src/back_top/docs/mod.md",
        "BadgeMdPage" => "../../thaw/src/badge/docs/mod.md",
        "BreadcrumbMdPage" => "../../thaw/src/breadcrumb/docs/mod.md",
        "ButtonMdPage" => "../../thaw/src/button/docs/mod.md",
        "CalendarMdPage" => "../../thaw/src/calendar/docs/mod.md",
        "CardMdPage" => "../../thaw/src/card/docs/mod.md",
        "CheckboxMdPage" => "../../thaw/src/checkbox/docs/mod.md",
        "ColorPickerMdPage" => "../../thaw/src/color_picker/docs/mod.md",
        "ComboboxMdPage" => "../../thaw/src/combobox/docs/mod.md",
        "ConfigProviderMdPage" => "../../thaw/src/config_provider/docs/mod.md",
        "DatePickerMdPage" => "../../thaw/src/date_picker/docs/mod.md",
        "DialogMdPage" => "../../thaw/src/dialog/docs/mod.md",
        "DividerMdPage" => "../../thaw/src/divider/docs/mod.md",
        "DrawerMdPage" => "../../thaw/src/drawer/docs/mod.md",
        "FieldMdPage" => "../../thaw/src/field/docs/mod.md",
        "GridMdPage" => "../../thaw/src/grid/docs/mod.md",
        "IconMdPage" => "../../thaw/src/icon/docs/mod.md",
        "ImageMdPage" => "../../thaw/src/image/docs/mod.md",
        "InputMdPage" => "../../thaw/src/input/docs/mod.md",
        "LayoutMdPage" => "../../thaw/src/layout/docs/mod.md",
        "LoadingBarMdPage" => "../../thaw/src/loading_bar/docs/mod.md",
        "MenuMdPage" => "../../thaw/src/menu/docs/mod.md",
        "MessageBarMdPage" => "../../thaw/src/message_bar/docs/mod.md",
        "NavMdPage" => "../../thaw/src/nav/docs/mod.md",
        "PaginationMdPage" => "../../thaw/src/pagination/docs/mod.md",
        "PopoverMdPage" => "../../thaw/src/popover/docs/mod.md",
        "ProgressBarMdPage" => "../../thaw/src/progress_bar/docs/mod.md",
        "RadioMdPage" => "../../thaw/src/radio/docs/mod.md",
        "ScrollbarMdPage" => "../../thaw/src/scrollbar/docs/mod.md",
        "SkeletonMdPage" => "../../thaw/src/skeleton/docs/mod.md",
        "SliderMdPage" => "../../thaw/src/slider/docs/mod.md",
        "SpaceMdPage" => "../../thaw/src/space/docs/mod.md",
        "SpinButtonMdPage" => "../../thaw/src/spin_button/docs/mod.md",
        "SpinnerMdPage" => "../../thaw/src/spinner/docs/mod.md",
        "SwitchMdPage" => "../../thaw/src/switch/docs/mod.md",
        "TabListMdPage" => "../../thaw/src/tab_list/docs/mod.md",
        "TableMdPage" => "../../thaw/src/table/docs/mod.md",
        "TagMdPage" => "../../thaw/src/tag/docs/mod.md",
        "TextMdPage" => "../../thaw/src/text/docs/mod.md",
        "TextareaMdPage" => "../../thaw/src/textarea/docs/mod.md",
        "TimePickerMdPage" => "../../thaw/src/time_picker/docs/mod.md",
        "ToastMdPage" => "../../thaw/src/toast/docs/mod.md",
        "TooltipMdPage" => "../../thaw/src/tooltip/docs/mod.md",
        "UploadMdPage" => "../../thaw/src/upload/docs/mod.md"
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

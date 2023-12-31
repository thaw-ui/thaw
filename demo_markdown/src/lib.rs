mod markdown;

use crate::markdown::parse_markdown;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::ItemFn;

#[proc_macro]
pub fn include_md(_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let file_list = vec![
        ("SpaceMdPage", include_str!("../docs/space/mod.md")),
        ("SpinnerMdPage", include_str!("../docs/spinner/mod.md")),
        ("SwitchMdPage", include_str!("../docs/switch/mod.md")),
        ("TableMdPage", include_str!("../docs/table/mod.md")),
        ("TabsMdPage", include_str!("../docs/tabs/mod.md")),
        ("TagMdPage", include_str!("../docs/tag/mod.md")),
        ("ThemeMdPage", include_str!("../docs/theme/mod.md")),
        (
            "TimePickerMdPage",
            include_str!("../docs/time_picker/mod.md"),
        ),
        (
            "TypographyMdPage",
            include_str!("../docs/typography/mod.md"),
        ),
        ("UploadMdPage", include_str!("../docs/upload/mod.md")),
    ];

    let mut fn_list = vec![];

    for (fn_name, file_str) in file_list {
        let fn_name = Ident::new(fn_name, Span::call_site());

        let (body, demos) = match parse_markdown(file_str) {
            Ok(body) => body,
            Err(err) => {
                return quote!(compile_error!(#err)).into();
            }
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
                    .expect(&format!("Cannot be resolved as a function: \n {demo}"))
            })
            .collect();

        fn_list.push(quote! {
            #[component]
            pub fn #fn_name() -> impl IntoView {
                #(#demos)*

                view! {
                    <div style="width: 896px; margin: 0 auto;">
                        #body
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

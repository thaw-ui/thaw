mod key_value;
mod markdown;

use crate::{
    key_value::{KeyValue, KeyValueList},
    markdown::parse_markdown,
};
use quote::quote;
use syn::ItemFn;

#[proc_macro]
pub fn include_md(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let key_value_list = match syn::parse::<KeyValueList>(token_stream) {
        Ok(key_value_list) => key_value_list,
        Err(err) => {
            return err.to_compile_error().into();
        }
    };

    let mut fn_list = vec![];

    for key_value in key_value_list.0 {
        let KeyValue {
            key: fn_name,
            value: file_path,
        } = key_value;

        let (body, demos) = match parse_markdown(file_path) {
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
            .map(|demo| syn::parse_str::<ItemFn>(&demo).unwrap())
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

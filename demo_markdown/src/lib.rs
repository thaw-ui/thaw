mod key_value;
mod markdown;

use crate::{key_value::KeyValue, markdown::parse_markdown};
use key_value::KeyValueList;
use quote::quote;

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

        let body = match parse_markdown(file_path) {
            Ok(body) => body,
            Err(err) => {
                return quote!(compile_error!(#err)).into();
            }
        };

        fn_list.push(quote! {
            #[component]
            pub fn #fn_name() -> impl IntoView {
                view! {
                    #body
                }
            }
        });
    }

    quote! {
        #(#fn_list)*
    }
    .into()
}

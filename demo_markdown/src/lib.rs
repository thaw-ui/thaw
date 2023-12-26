use quote::quote;

#[proc_macro]
pub fn include_md(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let file_path = token_stream.to_string();
    let file_path = file_path.trim_matches('"');
    let md = std::fs::read_to_string(file_path);
    let md = match md {
        Ok(md) => md,
        Err(err) => {
            let err = err.to_string();
            return proc_macro::TokenStream::from(quote! { compile_error!(#err) });
        }
    };

    let expanded = quote! {
        ""
    };
    proc_macro::TokenStream::from(expanded)
}

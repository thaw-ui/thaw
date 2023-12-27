use litrs::StringLit;
use proc_macro2::*;
use quote::quote;

#[proc_macro]
pub fn include_md(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let token_stream = TokenStream::from(token_stream).into_iter();

    let expanded =
        include_md_inner(token_stream).unwrap_or_else(|err| quote! { compile_error!(#err) });

    proc_macro::TokenStream::from(expanded)
}

fn include_md_inner(token_stream: impl Iterator<Item = TokenTree>) -> Result<TokenStream, String> {
    let (file_path, fn_name) = parse_file_path_and_fn_name(token_stream)?;
    let md = std::fs::read_to_string(file_path).map_err(|err| err.to_string())?;

    let expanded = quote! {
        compile_error!(#fn_name)
    };
    Ok(expanded)
}

fn parse_file_path_and_fn_name(
    mut token_stream: impl Iterator<Item = TokenTree>,
) -> Result<(String, String), String> {
    let first = token_stream.next();
    let Some(TokenTree::Literal(code)) = first else {
        return Err("Expected only a string literal".to_string());
    };
    let file_path =
        StringLit::try_from(code).map_err(|err| format!("Expected a string literal: {}", err))?;

    let second = token_stream.next();
    let Some(TokenTree::Punct(punct)) = second else {
        return Err("You must put a comma `,` after the code string".to_string());
    };

    if punct.as_char() != ',' {
        return Err("You must put a comma `,` after the code string".to_string());
    }

    let third = token_stream.next();
    let Some(TokenTree::Literal(third)) = third else {
        return Err("Expected only a string literal".to_string());
    };
    let fn_name =
        StringLit::try_from(third).map_err(|err| format!("Expected a string literal: {}", err))?;

    Ok((file_path.value().to_string(), fn_name.value().to_string()))
}

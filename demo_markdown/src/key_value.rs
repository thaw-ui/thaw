use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
    Ident, Lit, Token,
};

pub struct KeyValue {
    pub key: Ident,
    pub value: String,
}

impl Parse for KeyValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;
        let _arrow: Token![=>] = input.parse()?;
        let value: Lit = input.parse()?;
        let value = match value {
            Lit::Str(lit_str) => lit_str.value(),
            _ => return Err(syn::Error::new(value.span(), "")),
        };
        Ok(Self { key, value })
    }
}

pub struct KeyValueList(pub Vec<KeyValue>);

impl Parse for KeyValueList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let punctuated: Punctuated<KeyValue, Comma> = Punctuated::parse_terminated(input)?;
        Ok(KeyValueList(punctuated.into_iter().collect()))
    }
}

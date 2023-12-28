use comrak::nodes::NodeCodeBlock;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::sync::OnceLock;
use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet};

pub fn to_tokens(code_block: &NodeCodeBlock, demos: &mut Vec<String>) -> TokenStream {
    let langs: Vec<&str> = code_block.info.split_ascii_whitespace().collect();
    if langs.iter().any(|lang| lang == &"demo") {
        demos.push(code_block.literal.clone());

        let demo = Ident::new(&format!("Demo{}", demos.len()), Span::call_site());

        let literal = if let Some(lang) = langs.iter().find(|lang| lang != &&"demo") {
            highlight_to_html(&code_block.literal, lang, "Solarized (light)")
        } else {
            code_block.literal.clone()
        };

        quote! {
            <Demo>
                <#demo />
                <DemoCode slot>
                    #literal
                </DemoCode>
            </Demo>
        }
    } else {
        quote!("CodeBlock todo!!!")
    }
}

static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();

fn highlight_to_html(text: &str, syntax: &str, theme: &str) -> String {
    let syntax_set = SYNTAX_SET.get_or_init(|| SyntaxSet::load_defaults_newlines());
    let Some(syntax) = syntax_set.find_syntax_by_name(syntax) else {
        // TODO
        return format!("{syntax}-lang {}", text.to_string());
    };

    let theme = &THEME_SET.get_or_init(|| ThemeSet::load_defaults()).themes[theme];

    let Ok(html) = highlighted_html_for_string(text, syntax_set, syntax, theme) else {
        return text.to_string();
    };

    html
}

use comrak::nodes::NodeCodeBlock;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::sync::OnceLock;
use syntect::{
    html::{ClassStyle, ClassedHTMLGenerator},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

pub fn to_tokens(code_block: &NodeCodeBlock, demos: &mut Vec<String>) -> TokenStream {
    let langs: Vec<&str> = code_block.info.split_ascii_whitespace().collect();
    if langs.iter().any(|lang| lang == &"demo") {
        demos.push(code_block.literal.clone());

        let demo = Ident::new(&format!("Demo{}", demos.len()), Span::call_site());
        let mut is_highlight = true;
        let literal = langs
            .iter()
            .find(|lang| lang != &&"demo")
            .and_then(|lang| highlight_to_html(&code_block.literal, lang))
            .unwrap_or_else(|| {
                is_highlight = false;
                code_block.literal.clone()
            });

        quote! {
            <Demo>
                <#demo />
                <DemoCode slot is_highlight=#is_highlight>
                    #literal
                </DemoCode>
            </Demo>
        }
    } else {
        let mut is_highlight = true;
        let literal = langs
            .first()
            .and_then(|lang| highlight_to_html(&code_block.literal, lang))
            .unwrap_or_else(|| {
                is_highlight = false;
                code_block.literal.clone()
            });
        quote! {
            <Demo>
                <DemoCode slot is_highlight=#is_highlight>
                    #literal
                </DemoCode>
            </Demo>
        }
    }
}

static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();

fn highlight_to_html(text: &str, syntax: &str) -> Option<String> {
    let syntax_set = SYNTAX_SET.get_or_init(|| SyntaxSet::load_defaults_newlines());
    let Some(syntax) = syntax_set.find_syntax_by_token(syntax) else {
        return None;
    };

    let mut html_generator = ClassedHTMLGenerator::new_with_class_style(
        syntax,
        syntax_set,
        ClassStyle::SpacedPrefixed { prefix: "syntect-" },
    );

    for line in LinesWithEndings::from(text) {
        html_generator
            .parse_html_for_line_which_includes_newline(line)
            .expect(line);
    }

    Some(html_generator.finalize())
}

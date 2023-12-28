use comrak::{
    nodes::{AstNode, NodeValue},
    parse_document, Arena,
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn parse_markdown(file_path: String) -> Result<TokenStream, String> {
    let md_text = std::fs::read_to_string(file_path.clone()).map_err(|err| err.to_string())?;

    let arena = Arena::new();
    let root = parse_document(&arena, &md_text, &comrak::Options::default());

    Ok(iter_nodes(root))
}

fn iter_nodes<'a>(node: &'a AstNode<'a>) -> TokenStream {
    let mut children = vec![];
    for c in node.children() {
        children.push(iter_nodes(c));
    }
    match &node.data.borrow().value {
        NodeValue::Document => quote!("Document todo!!!"),
        NodeValue::FrontMatter(_) => quote!("FrontMatter todo!!!"),
        NodeValue::BlockQuote => quote!("BlockQuote todo!!!"),
        NodeValue::List(_) => quote!("List todo!!!"),
        NodeValue::Item(_) => quote!("Item todo!!!"),
        NodeValue::DescriptionList => quote!("DescriptionList todo!!!"),
        NodeValue::DescriptionItem(_) => quote!("DescriptionItem todo!!!"),
        NodeValue::DescriptionTerm => quote!("DescriptionTerm todo!!!"),
        NodeValue::DescriptionDetails => quote!("DescriptionDetails todo!!!"),
        NodeValue::CodeBlock(_) => quote!("CodeBlock todo!!!"),
        NodeValue::HtmlBlock(_) => quote!("HtmlBlock todo!!!"),
        NodeValue::Paragraph => quote!(
            <p>
                #(#children)*
            </p>
        ),
        NodeValue::Heading(h) => {
            let h = format!("h{}", h.level);
            quote!(
                <#h>
                    #(#children)*
                </#h>
            )
        }
        NodeValue::ThematicBreak => quote!("ThematicBreak todo!!!"),
        NodeValue::FootnoteDefinition(_) => quote!("FootnoteDefinition todo!!!"),
        NodeValue::Table(_) => quote!("Table todo!!!"),
        NodeValue::TableRow(_) => quote!("TableRow todo!!!"),
        NodeValue::TableCell => quote!("TableCell todo!!!"),
        NodeValue::Text(text) => {
            let text = text.clone();
            quote!(#text)
        }
        NodeValue::TaskItem(_) => quote!("TaskItem todo!!!"),
        NodeValue::SoftBreak => quote!("\n"),
        NodeValue::LineBreak => quote!(<br />),
        NodeValue::Code(_) => quote!("Code todo!!!"),
        NodeValue::HtmlInline(_) => quote!("HtmlInline todo!!!"),
        NodeValue::Emph => quote!("Emph todo!!!"),
        NodeValue::Strong => quote!("Strong todo!!!"),
        NodeValue::Strikethrough => quote!("Strikethrough todo!!!"),
        NodeValue::Superscript => quote!("Superscript todo!!!"),
        NodeValue::Link(_) => quote!("Link todo!!!"),
        NodeValue::Image(_) => quote!("Image todo!!!"),
        NodeValue::FootnoteReference(_) => quote!("FootnoteReference todo!!!"),
    }
}

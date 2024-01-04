mod code_block;

use comrak::{
    nodes::{AstNode, NodeValue},
    parse_document, Arena,
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::ItemMacro;

pub fn parse_markdown(md_text: &str) -> Result<(TokenStream, Vec<String>), String> {
    let mut demos: Vec<String> = vec![];

    let arena = Arena::new();
    let mut options = comrak::Options::default();
    options.extension.table = true;

    let root = parse_document(&arena, &md_text, &options);
    let body = iter_nodes(root, &mut demos);
    Ok((body, demos))
}

fn iter_nodes<'a>(node: &'a AstNode<'a>, demos: &mut Vec<String>) -> TokenStream {
    let mut children = vec![];
    for c in node.children() {
        children.push(iter_nodes(c, demos));
    }
    match &node.data.borrow().value {
        NodeValue::Document => quote!(#(#children)*),
        NodeValue::FrontMatter(_) => quote!("FrontMatter todo!!!"),
        NodeValue::BlockQuote => quote!("BlockQuote todo!!!"),
        NodeValue::List(_) => quote!("List todo!!!"),
        NodeValue::Item(_) => quote!("Item todo!!!"),
        NodeValue::DescriptionList => quote!("DescriptionList todo!!!"),
        NodeValue::DescriptionItem(_) => quote!("DescriptionItem todo!!!"),
        NodeValue::DescriptionTerm => quote!("DescriptionTerm todo!!!"),
        NodeValue::DescriptionDetails => quote!("DescriptionDetails todo!!!"),
        NodeValue::CodeBlock(node_code_block) => code_block::to_tokens(node_code_block, demos),
        NodeValue::HtmlBlock(node_html_block) => {
            let html =
                syn::parse_str::<ItemMacro>(&format!("view! {{ {} }}", node_html_block.literal))
                    .expect(&format!(
                        "Cannot be resolved as a macro: \n {}",
                        node_html_block.literal
                    ));
            quote!(
                {
                    #html
                }
            )
        }
        NodeValue::Paragraph => quote!(
            <p>
                #(#children)*
            </p >
        ),
        NodeValue::Heading(node_h) => {
            let h = Ident::new(&format!("h{}", node_h.level), Span::call_site());
            quote!(
                <#h>
                    #(#children)*
                </#h>
            )
        }
        NodeValue::ThematicBreak => quote!("ThematicBreak todo!!!"),
        NodeValue::FootnoteDefinition(_) => quote!("FootnoteDefinition todo!!!"),
        NodeValue::Table(_) => {
            let header_index = {
                let mut header_index = 0;
                for (index, c) in node.children().enumerate() {
                    let row = &c.data.borrow().value;
                    let is_header = match *row {
                        NodeValue::TableRow(header) => header,
                        _ => panic!(),
                    };
                    if !is_header {
                        header_index = index;
                        break;
                    }
                }
                header_index
            };
            let header_children: Vec<TokenStream> = children.drain(0..header_index).collect();

            quote!(
                <div class="demo-md-table-box">
                    <Table single_column=true>
                        <thead>
                            #(#header_children)*
                        </thead>
                        <tbody>
                            #(#children)*
                        </tbody>
                    </Table>
                </div>
            )
        }
        NodeValue::TableRow(_) => {
            quote!(
                <tr>
                    #(#children)*
                </tr>
            )
        }
        NodeValue::TableCell => {
            let row = &node.parent().unwrap().data.borrow().value;
            let is_header = match *row {
                NodeValue::TableRow(header) => header,
                _ => panic!(),
            };
            if is_header {
                quote!(
                    <th>
                        #(#children)*
                    </th>
                )
            } else {
                quote!(
                    <td>
                        #(#children)*
                    </td>
                )
            }
        }
        NodeValue::Text(text) => {
            let text = text.clone();
            quote!(#text)
        }
        NodeValue::TaskItem(_) => quote!("TaskItem todo!!!"),
        NodeValue::SoftBreak => quote!("\n"),
        NodeValue::LineBreak => quote!(<br />),
        NodeValue::Code(node_code) => {
            let code = node_code.literal.clone();
            quote!(
                <Text code=true>
                    #code
                </Text>
            )
        }
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

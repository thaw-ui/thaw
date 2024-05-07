mod code_block;

use comrak::{
    nodes::{AstNode, LineColumn, NodeValue},
    parse_document, Arena,
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::ItemMacro;

#[allow(clippy::type_complexity)]
pub fn parse_markdown(
    md_text: &str,
) -> Result<(TokenStream, Vec<String>, Vec<(String, String)>), String> {
    let mut demos: Vec<String> = vec![];
    let mut toc: Vec<(String, String)> = vec![];

    let arena = Arena::new();
    let mut options = comrak::Options::default();
    options.extension.table = true;

    let root = parse_document(&arena, md_text, &options);
    let body = iter_nodes(md_text, root, &mut demos, &mut toc);
    Ok((body, demos, toc))
}

fn iter_nodes<'a>(
    md_text: &str,
    node: &'a AstNode<'a>,
    demos: &mut Vec<String>,
    toc: &mut Vec<(String, String)>,
) -> TokenStream {
    let mut children = vec![];
    for c in node.children() {
        children.push(iter_nodes(md_text, c, demos, toc));
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
                    .unwrap_or_else(|_| {
                        panic!(
                            "Cannot be resolved as a macro: \n {}",
                            node_html_block.literal
                        )
                    });
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
            let sourcepos = node.data.borrow().sourcepos;
            let text = range_text(md_text, sourcepos.start, sourcepos.end);
            let level = node_h.level as usize + 1;
            let text = text[level..].to_string();
            let h_id = text.replace(' ', "-").to_ascii_lowercase().to_string();
            toc.push((h_id.clone(), text));
            let h = Ident::new(&format!("h{}", node_h.level), Span::call_site());
            quote!(
                <#h id=#h_id>
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
        NodeValue::MultilineBlockQuote(_) => quote!("FootnoteReference todo!!!"),
    }
}

fn range_text(text: &str, start: LineColumn, end: LineColumn) -> &str {
    let LineColumn {
        line: start_line,
        column: start_col,
    } = start;
    let LineColumn {
        line: end_line,
        column: end_col,
    } = end;

    let mut lines = text.lines();

    let mut current_line_num = 1;
    let mut start_line_text = lines.next().unwrap_or("");
    while current_line_num < start_line {
        start_line_text = lines.next().unwrap_or("");
        current_line_num += 1;
    }

    let start_index = start_col - 1;
    let mut start_line_text = &start_line_text[start_index..];

    let mut current_line_num = start_line + 1;
    while current_line_num < end_line {
        let next_line = lines.next().unwrap_or("");
        start_line_text = next_line;
        current_line_num += 1;
    }

    let end_index = end_col;
    if current_line_num == end_line {
        start_line_text = &start_line_text[..end_index];
    }

    start_line_text
}

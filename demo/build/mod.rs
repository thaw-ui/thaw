mod markdown;

use markdown::parse_markdown;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::{
    fs::File,
    io::{Read, Write},
};
use syn::ItemFn;

static GUIDE_LIST: &'static [&'static str] = &["installation", "server_sider_rendering"];

static COMPONENT_LIST: &'static [&'static str] = &[
    "accordion",
    "anchor",
    "auto_complete",
    "avatar",
    "back_top",
    "badge",
    "breadcrumb",
    "button",
    "calendar",
    "card",
    "checkbox",
    "color_picker",
    "combobox",
    "config_provider",
    "date_picker",
    "dialog",
    "divider",
    "drawer",
    "grid",
    "icon",
    "image",
    "input",
    "layout",
    "loading_bar",
    "menu",
    "message_bar",
    "nav",
    "pagination",
    "popover",
    "progress_bar",
    "radio",
    "scrollbar",
    "skeleton",
    "slider",
    "space",
    "spin_button",
    "spinner",
    "switch",
    "tab_list",
    "table",
    "tag",
    "text",
    "textarea",
    "time_picker",
    "toast",
    "tooltip",
    "upload",
];

fn main() {
    let mut mod_text = String::new();
    let mut pub_text = String::new();
    let mut docs = vec![];
    for file_name in GUIDE_LIST.iter() {
        let comp_name = format!("{}MdPage", snake_to_pascal(file_name));
        let path = format!("./src/pages/docs/_guide/{}.md", file_name);
        docs.push((file_name, comp_name, path))
    }
    for file_name in COMPONENT_LIST.iter() {
        let comp_name = format!("{}MdPage", snake_to_pascal(file_name));
        let path = format!("../thaw/src/{}/docs/mod.md", file_name);
        docs.push((file_name, comp_name, path))
    }
    for (file_name, comp_name, path) in docs.iter() {
        let mut file = File::open(path).expect(&format!("Failed to open file! path -> {path}"));
        let mut md_text = String::new();
        file.read_to_string(&mut md_text).unwrap();

        let (body, demos, toc) = parse_markdown(&md_text).unwrap();

        let toc = {
            let links = toc
                .into_iter()
                .map(|h| format!(r##"<AnchorLink href="#{}" title="{}" />"##, h.0, h.1))
                .collect::<Vec<_>>()
                .join(" ");
            let toc = format!(
                r##"#[component] fn Toc() -> impl IntoView {{ view! {{ <Anchor offset_target=".doc-content">{}</Anchor> }} }}"##,
                links
            );
            syn::parse_str::<ItemFn>(&toc)
                .expect(&format!("Cannot be resolved as a function: \n {toc}"))
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
            .map(|demo| {
                syn::parse_str::<ItemFn>(&demo)
                    .unwrap_or_else(|_| panic!("Cannot be resolved as a function: \n {demo}"))
            })
            .collect();

        let comp_name = Ident::new(&comp_name, Span::call_site());
        let token = quote! {
            use crate::components::{Demo, DemoCode};
            use leptos::{ev, prelude::*};
            use thaw::*;

            #[component]
            pub fn #comp_name() -> impl IntoView {
                #(#demos)*

                #toc

                view! {
                    <div class="demo-components__component">
                        #body
                    </div>
                    <div class="demo-components__toc">
                        <Toc />
                    </div>
                }
            }
        };

        mod_text.push_str(&format!("mod {file_name};\n"));
        pub_text.push_str(&format!("pub use {file_name}::*;\n"));
        let path = format!("./src/pages/docs/{}.rs", file_name);
        let mut file = File::create(path).unwrap();
        file.write_all(token.to_string().as_bytes()).unwrap();
    }

    let mut file = File::create("./src/pages/docs/mod.rs").unwrap();
    file.write_all(format!("{mod_text}\n{pub_text}").as_bytes())
        .unwrap();
}

fn snake_to_pascal(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let (first, word) = word.split_at(1);
            format!("{}{word}", first.to_ascii_uppercase())
        })
        .collect()
}

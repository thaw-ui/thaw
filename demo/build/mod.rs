mod markdown;

use markdown::parse_markdown;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::{
    fs::File,
    io::{Read, Write},
};
use syn::ItemFn;

static FILE_LIST: &'static [&'static str] = &["Accordion"];

fn main() {
    let mut mod_text = String::new();
    let mut pub_text = String::new();
    for name in FILE_LIST.iter() {
        let name_lowercase = name.to_ascii_lowercase();
        mod_text.push_str(&format!("mod {name_lowercase};\n"));
        pub_text.push_str(&format!("pub use {name_lowercase}::*;\n"));

        let path = format!("../thaw/src/{}/docs/mod.md", name_lowercase);
        let mut file = File::open(path).unwrap();
        let mut file_str = String::new();
        file.read_to_string(&mut file_str).unwrap();

        let fn_name = format!("{}MdPage", name);
        let fn_name = Ident::new(&fn_name, Span::call_site());

        let (body, demos, toc) = parse_markdown(&file_str).unwrap();

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

        let token = quote! {
            use crate::components::{Demo, DemoCode};
            use leptos::{ev, prelude::*};
            use thaw::*;

            #[component]
            pub fn #fn_name() -> impl IntoView {
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

        let path = format!("./src/pages/docs/{}.rs", name_lowercase);
        let mut file = File::create(path).unwrap();
        file.write_all(token.to_string().as_bytes()).unwrap();
    }

    let mut file = File::create("./src/pages/docs/mod.rs").unwrap();
    file.write_all(format!("{mod_text}\n{pub_text}").as_bytes())
        .unwrap();
}

use leptos::*;
use leptos_meta::Style;
use thaw::*;

#[slot]
pub struct DemoCode {
    #[prop(default = true)]
    is_highlight: bool,
    children: Children,
}

#[component]
pub fn Demo(demo_code: DemoCode, children: Children) -> impl IntoView {
    let theme = use_theme(Theme::light);
    let style = create_memo(move |_| {
        let mut style = String::from("background-image: url(/thaw/grid_dot.svg); background-repeat: repeat; background-size: 1.5rem; margin-top: 1rem; padding: 1rem; border-top-left-radius: 0.5rem; border-top-right-radius: 0.5rem;");
        theme.with(|theme| {
            if theme.common.color_scheme == "dark" {
                style.push_str("border: 1px solid #383f52;");
            } else {
                style.push_str(&format!("border: 1px solid {};", theme.common.border_color));
            }
        });
        style
    });
    let code_style = create_memo(move |_| {
        let mut style = String::from("font-weight: 400; font-size: 0.875em; line-height: 1.7142857;margin-bottom: 1rem; padding: 1rem; border-bottom-left-radius: 0.5rem; border-bottom-right-radius: 0.5rem;");
        theme.with(|theme| {
            if theme.common.color_scheme == "dark" {
                style.push_str("border: 1px solid #383f52; border-top-width: 0;");
                style.push_str("background-color: #242832;");
            } else {
                style.push_str(&format!(
                    "border: 1px solid {}; border-top-width: 0;",
                    theme.common.border_color
                ));
                style.push_str("background-color: #f9fafb;");
            }
        });
        style
    });

    let is_highlight = demo_code.is_highlight;
    let frag = (demo_code.children)();
    let mut html = String::new();
    for node in frag.nodes {
        match node {
            View::Text(text) => html.push_str(&text.content),
            _ => {
                leptos::logging::warn!("Only text nodes are supported as children of <DemoCode />.")
            }
        }
    }

    view! {
        <Style id="leptos-thaw-prism-css">{prisms::prism_css!()}</Style>
        <Style id="leptos-thaw-prism-css-fix">
            ".token.operator {
                background: hsla(0, 0%, 100%, 0) !important;
            }"
        </Style>
        <div style=move || style.get()>{children()}</div>
        <div style=move || code_style.get()>
            <Code>
                {
                    if is_highlight {
                        view! {
                            <pre style="margin: 0" inner_html=html></pre>
                        }
                    } else {
                        view! {
                            <pre style="margin: 0">
                                {html}
                            </pre>
                        }
                    }
                }
            </Code>
        </div>
    }
}

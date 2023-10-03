use leptos::*;
use melt_ui::{mount_style, Code};

#[slot]
pub struct DemoCode {
    #[prop(optional)]
    html: &'static str,
    children: Children,
}

#[component]
pub fn Demo(demo_code: DemoCode, children: Children) -> impl IntoView {
    mount_style("demo", || ("", prisms::prism_css!()));
    view! {
       <div style="background-image: url(/melt-ui/grid_dot.svg); background-repeat: repeat; background-size: 1.5rem; margin-top: 1rem; padding: 1rem; border: 1px solid #e5e8eb; border-top-left-radius: 0.5rem; border-top-right-radius: 0.5rem;">
            { children() }
       </div>
       <div style="font-weight: 400; font-size: 0.875em; line-height: 1.7142857;margin-bottom: 1rem; padding: 1rem; background-color: #f9fafb; border: 1px solid #e5e8eb; border-bottom-left-radius: 0.5rem; border-bottom-right-radius: 0.5rem; border-top-width: 0;">
            <Code>
                <pre style="margin: 0" inner_html=demo_code.html>
                    { (demo_code.children)() }
                </pre>
            </Code>
       </div>
    }
}

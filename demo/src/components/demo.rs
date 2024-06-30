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
pub fn Demo(demo_code: DemoCode, #[prop(optional)] children: Option<Children>) -> impl IntoView {
    let theme = Theme::use_theme(Theme::light);
    let css_vars = Memo::new(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            if theme.color.color_scheme == "dark" {
                css_vars.push_str("--demo-color: #ffffff60;");
                css_vars.push_str("--demo-color-hover: #ffffffe0;");
                css_vars.push_str("--demo-border-color: #383f52;");
                css_vars.push_str("--demo-background-color: #242832;");
            } else {
                css_vars.push_str("--demo-color: #00000060;");
                css_vars.push_str("--demo-color-hover: #000000e0;");
                css_vars.push_str(&format!(
                    "--demo-border-color: var(--colorNeutralStroke2);",
                ));
                css_vars.push_str("--demo-background-color: #f9fafb;");
            }
        });
        css_vars
    });

    let code_class = Memo::new(move |_| {
        theme.with(|theme| format!("demo-demo__code color-scheme--{}", theme.color.color_scheme))
    });
    let is_show_code = RwSignal::new(children.is_none());

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
        <Style id="leptos-thaw-syntect-css">
            {include_str!("./syntect-css.css")}
        </Style>
        <Style id="demo-demo">
            {include_str!("./demo.css")}
        </Style>
        <div class="demo-demo" style=move || css_vars.get()>
            {
                if let Some(children) = children {
                    view! {
                        <div class="demo-demo__view">{children()}</div>
                        <div class="demo-demo__toolbar" class=("demo-demo__toolbar--code", move || !is_show_code.get())>
                            <Popover appearance=PopoverAppearance::Inverted>
                                <PopoverTrigger slot>
                                    <span on:click=move |_| is_show_code.update(|show| *show = !*show) class="demo-demo__toolbar-btn">
                                        {
                                            move || if is_show_code.get() {
                                                view! {
                                                    <Icon icon=icondata::LuCode2/>
                                                }
                                            } else {
                                                view! {
                                                    <Icon icon=icondata::LuCode/>
                                                }
                                            }
                                        }
                                    </span>
                                </PopoverTrigger>
                                {
                                    move || if is_show_code.get() {
                                        "Hide code"
                                    } else {
                                        "Show code"
                                    }
                                }
                            </Popover>

                        </div>
                    }.into()
                } else {
                    None
                }
            }
            <div class=move || code_class.get() style:display=move || (!is_show_code.get()).then_some("none")>
                {
                    if is_highlight {
                        view! {
                            <Code inner_html=html />
                        }
                    } else {
                        view! {
                            <Code text=html />
                        }
                    }
                }
            </div>
        </div>
    }
}

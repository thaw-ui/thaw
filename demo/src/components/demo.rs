use leptos::{either::Either, prelude::*};
use leptos_meta::Style;
use thaw::*;

#[slot]
pub struct DemoCode {
    #[prop(default = true)]
    is_highlight: bool,
    #[prop(into)]
    text: String,
}

#[component]
pub fn Demo(
    #[prop(optional)] remove_scrollbar: bool,
    demo_code: DemoCode,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let theme = Theme::use_theme(Theme::light);
    let css_vars = Memo::new(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            if theme.color.color_scheme() == "dark" {
                css_vars.push_str("--demo-border-color: #383f52;");
                css_vars.push_str("--demo-background-color: #242832;");
            } else {
                css_vars.push_str(&format!("--demo-border-color: var(--colorNeutralStroke2);",));
                css_vars.push_str("--demo-background-color: #f9fafb;");
            }
        });
        css_vars
    });

    let code_class = Memo::new(move |_| {
        theme.with(|theme| {
            format!(
                "demo-demo__code color-scheme--{}",
                theme.color.color_scheme()
            )
        })
    });
    let is_show_code = RwSignal::new(children.is_none());

    let is_highlight = demo_code.is_highlight;
    let html = demo_code.text;
    let styles = use_context::<DemoStyle>().is_none().then(|| {
        view! {
            <Style id="leptos-thaw-syntect-css">{include_str!("./syntect-css.css")}</Style>
            <Style id="demo-demo">{include_str!("./demo.css")}</Style>
        }
    });
    provide_context(DemoStyle);

    view! {
        {styles}
        <div class="demo-demo" style=move || css_vars.get()>
            {if let Some(children) = children {
                view! {
                    {if remove_scrollbar {
                        Either::Left(view! { <div class="demo-demo__view">{children()}</div> })
                    } else {
                        Either::Right(
                            view! {
                                <Scrollbar>
                                    <div class="demo-demo__view">{children()}</div>
                                </Scrollbar>
                            },
                        )
                    }}
                    <div
                        class="demo-demo__toolbar"
                        class=("demo-demo__toolbar--code", move || !is_show_code.get())
                    >
                        <Tooltip
                            content=Signal::derive(move || {
                                if is_show_code.get() {
                                    "Hide code".to_string()
                                } else {
                                    "Show code".to_string()
                                }
                            })
                            appearance=TooltipAppearance::Inverted
                        >
                            <Button
                                icon=MaybeProp::derive(move || {
                                    if is_show_code.get() {
                                        Some(icondata::LuCode2)
                                    } else {
                                        Some(icondata::LuCode)
                                    }
                                })
                                on:click=move |_| is_show_code.update(|show| *show = !*show)
                                appearance=ButtonAppearance::Transparent
                                size=ButtonSize::Small
                            />
                        </Tooltip>
                    </div>
                }
                    .into()
            } else {
                None
            }}
            <Scrollbar>
                <div
                    class=move || code_class.get()
                    style:display=move || {
                        (!is_show_code.get()).then_some("none").unwrap_or_default()
                    }
                >
                    {if is_highlight {
                        view! { <Code inner_html=html /> }
                    } else {
                        view! { <Code text=html /> }
                    }}
                </div>
            </Scrollbar>
        </div>
    }
}

#[derive(Clone)]
pub struct DemoStyle;

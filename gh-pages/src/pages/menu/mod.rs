use indoc::indoc;
use leptos::*;
use melt_ui::*;

#[component]
pub fn MenuPage(cx: Scope) -> impl IntoView {
    let selected = create_rw_signal(cx, String::from("o"));
    view! { cx,
        <Card>
            { move || selected.get() }
            <Menu selected>
                <MenuItem key="a" label="and"/>
                <MenuItem key="o" label="or"/>
            </Menu>
            <CardFooter slot>
                <Code>
                    <pre>
                        {
                            indoc!(r#"
                            <Menu selected>
                                <MenuItem key="a" label="and"/>
                                <MenuItem key="o" label="or"/>
                            </Menu>
                            "#)
                        }
                    </pre>
                </Code>
            </CardFooter>
        </Card>
    }
}

use indoc::indoc;
use leptos::*;
use melt_ui::*;

#[component]
pub fn MenuPage() -> impl IntoView {
    let selected = create_rw_signal(String::from("o"));
    view! {
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

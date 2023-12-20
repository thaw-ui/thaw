use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn PopoverPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Popover"</h1>
            <Demo>
                <Popover>
                    <PopoverTrigger slot>
                        <Button>
                            "Hover"
                        </Button>
                    </PopoverTrigger>
                    "Content"
                </Popover>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        <Popover>
                            <PopoverTrigger slot>
                                <Button>
                                    "Hover"
                                </Button>
                            </PopoverTrigger>
                            "Content"
                        </Popover>
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Popover Props"</h3>
            <Table single_column=true>
                <thead>
                    <tr>
                        <th>"Name"</th>
                        <th>"Type"</th>
                        <th>"Default"</th>
                        <th>"Description"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"children"</td>
                        <td>
                            <Text code=true>"Children"</Text>
                        </td>
                        <td></td>
                        <td></td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}

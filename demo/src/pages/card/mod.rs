use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn CardPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Card"</h1>
            <Demo>
                <Space vertical=true>
                    <Card title="title">"content"</Card>
                    <Card title="title">
                        <CardHeaderExtra slot>"header-extra"</CardHeaderExtra>
                        "content"
                    </Card>
                    <Card title="title">
                        <CardHeader slot>"header"</CardHeader>
                        "content"
                    </Card>
                    <Card title="title">
                        <CardHeaderExtra slot>"header-extra"</CardHeaderExtra>
                        "content"
                        <CardFooter slot>"footer"</CardFooter>
                    </Card>
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        <Space vertical=true> 
                            <Card title="title">
                                "content"
                            </Card>
                            <Card title="title">
                                <CardHeaderExtra slot>
                                    "header-extra"
                                </CardHeaderExtra>
                                "content"
                            </Card>
                            <Card title="title">
                                <CardHeader slot>
                                    "header"
                                </CardHeader>
                                "content"
                            </Card>
                            <Card title="title">
                                <CardHeaderExtra slot>
                                    "header-extra"
                                </CardHeaderExtra>
                                "content"
                                <CardFooter slot>
                                    "footer"
                                </CardFooter>
                            </Card>
                        </Space>
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Card Props"</h3>
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
                        <td>"title"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Card title."</td>
                    </tr>
                    <tr>
                        <td>"children"</td>
                        <td>"Children"</td>
                        <td></td>
                        <td>"Card's content."</td>
                    </tr>
                    <tr>
                        <td>"class"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Addtional classes for the card element."</td>
                    </tr>
                </tbody>
            </Table>
            <h3>"Card Slots"</h3>
            <Table single_column=true>
                <thead>
                    <tr>
                        <th>"Name"</th>
                        <th>"Default"</th>
                        <th>"Description"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"CardHeader"</td>
                        <td>"None"</td>
                        <td>"Header content."</td>
                    </tr>
                    <tr>
                        <td>"CardHeaderExtra"</td>
                        <td>"None"</td>
                        <td>"Header extra content."</td>
                    </tr>
                    <tr>
                        <td>"CardFooter"</td>
                        <td>"None"</td>
                        <td>"Footer content."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}

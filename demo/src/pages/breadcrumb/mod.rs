use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn BreadcrumbPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Breadcrumb"</h1>
            <Demo>
                <Breadcrumb>
                    <BreadcrumbItem>"Leptos"</BreadcrumbItem>
                    <BreadcrumbItem>"UI"</BreadcrumbItem>
                    <BreadcrumbItem>"Thaw"</BreadcrumbItem>
                </Breadcrumb>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        <Breadcrumb>
                            <BreadcrumbItem>
                                "Leptos"
                            </BreadcrumbItem>
                            <BreadcrumbItem>
                                "UI"
                            </BreadcrumbItem>
                            <BreadcrumbItem>
                                "Thaw"
                            </BreadcrumbItem>
                        </Breadcrumb>
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Separator"</h3>
            <Demo>
                <Breadcrumb separator=">">
                    <BreadcrumbItem>"Leptos"</BreadcrumbItem>
                    <BreadcrumbItem>"UI"</BreadcrumbItem>
                    <BreadcrumbItem>"Thaw"</BreadcrumbItem>
                </Breadcrumb>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        <Breadcrumb separator=">">
                            <BreadcrumbItem>
                                "Leptos"
                            </BreadcrumbItem>
                            <BreadcrumbItem>
                                "UI"
                            </BreadcrumbItem>
                            <BreadcrumbItem>
                                "Thaw"
                            </BreadcrumbItem>
                        </Breadcrumb>
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Breadcrumb Props"</h3>
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
                        <td>"separator"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"/"</td>
                        <td>"Breadcrumb separator."</td>
                    </tr>
                    <tr>
                        <td>"children"</td>
                        <td>"Children"</td>
                        <td></td>
                        <td>"Breadcrumb's content."</td>
                    </tr>
                    <tr>
                        <td>"class"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Addtional classes for the breadcrumb element."</td>
                    </tr>
                </tbody>
            </Table>
            <h3>"BreadcrumbItem Props"</h3>
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
                        <td>"Children"</td>
                        <td></td>
                        <td>"BreadcrumbItem's content."</td>
                    </tr>
                    <tr>
                        <td>"class"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Addtional classes for the breadcrumb link element."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}

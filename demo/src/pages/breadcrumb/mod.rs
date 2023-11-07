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
                    <BreadcrumbItem>
                        "Rust"
                    </BreadcrumbItem>
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
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    <Breadcrumb>
                        <BreadcrumbItem>
                            "Rust"
                        </BreadcrumbItem>
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
                    )
                >

                    ""
                </DemoCode>
            </Demo>
        </div>
    }
}

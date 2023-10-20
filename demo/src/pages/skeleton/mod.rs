use crate::components::{Demo, DemoCode};
use leptos::*;
use melt_ui::*;
use prisms::highlight_str;

#[component]
pub fn SkeletonPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Skeleton"</h1>
            <Demo>
                <Skeleton repeat=2 text=true />
                <Skeleton width="60%" text=true />
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                        <Skeleton repeat=2 text=true />
                        <Skeleton width="60%" text=true />
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

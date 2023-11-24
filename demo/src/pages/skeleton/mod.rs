use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn SkeletonPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Skeleton"</h1>
            <Demo>
                <Skeleton repeat=2 text=true/>
                <Skeleton width="60%" text=true/>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                            <Skeleton repeat=2 text=true />
                            <Skeleton width="60%" text=true />
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Skeleton Props"</h3>
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
                        <td>"repeat"</td>
                        <td>"MaybeSignal<u32>"</td>
                        <td>"1"</td>
                        <td>"Repeat frequency."</td>
                    </tr>
                    <tr>
                        <td>"text"</td>
                        <td>"MaybeSignal<bool>"</td>
                        <td>"false"</td>
                        <td>"Text skeleton."</td>
                    </tr>
                    <tr>
                        <td>"width"</td>
                        <td>"Option<MaybeSignal<String>>"</td>
                        <td>"None"</td>
                        <td>"Skeleton width."</td>
                    </tr>
                    <tr>
                        <td>"height"</td>
                        <td>"Option<MaybeSignal<String>>"</td>
                        <td>"None"</td>
                        <td>"Text skeleton."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}

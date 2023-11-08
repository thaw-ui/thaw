use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn GridPage() -> impl IntoView {
    mount_style(
        "grid-demo",
        r#".thaw-grid-item {
        height: 60px;
        text-align: center;
        line-height: 60px;
      }
      .thaw-grid-item:nth-child(odd) {
        background-color: #0078ff88;
      }
      .thaw-grid-item:nth-child(even) {
        background-color: #0078ffaa;
      }"#,
    );
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Grid"</h1>
            <Demo>
                <Space vertical=true>
                    <Grid>
                        <GridItem>"123"</GridItem>
                        <GridItem>"456"</GridItem>
                        <GridItem>"789"</GridItem>
                    </Grid>

                    <Grid cols=2>
                        <GridItem>"123"</GridItem>
                        <GridItem>"456"</GridItem>
                        <GridItem>"789"</GridItem>
                    </Grid>
                </Space>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                        <Grid>
                            <GridItem>"123"</GridItem>
                            <GridItem>"456"</GridItem>
                            <GridItem>"789"</GridItem>
                        </Grid>

                        <Grid :cols=2>
                            <GridItem>"123"</GridItem>
                            <GridItem>"456"</GridItem>
                            <GridItem>"789"</GridItem>
                        </Grid>
                "#,
                        "rust"
                    )
                >

                    ""
                </DemoCode>
            </Demo>
            <h3>"gap"</h3>
            <Demo>
                <Grid cols=3 x_gap=8 y_gap=8>
                    <GridItem>"123"</GridItem>
                    <GridItem>"321"</GridItem>
                    <GridItem>"123"</GridItem>
                    <GridItem>"456"</GridItem>
                    <GridItem>"7"</GridItem>
                    <GridItem>"123"</GridItem>
                    <GridItem>"123"</GridItem>
                    <GridItem span=2>"1234"</GridItem>
                    <GridItem>"567"</GridItem>
                    <GridItem>"567"</GridItem>
                </Grid>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                        <Grid cols=3 x_gap=8 y_gap=8>
                            <GridItem>"123"</GridItem>
                            <GridItem>"321"</GridItem>
                            <GridItem>"123"</GridItem>
                            <GridItem>"456"</GridItem>
                            <GridItem>"7"</GridItem>
                            <GridItem>"123"</GridItem>
                            <GridItem>"123"</GridItem>
                            <GridItem span=2>"1234"</GridItem>
                            <GridItem >"567"</GridItem>
                            <GridItem >"567"</GridItem>
                        </Grid>
                "#,
                        "rust"
                    )
                >

                    ""
                </DemoCode>
            </Demo>
            <h3>"gap"</h3>
            <Demo>
                <Grid cols=4>
                    <GridItem offset=2>"123"</GridItem>
                    <GridItem>"456"</GridItem>
                </Grid>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                        <Grid cols=4>
                            <GridItem offset=2>"123"</GridItem>
                            <GridItem>"456"</GridItem>
                        </Grid>
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

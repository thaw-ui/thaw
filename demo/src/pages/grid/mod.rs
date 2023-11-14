use crate::components::{Demo, DemoCode};
use leptos::*;
use leptos_meta::Style;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn GridPage() -> impl IntoView {
    view! {
        <Style>
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
            }"#
        </Style>
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
                    <GridItem column=2>"1234"</GridItem>
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
                            <GridItem column=2>"1234"</GridItem>
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
            <h3>"Grid Props"</h3>
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
                        <td>"cols"</td>
                        <td>"MaybeSignal<u16>"</td>
                        <td>"1"</td>
                        <td>"Number of grids displayed."</td>
                    </tr>
                    <tr>
                        <td>"x_gap"</td>
                        <td>"MaybeSignal<u16>"</td>
                        <td>"0"</td>
                        <td>"Horizontal gap."</td>
                    </tr>
                    <tr>
                        <td>"y_gap"</td>
                        <td>"MaybeSignal<u16>"</td>
                        <td>"0"</td>
                        <td>"Vertical gap."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}

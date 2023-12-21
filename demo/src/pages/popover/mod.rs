use crate::components::{Demo, DemoCode};
use leptos::*;
use leptos_meta::Style;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn PopoverPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Popover"</h1>
            <Demo>
                <Space>
                    <Popover>
                        <PopoverTrigger slot>
                            <Button>"Hover"</Button>
                        </PopoverTrigger>
                        "Content"
                    </Popover>
                    <Popover trigger_type=PopoverTriggerType::Click>
                        <PopoverTrigger slot>
                            <Button>"Click"</Button>
                        </PopoverTrigger>
                        "Content"
                    </Popover>
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        <Space>
                            <Popover>
                                <PopoverTrigger slot>
                                    <Button>
                                        "Hover"
                                    </Button>
                                </PopoverTrigger>
                                "Content"
                            </Popover>
                        </Space>
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Placement"</h3>
            <Style>".demo-popover .thaw-button { width: 100% } .demo-popover .thaw-popover-trigger { display: block }"</Style>
            <Demo>
                <Grid x_gap=8 y_gap=8 cols=3 class="demo-popover">
                    <GridItem>
                        <Popover placement=PopoverPlacement::TopStart>
                            <PopoverTrigger slot>
                                <Button>"Top Start"</Button>
                            </PopoverTrigger>
                            "Content"
                        </Popover>
                    </GridItem>
                    <GridItem>
                        <Popover placement=PopoverPlacement::Top>
                            <PopoverTrigger slot>
                                <Button>"Top"</Button>
                            </PopoverTrigger>
                            "Content"
                        </Popover>
                    </GridItem>
                    <GridItem>
                        <Popover placement=PopoverPlacement::TopEnd>
                            <PopoverTrigger slot>
                                <Button>"Top End"</Button>
                            </PopoverTrigger>
                            "Content"
                        </Popover>
                    </GridItem>
                    <GridItem>
                        <Popover placement=PopoverPlacement::LeftStart>
                            <PopoverTrigger slot>
                                <Button>"Left Start"</Button>
                            </PopoverTrigger>
                            "Content"
                        </Popover>
                    </GridItem>
                    <GridItem offset=1>
                        <Popover placement=PopoverPlacement::RightStart>
                            <PopoverTrigger slot>
                                <Button>"Right Start"</Button>
                            </PopoverTrigger>
                            "Content"
                        </Popover>
                    </GridItem>
                    <GridItem>
                        <Popover placement=PopoverPlacement::Left>
                            <PopoverTrigger slot>
                                <Button>"Left"</Button>
                            </PopoverTrigger>
                            "Content"
                        </Popover>
                    </GridItem>
                    <GridItem offset=1>
                        <Popover placement=PopoverPlacement::Right>
                            <PopoverTrigger slot>
                                <Button>"Right"</Button>
                            </PopoverTrigger>
                            "Content"
                        </Popover>
                    </GridItem>
                    <GridItem>
                        <Popover placement=PopoverPlacement::LeftEnd>
                            <PopoverTrigger slot>
                                <Button>"Left End"</Button>
                            </PopoverTrigger>
                            "Content"
                        </Popover>
                    </GridItem>
                    <GridItem offset=1>
                        <Popover placement=PopoverPlacement::RightEnd>
                            <PopoverTrigger slot>
                                <Button>"Right End"</Button>
                            </PopoverTrigger>
                            "Content"
                        </Popover>
                    </GridItem>
                    <GridItem>
                        <Popover placement=PopoverPlacement::BottomStart>
                            <PopoverTrigger slot>
                                <Button>"Bottom Start"</Button>
                            </PopoverTrigger>
                            "Content"
                        </Popover>
                    </GridItem>
                    <GridItem>
                        <Popover placement=PopoverPlacement::Bottom>
                            <PopoverTrigger slot>
                                <Button>"Bottom"</Button>
                            </PopoverTrigger>
                            "Content"
                        </Popover>
                    </GridItem>
                    <GridItem>
                        <Popover placement=PopoverPlacement::BottomEnd>
                            <PopoverTrigger slot>
                                <Button>"Bottom End"</Button>
                            </PopoverTrigger>
                            "Content"
                        </Popover>
                    </GridItem>
                </Grid>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                    <Grid x_gap=8 y_gap=8 cols=3 class="demo-popover">
                        <GridItem>
                            <Popover placement=PopoverPlacement::TopStart>
                                <PopoverTrigger slot>
                                    <Button>"Top Start"</Button>
                                </PopoverTrigger>
                                "Content"
                            </Popover>
                        </GridItem>
                        <GridItem>
                            <Popover placement=PopoverPlacement::Top>
                                <PopoverTrigger slot>
                                    <Button>"Top"</Button>
                                </PopoverTrigger>
                                "Content"
                            </Popover>
                        </GridItem>
                        <GridItem>
                            <Popover placement=PopoverPlacement::TopEnd>
                                <PopoverTrigger slot>
                                    <Button>"Top End"</Button>
                                </PopoverTrigger>
                                "Content"
                            </Popover>
                        </GridItem>
                        <GridItem>
                            <Popover placement=PopoverPlacement::LeftStart>
                                <PopoverTrigger slot>
                                    <Button>"Left Start"</Button>
                                </PopoverTrigger>
                                "Content"
                            </Popover>
                        </GridItem>
                        <GridItem offset=1>
                            <Popover placement=PopoverPlacement::RightStart>
                                <PopoverTrigger slot>
                                    <Button>"Right Start"</Button>
                                </PopoverTrigger>
                                "Content"
                            </Popover>
                        </GridItem>
                        <GridItem>
                            <Popover placement=PopoverPlacement::Left>
                                <PopoverTrigger slot>
                                    <Button>"Left"</Button>
                                </PopoverTrigger>
                                "Content"
                            </Popover>
                        </GridItem>
                        <GridItem offset=1>
                            <Popover placement=PopoverPlacement::Right>
                                <PopoverTrigger slot>
                                    <Button>"Right"</Button>
                                </PopoverTrigger>
                                "Content"
                            </Popover>
                        </GridItem>
                        <GridItem>
                            <Popover placement=PopoverPlacement::LeftEnd>
                                <PopoverTrigger slot>
                                    <Button>"Left End"</Button>
                                </PopoverTrigger>
                                "Content"
                            </Popover>
                        </GridItem>
                        <GridItem offset=1>
                            <Popover placement=PopoverPlacement::RightEnd>
                                <PopoverTrigger slot>
                                    <Button>"Right End"</Button>
                                </PopoverTrigger>
                                "Content"
                            </Popover>
                        </GridItem>
                        <GridItem>
                            <Popover placement=PopoverPlacement::BottomStart>
                                <PopoverTrigger slot>
                                    <Button>"Bottom Start"</Button>
                                </PopoverTrigger>
                                "Content"
                            </Popover>
                        </GridItem>
                        <GridItem>
                            <Popover placement=PopoverPlacement::Bottom>
                                <PopoverTrigger slot>
                                    <Button>"Bottom"</Button>
                                </PopoverTrigger>
                                "Content"
                            </Popover>
                        </GridItem>
                        <GridItem>
                            <Popover placement=PopoverPlacement::BottomEnd>
                                <PopoverTrigger slot>
                                    <Button>"Bottom End"</Button>
                                </PopoverTrigger>
                                "Content"
                            </Popover>
                        </GridItem>
                    </Grid>
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
                        <td>"class"</td>
                        <td>
                            <Text code=true>"MaybeSignal<String>"</Text>
                        </td>
                        <td>
                            <Text code=true>"Default::default()"</Text>
                        </td>
                        <td>"Addtional classes for the trigger element."</td>
                    </tr>
                    <tr>
                        <td>"content_class"</td>
                        <td>
                            <Text code=true>"MaybeSignal<String>"</Text>
                        </td>
                        <td>
                            <Text code=true>"Default::default()"</Text>
                        </td>
                        <td>"Content class of the popover."</td>
                    </tr>
                    <tr>
                        <td>"placement"</td>
                        <td>
                            <Text code=true>"PopoverPlacement"</Text>
                        </td>
                        <td>
                            <Text code=true>"PopoverPlacement::Top"</Text>
                        </td>
                        <td>"Popover placement."</td>
                    </tr>
                    <tr>
                        <td>"children"</td>
                        <td>
                            <Text code=true>"Children"</Text>
                        </td>
                        <td></td>
                        <td>"The content inside popover."</td>
                    </tr>
                </tbody>
            </Table>
            <h3>"Popover Slots"</h3>
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
                        <td>"PopoverTrigger"</td>
                        <td></td>
                        <td>"The element or component that triggers popover."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}

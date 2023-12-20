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
            <Demo>
            <Grid x_gap=8 y_gap=8 cols=3>
                <GridItem>
                    <Popover>
                        <PopoverTrigger slot>
                            <Button>
                                "Top Start"
                            </Button>
                        </PopoverTrigger>
                        "Content"
                    </Popover>
                </GridItem>
                <GridItem>
                    <Popover placement=PopoverPlacement::Top>
                        <PopoverTrigger slot>
                            <Button>
                                "Top"
                            </Button>
                        </PopoverTrigger>
                        "Content"
                    </Popover>
                </GridItem>
                <GridItem>
                    <Popover>
                        <PopoverTrigger slot>
                            <Button>
                                "Top End"
                            </Button>
                        </PopoverTrigger>
                        "Content"
                    </Popover>
                </GridItem>
                <GridItem>
                    <Popover>
                        <PopoverTrigger slot>
                            <Button>
                                "Left Start"
                            </Button>
                        </PopoverTrigger>
                        "Content"
                    </Popover>
                </GridItem>
                <GridItem offset=1>
                    <Popover>
                        <PopoverTrigger slot>
                            <Button>
                                "Right Start"
                            </Button>
                        </PopoverTrigger>
                        "Content"
                    </Popover>
                </GridItem>
                <GridItem>
                    <Popover placement=PopoverPlacement::Left>
                        <PopoverTrigger slot>
                            <Button>
                                "Left"
                            </Button>
                        </PopoverTrigger>
                        "Content"
                    </Popover>
                </GridItem>
                <GridItem offset=1>
                    <Popover placement=PopoverPlacement::Right>
                        <PopoverTrigger slot>
                            <Button>
                                "Right"
                            </Button>
                        </PopoverTrigger>
                        "Content"
                    </Popover>
                </GridItem>
                <GridItem>
                    <Popover>
                        <PopoverTrigger slot>
                            <Button>
                                "Left End"
                            </Button>
                        </PopoverTrigger>
                        "Content"
                    </Popover>
                </GridItem>
                <GridItem offset=1>
                    <Popover>
                        <PopoverTrigger slot>
                            <Button>
                                "Right End"
                            </Button>
                        </PopoverTrigger>
                        "Content"
                    </Popover>
                </GridItem>
                <GridItem>
                    <Popover>
                        <PopoverTrigger slot>
                            <Button>
                                "Bottom Start"
                            </Button>
                        </PopoverTrigger>
                        "Content"
                    </Popover>
                </GridItem>
                <GridItem>
                    <Popover placement=PopoverPlacement::Bottom>
                        <PopoverTrigger slot>
                            <Button>
                                "Bottom"
                            </Button>
                        </PopoverTrigger>
                        "Content"
                    </Popover>
                </GridItem>
                <GridItem>
                    <Popover>
                        <PopoverTrigger slot>
                            <Button>
                                "Bottom End"
                            </Button>
                        </PopoverTrigger>
                        "Content"
                    </Popover>
                </GridItem>
            </Grid>
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

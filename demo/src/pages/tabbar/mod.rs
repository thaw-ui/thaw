use crate::{
    components::{Demo, DemoCode},
    pages::MobilePage,
};
use leptos::*;
use prisms::highlight_str;
use thaw::{mobile::*, Table};

#[component]
pub fn TabbarPage() -> impl IntoView {
    view! {
        <div style="display: flex">
            <div style="width: 896px; margin: 0 auto;">
                <h1>"Tabbar"</h1>
                <Demo>
                    ""
                    <DemoCode slot>

                        {highlight_str!(
                            r#"
                            let value = create_rw_signal(String::from("o"));
                                    
                            <Tabbar value>
                                <TabbarItem name="a">
                                    "and"
                                </TabbarItem>
                                <TabbarItem name="i">
                                    "if"
                                </TabbarItem>
                                <TabbarItem name="o" icon=icondata::AiIcon::AiCloseOutlined>
                                    "or"
                                </TabbarItem>
                            </Tabbar>
                        "#,
                            "rust"
                        )}

                    </DemoCode>
                </Demo>
                <h3>"Tabbar Props"</h3>
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
                            <td>"value"</td>
                            <td>"RwSignal<String>"</td>
                            <td>"Default::default()"</td>
                            <td>"Tabbar's value."</td>
                        </tr>
                        <tr>
                            <td>"children"</td>
                            <td>"Children"</td>
                            <td></td>
                            <td>"Tabbar's content."</td>
                        </tr>
                    </tbody>
                </Table>
                <h3>"TabbarItem Props"</h3>
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
                            <td>"key"</td>
                            <td>"MaybeSignal<String>"</td>
                            <td>"Default::default()"</td>
                            <td>"The indentifier of the tabbar item."</td>
                        </tr>
                        <tr>
                            <td>"icon"</td>
                            <td>"Option<Icon>"</td>
                            <td>"None"</td>
                            <td>"TabbarItem's icon."</td>
                        </tr>
                        <tr>
                            <td>"children"</td>
                            <td>"Children"</td>
                            <td></td>
                            <td>"TabbarItem's content."</td>
                        </tr>
                    </tbody>
                </Table>
            </div>
            <div>
                <MobilePage path="/thaw?path=/mobile/tabbar"/>
            </div>
        </div>
    }
}

#[component]
pub fn TabbarDemoPage() -> impl IntoView {
    let value = create_rw_signal(String::from("o"));
    view! {
        <div style="height: 100vh;">
            {move || value.get()} <Tabbar value>
                <TabbarItem key="a">"and"</TabbarItem>
                <TabbarItem key="i">"if"</TabbarItem>
                <TabbarItem key="o" icon=icondata::AiIcon::AiCloseOutlined>
                    "or"
                </TabbarItem>
            </Tabbar>
        </div>
    }
}

use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn MenuPage() -> impl IntoView {
    let value = create_rw_signal(String::from("o"));
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Menu"</h1>
            <Demo>
                <Menu value>
                    <MenuItem key="a" label="and"/>
                    <MenuItem key="o" label="or"/>
                </Menu>
                <DemoCode slot>
{highlight_str!( r#"
                        let value = create_rw_signal(String::from("o"));
                                
                        <Menu value>
                            <MenuItem key="a" label="and"/>
                            <MenuItem key="o" label="or"/>
                        </Menu>
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Menu Props"</h3>
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
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"The selected item key of the menu."</td>
                    </tr>
                    <tr>
                        <td>"children"</td>
                        <td>"Children"</td>
                        <td></td>
                        <td>"Menu's content."</td>
                    </tr>
                    <tr>
                        <td>"class"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Addtional classes for the menu element."</td>
                    </tr>
                </tbody>
            </Table>
            <h3>"MenuGroup Props"</h3>
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
                        <td>"label"</td>
                        <td>"String"</td>
                        <td></td>
                        <td>"The label of the menu group."</td>
                    </tr>
                    <tr>
                        <td>"children"</td>
                        <td>"Children"</td>
                        <td></td>
                        <td>"MenuGroup's content."</td>
                    </tr>
                    <tr>
                        <td>"class"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Addtional classes for the menu group element."</td>
                    </tr>
                </tbody>
            </Table>
            <h3>"MenuItem Props"</h3>
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
                        <td>"label"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"The label of the menu item."</td>
                    </tr>
                    <tr>
                        <td>"key"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"The indentifier of the menu item."</td>
                    </tr>
                    <tr>
                        <td>"class"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Addtional classes for the menu item element."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}

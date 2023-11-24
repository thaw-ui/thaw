use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn ImagePage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Image"</h1>
            <Demo>
                <Image src="https://s3.bmp.ovh/imgs/2021/10/2c3b013418d55659.jpg" width="500px"/>
                <Image width="200px" height="200px"/>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        <Image src="https://s3.bmp.ovh/imgs/2021/10/2c3b013418d55659.jpg" width="500px"/>
                        <Image width="200px" height="200px"/>
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Image Props"</h3>
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
                        <td>"src"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Image source."</td>
                    </tr>
                    <tr>
                        <td>"alt"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Image alt information."</td>
                    </tr>
                    <tr>
                        <td>"width"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Image width."</td>
                    </tr>
                    <tr>
                        <td>"height"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Image height."</td>
                    </tr>
                    <tr>
                        <td>"border_radius"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Image border radius."</td>
                    </tr>
                    <tr>
                        <td>"object_fit"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Object-fit type of the image in the container."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}

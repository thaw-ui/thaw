use crate::{
    components::{Demo, DemoCode},
    pages::MobilePage,
};
use leptos::*;
use prisms::highlight_str;
use thaw::mobile::NavBar;
use thaw::Table;

#[component]
pub fn NavBarPage() -> impl IntoView {
    view! {
        <div style="display: flex">
            <div style="width: 896px; margin: 0 auto;">
                <h1>"Navbar"</h1>
                <Demo>
                    ""
                    <DemoCode
                        slot
                        html=highlight_str!(
                            r#"
                            let click_text = create_rw_signal(String::from("none"));
                            let on_click_left = move |_| click_text.set("left".to_string());
                            let on_click_right = move |_| click_text.set("right".to_string());
                        
                            view! {
                                <div style="height: 100vh; background: #f5f5f5">
                                    <NavBar
                                        title="Home"
                                        left_arrow=true
                                        left_text="back"
                                        right_text="add"
                                        on_click_left=on_click_left
                                        on_click_right=on_click_right
                                    />
                                    <div style="padding-top: 50px">{move || click_text.get()}</div>
                                </div>
                            }
                    "#,
                            "rust"
                        )
                    >

                        ""
                    </DemoCode>
                </Demo>
                <h3>"NavBar Props"</h3>
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
                            <td>"title"</td>
                            <td>"MaybeSignal<String>"</td>
                            <td>r#""""#</td>
                            <td>"NavBar title."</td>
                        </tr>
                        <tr>
                            <td>"left_arrow"</td>
                            <td>"MaybeSignal<bool>"</td>
                            <td></td>
                            <td>"Whether to show left arrow."</td>
                        </tr>
                        <tr>
                            <td>"left_text"</td>
                            <td>"MaybeSignal<String>"</td>
                            <td>r#""""#</td>
                            <td>"NavBar left text."</td>
                        </tr>
                        <tr>
                            <td>"on_click_left"</td>
                            <td>"MaybeSignal<String>"</td>
                            <td>r#""""#</td>
                            <td>"NavBar left click."</td>
                        </tr>
                        <tr>
                            <td>"right_text"</td>
                            <td>"MaybeSignal<String>"</td>
                            <td>r#""""#</td>
                            <td>"NavBar right text."</td>
                        </tr>
                        <tr>
                            <td>"on_click_right"</td>
                            <td>"MaybeSignal<String>"</td>
                            <td>r#""""#</td>
                            <td>"NavBar right click."</td>
                        </tr>
                    </tbody>
                </Table>
            </div>
            <div>
                <MobilePage path="/thaw?path=/mobile/nav-bar"/>
            </div>
        </div>
    }
}

#[component]
pub fn NavBarDemoPage() -> impl IntoView {
    let click_text = create_rw_signal(String::from("none"));
    let on_click_left = move |_| click_text.set("left".to_string());
    let on_click_right = move |_| click_text.set("right".to_string());

    view! {
        <div style="height: 100vh;">
            <NavBar
                title="Home"
                left_arrow=true
                left_text="back"
                right_text="add"
                on_click_left=on_click_left
                on_click_right=on_click_right
            />
            <div style="padding-top: 50px">{move || click_text.get()}</div>
        </div>
    }
}

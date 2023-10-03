use crate::{
    components::{Demo, DemoCode},
    pages::MobilePage,
};
use leptos::*;
use melt_ui::mobile::NavBar;
use prisms::highlight_str;

#[component]
pub fn NavBarPage() -> impl IntoView {
    view! {
        <div style="display: flex">
            <div style="width: 896px; margin: 0 auto;">
                <h1>"Navbar"</h1>
                <Demo>
                    ""
                    <DemoCode slot html=highlight_str!(r#"
                        <NavBar 
                            title="Home" 
                            left_arrow=true 
                            left_text="back" 
                            right_text="add" 
                            click_left=click_left 
                            click_right=click_right
                        />
                    "#, "rust")>
                        ""
                    </DemoCode>
                </Demo>
            </div>
            <div>
                <MobilePage path="/melt-ui?path=/mobile/nav-bar" />
            </div>
        </div>
    }
}

#[component]
pub fn NavBarDemoPage() -> impl IntoView {
    let click_text = create_rw_signal(String::from("none"));

    let click_left = SignalSetter::map(move |_| click_text.set("left".to_string()));

    let click_right = SignalSetter::map(move |_| click_text.set("right".to_string()));

    view! {
        <div style="height: 100vh; background: #f5f5f5">
            <NavBar title="Home" left_arrow=true left_text="back" right_text="add" click_left=click_left click_right=click_right/>
            <div style="padding-top: 50px">
                { move || click_text.get() }
            </div>
        </div>
    }
}

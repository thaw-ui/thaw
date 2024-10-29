use super::NavBarMdPage;
use crate::pages::MobilePage;
use leptos::*;
use thaw::mobile::{NavBar, NavBarRight};
use thaw::Icon;


#[component]
pub fn NavBarPage() -> impl IntoView {
    view! {
        <div style="display: flex">
            <NavBarMdPage />
            <div>
                <MobilePage path="/?path=/mobile/nav-bar" />
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
                on_click_left=on_click_left
                on_click_right=on_click_right
            >
                <NavBarRight slot>
                    <Icon icon=icondata::AiCloseOutlined />
                </NavBarRight>
            </NavBar>
            <div style="padding-top: 50px">{move || click_text.get()}</div>
        </div>
    }
}

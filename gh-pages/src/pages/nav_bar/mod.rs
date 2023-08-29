use leptos::*;
use melt_ui::mobile::NavBar;

#[component]
pub fn NavBarPage() -> impl IntoView {
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

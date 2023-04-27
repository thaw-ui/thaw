use crate::utils::mount_style::mount_style;
use leptos::*;
use stylers::style_sheet_str;

#[component]
pub fn Wave(cx: Scope, children: Children) -> impl IntoView {
    let class_name = mount_style("wave", || style_sheet_str!("./src/wave/wave.css"));
    let (css_vars, set_css_vars) = create_signal(cx, String::new());
    let wave_ref = create_node_ref::<html::Div>(cx);
    wave_ref.on_load(cx, move |wave| {
        wave.on(ev::mousedown, move |ev| {
            wave_ref.on_load(cx, move |wave| {
                let rect = wave.get_bounding_client_rect();
                let client_x = f64::from(ev.client_x());
                let client_y = f64::from(ev.client_y());
                set_css_vars.set(format!(
                    "--x: {}px; --y: {}px",
                    client_x - rect.left(),
                    client_y - rect.top()
                ));
            })
        });
    });
    view! {
        cx, class=class_name,
        <div class="melt-wave" ref=wave_ref style=move || css_vars.get()>
            { children(cx) }
        </div>
    }
}

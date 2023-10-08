use crate::utils::mount_style::mount_style;
use leptos::*;

#[component]
pub fn Wave(children: Children) -> impl IntoView {
    mount_style("wave", include_str!("./wave.css"));
    let (css_vars, set_css_vars) = create_signal(String::new());
    let wave_ref = create_node_ref::<html::Div>();
    wave_ref.on_load(move |wave| {
        wave.on(ev::mousedown, move |ev| {
            wave_ref.on_load(move |wave| {
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
        <div class="melt-wave" ref=wave_ref style=move || css_vars.get()>
            {children()}
        </div>
    }
}

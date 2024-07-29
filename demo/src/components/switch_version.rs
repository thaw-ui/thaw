use leptos::prelude::*;
use thaw::*;

#[component]
pub fn SwitchVersion() -> impl IntoView {
    let options = vec![
        ("main", "https://thawui.vercel.app"),
        ("0.3.0", "https://thaw-gxcwse9r5-thaw.vercel.app"),
        ("0.2.6", "https://thaw-mzh1656cm-thaw.vercel.app"),
        ("0.2.5", "https://thaw-8og1kv8zs-thaw.vercel.app"),
    ];

    cfg_if::cfg_if! {
        if #[cfg(any(feature = "csr", feature = "hydrate"))] {
            let location = window().location();
            let origin = location.origin().ok();
            let version = RwSignal::new(origin);
            let _ = version.watch(move |origin| {
                if let Some(origin) = origin {
                    let pathname = location.pathname().unwrap_or_default();
                    let href = format!("{}{}", origin, pathname);
                    let _ = location.set_href(&href);
                }
            });
        } else {
            // let version = RwSignal::new(None::<String>);
        }
    }

    view! {
        <Combobox>
            {
                options.into_iter().map(|option| view! {
                    <ComboboxOption value=option.1 text=option.0 />
                }).collect_view()
            }
        </Combobox>
    }
}

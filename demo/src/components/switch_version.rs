use leptos::prelude::*;
use thaw::*;

#[component]
pub fn SwitchVersion() -> impl IntoView {
    let options = vec![
        ("main", "https://thawui.vercel.app"),
        ("0.3.3", "https://thaw-85fsrigp0-thaw.vercel.app"),
        ("0.3.2", "https://thaw-czldv7au5-thaw.vercel.app"),
        ("0.3.1", "https://thaw-bwh2r7eok-thaw.vercel.app"),
        ("0.3.0", "https://thaw-gxcwse9r5-thaw.vercel.app"),
        ("0.2.6", "https://thaw-mzh1656cm-thaw.vercel.app"),
        ("0.2.5", "https://thaw-8og1kv8zs-thaw.vercel.app"),
    ];

    let version = RwSignal::new(None::<String>);
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        let location = window().location();
        let origin = location.origin().ok();
        version.set(origin);
        let _ = version.watch(move |origin| {
            if let Some(origin) = origin {
                let pathname = location.pathname().unwrap_or_default();
                let href = format!("{}{}", origin, pathname);
                let _ = location.set_href(&href);
            }
        });
    }

    view! {
        <Combobox selected_options=version placeholder="Switch version">
            {
                options.into_iter().map(|option| view! {
                    <ComboboxOption value=option.1 text=option.0 />
                }).collect_view()
            }
        </Combobox>
    }
}

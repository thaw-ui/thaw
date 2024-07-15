use leptos::*;
use thaw::*;

#[component]
pub fn SwitchVersion() -> impl IntoView {
    let options = vec![
        SelectOption::new("main", "https://thawui.vercel.app".into()),
        SelectOption::new("0.3.3", "https://thaw-npgo2zdtv-thaw.vercel.app".into()),
        SelectOption::new("0.3.2", "https://thaw-czldv7au5-thaw.vercel.app".into()),
        SelectOption::new("0.3.1", "https://thaw-bwh2r7eok-thaw.vercel.app".into()),
        SelectOption::new("0.3.0", "https://thaw-gxcwse9r5-thaw.vercel.app".into()),
        SelectOption::new("0.2.6", "https://thaw-mzh1656cm-thaw.vercel.app".into()),
        SelectOption::new("0.2.5", "https://thaw-8og1kv8zs-thaw.vercel.app".into()),
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
            let version = RwSignal::new(None::<String>);
        }
    }

    view! {
        <Select value=version options/>
    }
}

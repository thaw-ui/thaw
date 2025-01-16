use leptos::prelude::*;
use thaw::*;

#[component]
pub fn SwitchVersion() -> impl IntoView {
    let options = RwSignal::new(Vec::<(String, String)>::new());
    let version_url = RwSignal::new(None::<String>);
    let label = RwSignal::new(String::new());
    Effect::watch(
        move || version_url.get(),
        move |url, prev_url, _| {
            let Some(Some(prev_url)) = prev_url else {
                return;
            };
            let Some(url) = url else {
                return;
            };

            if url != prev_url {
                let location = window().location();
                let pathname = location.pathname().unwrap_or_default();
                let href = format!("{}{}", url, pathname);
                let _ = location.set_href(&href);
            }
        },
        false,
    );

    #[cfg(any(feature = "csr", feature = "hydrate"))]
    leptos::task::spawn_local(async move {
        use gloo_net::http::Request;
        let Ok(res) = Request::get(
            "https://raw.githubusercontent.com/thaw-ui/thaw/refs/heads/main/version-site.txt",
        )
        .send()
        .await
        else {
            return;
        };

        let Ok(text) = res.text().await else {
            return;
        };

        let ops: Vec<_> = text
            .split('\n')
            .filter_map(|line| {
                let Some((v, url)) = line.trim().split_once('=') else {
                    return None;
                };

                Some((v.trim().to_string(), url.trim().to_string()))
            })
            .collect();
        options.set(ops);

        let location = window().location();
        let origin = location.origin().ok();
        if let Some(origin) = origin {
            {
                let origin = origin.clone();
                options.with_untracked(move |options| {
                    if let Some(item) = options.iter().find(|item| item.1 == origin) {
                        label.set(item.0.to_string());
                    }
                });
            }
            version_url.set(Some(origin));
        }
    });

    view! {
        <Combobox value=label selected_options=version_url placeholder="Switch version">
            {move || {
                options
                    .get()
                    .into_iter()
                    .map(|option| view! { <ComboboxOption value=option.1 text=option.0 /> })
                    .collect_view()
            }}
        </Combobox>
    }
}

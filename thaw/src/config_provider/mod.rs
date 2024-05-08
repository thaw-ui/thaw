use crate::Theme;
use leptos::*;
use thaw_utils::mount_style;

#[component]
pub fn ConfigProvider(
    /// Sets the theme used in a scope.
    #[prop(optional, into)]
    theme: Option<RwSignal<Theme>>,
    /// Sets the direction of text & generated styles.
    #[prop(optional, into)]
    dir: RwSignal<Option<ConfigDirection>>,
    children: Children,
) -> impl IntoView {
    mount_style("config-provider", include_str!("./config-provider.css"));

    let theme = theme.unwrap_or_else(|| RwSignal::new(Theme::light()));
    let css_vars = Memo::new(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            theme.common.write_css_vars(&mut css_vars);
            theme.color.write_css_vars(&mut css_vars);
        });
        css_vars
    });

    let config_injection = ConfigInjection {
        theme,
        dir: dir.clone(),
    };

    view! {
        <Provider value=config_injection>
            <div
                class="thaw-config-provider"
                style=move || css_vars.get()
                dir=move || dir.get().map(move |dir| dir.as_str())
            >
                {children()}
            </div>
        </Provider>
    }
}

#[derive(Clone)]
pub struct ConfigInjection {
    theme: RwSignal<Theme>,
    dir: RwSignal<Option<ConfigDirection>>,
}

#[derive(Clone)]
pub enum ConfigDirection {
    Ltr,
    Rtl,
    Auto,
}

impl ConfigDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConfigDirection::Ltr => "ltr",
            ConfigDirection::Rtl => "rtl",
            ConfigDirection::Auto => "auto",
        }
    }
}

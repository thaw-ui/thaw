use crate::Theme;
use leptos::{context::Provider, prelude::*};
use thaw_utils::{mount_dynamic_style, mount_style};

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
    let id = StoredValue::new(uuid::Uuid::new_v4().to_string());

    mount_dynamic_style(id.get_value(), move || {
        let mut css_vars = String::new();
        theme.with(|theme| {
            theme.common.write_css_vars(&mut css_vars);
            theme.color.write_css_vars(&mut css_vars);
        });
        format!(
            ".thaw-config-provider[data-thaw-id=\"{}\"]{{{css_vars}}}",
            id.get_value()
        )
    });

    on_cleanup(move || {
        if let Ok(Some(style)) =
            document().query_selector(&format!("head style[data-thaw-id=\"{}\"]", id.get_value()))
        {
            style.remove();
        }
    });

    let config_injection = ConfigInjection {
        theme,
        dir,
        id: id.get_value(),
    };

    view! {
        <Provider value=config_injection>
            <div
                class="thaw-config-provider"
                data-thaw-id=id.get_value()
                dir=move || dir.get().map(move |dir| dir.as_str())
            >
                {children()}
            </div>
        </Provider>
    }
}

#[derive(Clone)]
pub struct ConfigInjection {
    pub theme: RwSignal<Theme>,
    pub dir: RwSignal<Option<ConfigDirection>>,
    id: String,
}

impl ConfigInjection {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn use_() -> ConfigInjection {
        expect_context()
    }
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

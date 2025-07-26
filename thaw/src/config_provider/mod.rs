use crate::Theme;
use leptos::{context::Provider, prelude::*};
use thaw_utils::class_list;

#[cfg(feature = "manganis")]
const _: manganis::Asset = manganis::asset!("/src/config_provider/config-provider.css");

#[component]
pub fn ConfigProvider(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Sets the theme used in a scope.
    #[prop(optional, into)]
    theme: Option<RwSignal<Theme>>,
    /// Theme id
    #[prop(optional, into)]
    theme_id: Option<String>,
    /// Sets the direction of text & generated styles.
    #[prop(optional, into)]
    dir: Option<RwSignal<ConfigDirection>>,
    children: Children,
) -> impl IntoView {
    #[cfg(not(feature = "manganis"))]
    thaw_utils::mount_style("config-provider", include_str!("./config-provider.css"));

    let theme = theme.unwrap_or_else(|| RwSignal::new(Theme::light()));
    let theme_id = theme_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let id = StoredValue::new(theme_id);
    let css_vars = move || {
        let mut css_vars = String::new();
        theme.with(|theme| {
            theme.common.write_css_vars(&mut css_vars);
            theme.color.write_css_vars(&mut css_vars);
        });
        format!(
            ".thaw-config-provider[data-thaw-id=\"{}\"]{{{css_vars}}}",
            id.get_value(),
        )
    };

    #[cfg(not(feature = "manganis"))]
    thaw_utils::mount_dynamic_style(id.get_value(), css_vars);

    #[cfg(not(feature = "ssr"))]
    Owner::on_cleanup(move || {
        if let Ok(Some(style)) =
            document().query_selector(&format!("head style[data-thaw-id=\"{}\"]", id.get_value()))
        {
            style.remove();
        }
    });

    let config_injection = ConfigInjection { theme, dir, id };

    view! {
        <Provider value=config_injection>
            {
                #[cfg(feature = "manganis")]
                {
                    view! {
                        <leptos_meta::Style id=id.get_value()>
                            {css_vars}
                        </leptos_meta::Style>
                    }
                }
            }
            <div
                class=class_list!["thaw-config-provider", class]
                data-thaw-id=id.get_value()
                dir=move || dir.map(move |dir| dir.get().as_str())
            >
                {children()}
            </div>
        </Provider>
    }
}

#[derive(Clone, Copy)]
pub struct ConfigInjection {
    pub theme: RwSignal<Theme>,
    pub dir: Option<RwSignal<ConfigDirection>>,
    id: StoredValue<String>,
}

impl ConfigInjection {
    pub fn id(&self) -> String {
        self.id.get_value()
    }

    pub fn expect_context() -> Self {
        expect_context()
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ConfigDirection {
    Ltr,
    Rtl,
    #[default]
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

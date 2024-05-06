use crate::Theme;
use leptos::*;
use thaw_utils::mount_style;

#[component]
pub fn ConfigProvider(
    #[prop(optional, into)] theme: Option<RwSignal<Theme>>,
    children: Children,
) -> impl IntoView {
    mount_style("config-provider", include_str!("./config-provider.css"));

    let theme = theme.unwrap_or_else(|| RwSignal::new(Theme::light()));
    let css_vars = Memo::new(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            // css_vars.push_str(&format!(
            //     "--fontFamilyBase: {};",
            //     theme.common.font_family_base
            // ));
            // css_vars.push_str(&format!(
            //     "--fontSizeBase300: {};",
            //     theme.common.font_size_base_300
            // ));
            // css_vars.push_str(&format!(
            //     "--lineHeightBase300: {};",
            //     theme.common.line_height_base300,
            // ));
            // css_vars.push_str(&format!(
            //     "--fontWeightRegular: {};",
            //     theme.common.font_weight_regular
            // ));
            // css_vars.push_str(&format!(
            //     "--fontWeightSemibold: {};",
            //     theme.common.font_weight_semibold
            // ));
            // css_vars.push_str(&format!(
            //     "--fontWeightBold: {};",
            //     theme.common.font_weight_bold
            // ));
            // css_vars.push_str(&format!(
            //     "--strokeWidthThin: {};",
            //     theme.common.stroke_width_thin,
            // ));
            // css_vars.push_str(&format!(
            //     "--borderRadiusMedium: {};",
            //     theme.common.border_radius_medium
            // ));
            // css_vars.push_str(&format!(
            //     "--spacingHorizontalM: {};",
            //     theme.common.spacing_horizontal_m
            // ));

            // css_vars.push_str(&format!(
            //     "--durationFaster: {};",
            //     theme.common.duration_faster
            // ));
            // css_vars.push_str(&format!(
            //     "--curveEasyEase: {};",
            //     theme.common.curve_easy_ease
            // ));
            theme.common.write_css_vars(&mut css_vars);
            theme.color.write_css_vars(&mut css_vars);
        });
        css_vars
    });

    let config_injection = ConfigInjection { theme };

    view! {
        <Provider value=config_injection>
            <div style=move || css_vars.get() class="thaw-config-provider">
                {children()}
            </div>
        </Provider>
    }
}

#[derive(Clone)]
pub struct ConfigInjection {
    theme: RwSignal<Theme>,
}

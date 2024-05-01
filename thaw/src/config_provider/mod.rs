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
            css_vars.push_str(&format!(
                "--fontFamilyBase: {};",
                theme.common.font_family_base
            ));
            css_vars.push_str(&format!(
                "--fontSizeBase300: {};",
                theme.common.font_size_base_300
            ));
            css_vars.push_str(&format!(
                "--lineHeightBase300: {};",
                theme.common.line_height_base300,
            ));
            css_vars.push_str(&format!(
                "--fontWeightRegular: {};",
                theme.common.font_weight_regular
            ));
            css_vars.push_str(&format!(
                "--fontWeightSemibold: {};",
                theme.common.font_weight_semibold
            ));
            css_vars.push_str(&format!(
                "--fontWeightBold: {};",
                theme.common.font_weight_bold
            ));
            css_vars.push_str(&format!(
                "--strokeWidthThin: {};",
                theme.common.stroke_width_thin,
            ));
            css_vars.push_str(&format!(
                "--borderRadiusMedium: {};",
                theme.common.border_radius_medium
            ));
            css_vars.push_str(&format!(
                "--spacingHorizontalM: {};",
                theme.common.spacing_horizontal_m
            ));

            css_vars.push_str(&format!(
                "--durationFaster: {};",
                theme.common.duration_faster
            ));
            css_vars.push_str(&format!(
                "--curveEasyEase: {};",
                theme.common.curve_easy_ease
            ));

            css_vars.push_str(&format!(
                "--colorNeutralBackground1: {};",
                theme.common.color_neutral_background_1
            ));
            css_vars.push_str(&format!(
                "--colorNeutralBackground1Hover: {};",
                theme.common.color_neutral_background_1_hover
            ));
            css_vars.push_str(&format!(
                "--colorNeutralBackground1Pressed: {};",
                theme.common.color_neutral_background_1_pressed
            ));

            css_vars.push_str(&format!(
                "--colorNeutralForeground1: {};",
                theme.common.color_neutral_foreground_1
            ));
            css_vars.push_str(&format!(
                "--colorNeutralForeground1Hover: {};",
                theme.common.color_neutral_foreground_1_hover
            ));
            css_vars.push_str(&format!(
                "--colorNeutralForeground1Pressed: {};",
                theme.common.color_neutral_foreground_1_pressed
            ));
            css_vars.push_str(&format!(
                "--colorNeutralForeground2: {};",
                theme.common.color_neutral_foreground_2
            ));
            css_vars.push_str(&format!(
                "--colorNeutralForeground2Hover: {};",
                theme.common.color_neutral_foreground_2_hover
            ));
            css_vars.push_str(&format!(
                "--colorNeutralForeground2Pressed: {};",
                theme.common.color_neutral_foreground_2_pressed
            ));
            css_vars.push_str(&format!(
                "--colorNeutralForeground2BrandHover: {};",
                theme.common.color_neutral_foreground_2_brand_hover
            ));
            css_vars.push_str(&format!(
                "--colorNeutralForeground2BrandPressed: {};",
                theme.common.color_neutral_foreground_2_brand_pressed
            ));
            css_vars.push_str(&format!(
                "--colorNeutralForegroundOnBrand: {};",
                theme.common.color_neutral_foreground_on_brand
            ));
            css_vars.push_str(&format!(
                "--colorNeutralStroke1: {};",
                theme.common.color_neutral_stroke_1
            ));
            css_vars.push_str(&format!(
                "--colorNeutralStroke1Hover: {};",
                theme.common.color_neutral_stroke_1_hover
            ));
            css_vars.push_str(&format!(
                "--colorNeutralStroke1Pressed: {};",
                theme.common.color_neutral_stroke_1_pressed
            ));
            css_vars.push_str(&format!(
                "--colorBrandBackground: {};",
                theme.common.color_brand_background
            ));
            css_vars.push_str(&format!(
                "--colorBrandBackgroundHover: {};",
                theme.common.color_brand_background_hover
            ));
            css_vars.push_str(&format!(
                "--colorBrandBackgroundPressed: {};",
                theme.common.color_brand_background_pressed
            ));
            css_vars.push_str(&format!(
                "--colorSubtleBackground: {};",
                theme.common.color_subtle_background
            ));
            css_vars.push_str(&format!(
                "--colorSubtleBackgroundHover {};",
                theme.common.color_subtle_background_hover
            ));
            css_vars.push_str(&format!(
                "--colorSubtleBackgroundPressed: {};",
                theme.common.color_subtle_background_pressed
            ));
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

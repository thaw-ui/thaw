mod grid_item;

#[cfg(not(feature = "ssr"))]
use crate::utils::dyn_classes;
use crate::utils::ssr_class;

pub use grid_item::*;
use leptos::*;

#[component]
pub fn Grid(
    #[prop(default = MaybeSignal::Static(1u16), into)] cols: MaybeSignal<u16>,
    #[prop(optional, into)] x_gap: MaybeSignal<u16>,
    #[prop(optional, into)] y_gap: MaybeSignal<u16>,
    #[prop(optional, into)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let style = create_memo(move |_| {
        let mut style = String::from("display: grid;");
        style.push_str(&format!(
            "grid-template-columns: repeat({}, minmax(0px, 1fr));",
            cols.get()
        ));
        style.push_str(&format!("grid-gap: {}px {}px;", y_gap.get(), x_gap.get()));
        style
    });

    let ssr_class = ssr_class(&class);
    view! {
        <Provider value=GridInjection::new(x_gap)>
            <div class=ssr_class use:dyn_classes=class class="thaw-grid" style=move || style.get()>
                {children()}
            </div>
        </Provider>
    }
}

#[derive(Clone)]
pub(crate) struct GridInjection {
    x_gap: MaybeSignal<u16>,
}

impl GridInjection {
    pub fn new(x_gap: MaybeSignal<u16>) -> Self {
        Self { x_gap }
    }
}

pub(crate) fn use_grid() -> GridInjection {
    expect_context()
}

mod grid_item;

pub use grid_item::*;
use leptos::*;

#[component]
fn Grid(
    #[prop(default = MaybeSignal::Static(1u16), into)] cols: MaybeSignal<u16>,
    #[prop(optional, into)] x_gap: MaybeSignal<i32>,
    #[prop(optional, into)] y_gap: MaybeSignal<i32>,
    children: Children,
) -> impl IntoView {
    let grid_injection_key = create_rw_signal(GridInjectionKey::new(x_gap));
    provide_context(grid_injection_key);

    let style = create_memo(move |_| {
        let mut style = String::from("display: grid;");
        style.push_str(&format!(
            "grid-template-columns: repeat({}, minmax(0px, 1fr));",
            cols.get()
        ));
        style.push_str(&format!("grid-gap: {}px ${}px;", y_gap.get(), x_gap.get()));
        style
    });

    view! {
        <div class="melt-grid" style=move || style.get()>
            {children()}
        </div>
    }
}

#[derive(Clone)]
pub struct GridInjectionKey {
    x_gap: MaybeSignal<i32>,
}

impl GridInjectionKey {
    pub fn new(x_gap: MaybeSignal<i32>) -> Self {
        Self { x_gap }
    }
}

pub fn use_grid() -> GridInjectionKey {
    expect_context::<GridInjectionKey>()
}

use super::use_grid;
use leptos::*;

#[component]
pub fn GridItem(
    #[prop(default = MaybeSignal::Static(1u16), into)] span: MaybeSignal<u16>,
    #[prop(optional, into)] offset: MaybeSignal<i32>,
    children: Children,
) -> impl IntoView {
    let grid = use_grid();

    let style = create_memo(move |_| {
        let mut style = String::new();
        let offset = offset.get();
        let span = i32::from(span.get());
        let x_gap = grid.x_gap.get();

        if offset > 0 {
            style.push_str(&format!(
                "margin-left: calc((100% - {}px) / {} * {} + {}px);",
                (span + offset - 1) * x_gap,
                span + offset,
                offset,
                offset * x_gap
            ));
        }
        style.push_str(&format!(
            "grid-column: span {} / span {};",
            span + offset,
            span + offset
        ));

        style
    });
    view! {
        <div class="thaw-grid-item" style=move || style.get()>
            {children()}
        </div>
    }
}

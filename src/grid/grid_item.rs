use super::use_grid;
use leptos::*;

#[component]
pub fn GridItem(
    #[prop(default = MaybeSignal::Static(1u16), into)] column: MaybeSignal<u16>,
    #[prop(optional, into)] offset: MaybeSignal<u16>,
    children: Children,
) -> impl IntoView {
    let grid = use_grid();

    let style = create_memo(move |_| {
        let mut style = String::new();
        let offset = offset.get();
        let column = column.get();
        let x_gap = grid.x_gap.get();

        if offset > 0 {
            style.push_str(&format!(
                "margin-left: calc((100% - {}px) / {} * {} + {}px);",
                (column + offset - 1) * x_gap,
                column + offset,
                offset,
                offset * x_gap
            ));
        }
        style.push_str(&format!(
            "grid-column: span {} / span {};",
            column + offset,
            column + offset
        ));

        style
    });
    view! {
        <div class="thaw-grid-item" style=move || style.get()>
            {children()}
        </div>
    }
}

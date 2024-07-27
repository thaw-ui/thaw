use super::use_grid;
use leptos::prelude::*;
use thaw_utils::class_list;

#[component]
pub fn GridItem(
    /// The number of columns occupied by the grid. The grid item would be hidden if it's 0.
    #[prop(default = MaybeSignal::Static(1u16), into)] column: MaybeSignal<u16>,
    /// The number of intervals to the left of the grid.
    #[prop(optional, into)] offset: MaybeSignal<u16>,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let grid = use_grid();

    let style = Memo::new(move |_| {
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
        <div
            class=class_list!["thaw-grid-item", class]
            style=move || style.get()
        >
            {children()}
        </div>
    }
}

use leptos::prelude::*;
use thaw_components::OptionComp;
use thaw_utils::{class_list, mount_style};

#[component]
pub fn Table(
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    mount_style("table", include_str!("./table.css"));

    view! {
        <table class=class_list!["thaw-table", class]>
            {children()}
        </table>
    }
}

#[component]
pub fn TableHeader(children: Children) -> impl IntoView {
    view! {
        <thead class="thaw-table-header">
            {children()}
        </thead>
    }
}

#[component]
pub fn TableHeaderCell(#[prop(optional)] children: Option<Children>) -> impl IntoView {
    view! {
        <th class="thaw-table-header-cell">
            <button class="thaw-table-header-cell__button" role="presentation">
                <OptionComp value=children let:children>
                    {children()}
                </OptionComp>
            </button>
        </th>
    }
}

#[component]
pub fn TableBody(children: Children) -> impl IntoView {
    view! {
        <tbody class="thaw-table-body">
            {children()}
        </tbody>
    }
}

#[component]
pub fn TableRow(children: Children) -> impl IntoView {
    view! {
        <tr class="thaw-table-row">
            {children()}
        </tr>
    }
}

#[component]
pub fn TableCell(#[prop(optional)] children: Option<Children>) -> impl IntoView {
    view! {
        <td class="thaw-table-cell">
            <OptionComp value=children let:children>
                {children()}
            </OptionComp>
        </td>
    }
}

#[component]
pub fn TableCellLayout(children: Children) -> impl IntoView {
    view! {
        <div class="thaw-table-cell-layout">
            {children()}
        </div>
    }
}

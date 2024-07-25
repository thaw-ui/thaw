use leptos::{either::Either, prelude::*};
use thaw_utils::{class_list, mount_style};

#[component]
pub fn Table(
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
pub fn TableHeader(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <thead class=class_list!["thaw-table-header", class]>
            {children()}
        </thead>
    }
}

#[component]
pub fn TableHeaderCell(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <th class=class_list!["thaw-table-header-cell", class]>
            <button class="thaw-table-header-cell__button" role="presentation">
                {
                    if let Some(children) = children  {
                        Either::Left(children())
                    } else {
                        Either::Right(())
                    }
                }
            </button>
        </th>
    }
}

#[component]
pub fn TableBody(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <tbody class=class_list!["thaw-table-body", class]>
            {children()}
        </tbody>
    }
}

#[component]
pub fn TableRow(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <tr class=class_list!["thaw-table-row", class]>
            {children()}
        </tr>
    }
}

#[component]
pub fn TableCell(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <td class=class_list!["thaw-table-cell", class]>
            {
                if let Some(children) = children  {
                    Either::Left(children())
                } else {
                    Either::Right(())
                }
            }
        </td>
    }
}

#[component]
pub fn TableCellLayout(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=class_list!["thaw-table-cell-layout", class]>
            {children()}
        </div>
    }
}

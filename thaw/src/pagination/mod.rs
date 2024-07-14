use crate::{Button, ButtonSize, ButtonVariant};
use leptos::*;
use std::cmp::min;
use thaw_utils::{class_list, mount_style, Model, OptionalProp, StoredMaybeSignal};

fn range(start: usize, end: usize) -> Vec<PaginationItem> {
    let mut ret = vec![];
    for idx in start..=end {
        ret.push(PaginationItem::Number(idx));
    }
    ret
}

enum PaginationItem {
    DotLeft,
    DotRight,
    Number(usize),
}

fn use_pagination(page: usize, count: usize, sibling_count: usize) -> Vec<PaginationItem> {
    // Pages count is determined as siblingCount + firstPage + lastPage + currentPage + 2*DOTS
    let total_page_numbers = sibling_count + 5;
    // Case 1:
    //       If the number of pages is less than the page numbers we want to show in our
    //       paginationComponent, we return the range [1..totalPageCount]
    if total_page_numbers >= count {
        return range(1, count);
    }
    let current_page = page;
    // Calculate left and right sibling index and make sure they are within range 1 and totalPageCount
    let left_sibling_index = if current_page > sibling_count + 1 {
        current_page - sibling_count
    } else {
        1
    };
    let right_sibling_index = min(current_page + sibling_count, count);
    // We do not show dots just when there is just one page number to be inserted between the extremes of sibling and the page limits i.e 1 and totalPageCount.
    // Hence we are using leftSiblingIndex > 2 and rightSiblingIndex < totalPageCount - 2
    let should_show_left_dots = left_sibling_index > 2;
    let should_show_right_dots = right_sibling_index < count - 2;

    let first_page_index = 1;
    let last_page_index = count;

    // Case 2: No left dots to show, but rights dots to be shown
    if !should_show_left_dots && should_show_right_dots {
        let left_item_count = 3 + 2 * sibling_count;
        let mut left_range = range(1, left_item_count);
        left_range.push(PaginationItem::DotRight);
        left_range.push(PaginationItem::Number(count));
        left_range
    } else if should_show_left_dots && !should_show_right_dots {
        // Case 3: No right dots to show, but left dots to be shown
        let right_item_count = 3 + 2 * sibling_count;
        let mut right_range = range(count - right_item_count + 1, count);
        let mut ret = vec![
            PaginationItem::Number(first_page_index),
            PaginationItem::DotLeft,
        ];
        ret.append(&mut right_range);
        ret
    } else {
        // Case 4: Both left and right dots to be shown
        let mut middle_range = range(left_sibling_index, right_sibling_index);
        let mut range = vec![
            PaginationItem::Number(first_page_index),
            PaginationItem::DotLeft,
        ];
        range.append(&mut middle_range);
        range.append(&mut vec![
            PaginationItem::DotRight,
            PaginationItem::Number(last_page_index),
        ]);
        range
    }
}

#[component]
pub fn Pagination(
    #[prop(default = 1.into(), into)] page: Model<usize>,
    #[prop(into)] count: MaybeSignal<usize>,
    #[prop(default = 1.into(), into)] sibling_count: MaybeSignal<usize>,
    #[prop(optional, into)] on_change: Option<Callback<usize>>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] size: MaybeSignal<ButtonSize>,
) -> impl IntoView {
    mount_style("pagination", include_str!("./pagination.css"));

    let size: StoredMaybeSignal<_> = size.into();
    let no_next = Memo::new(move |_| page.get() == count.get());
    let no_previous = Memo::new(move |_| page.get() == 1);

    let on_click_previous = move |_| {
        page.update(|val| *val -= 1);
        if let Some(callback) = on_change.as_ref() {
            callback.call(page.get())
        }
    };

    let on_click_next = move |_| {
        page.update(|val| *val += 1);
        if let Some(callback) = on_change.as_ref() {
            callback.call(page.get())
        }
    };

    view! {
        <nav class=class_list!["thaw-pagination", class.map(| c | move || c.get())]>
            <ul>
                <li>
                    <Button
                        size=size.clone()
                        on_click=on_click_previous
                        variant=ButtonVariant::Text
                        icon=icondata_ai::AiLeftOutlined
                        disabled=no_previous
                        circle=true
                    />
                </li>

                <For
                    each=move || use_pagination(page.get(), count.get(), sibling_count.get())
                    key=|item| match item {
                        PaginationItem::DotLeft => -2,
                        PaginationItem::DotRight => -1,
                        PaginationItem::Number(nb) => nb.clone() as i64
                    }
                    let:item
                >
                    {
                        if let PaginationItem::Number(nb) = item {
                           view! {
                                <li>
                                    <Button
                                        size=size.clone()
                                        style=Memo::new(move |_| if page.get() == nb {
                                            "color: var(--thaw-font-color-hover); border-color: var(--thaw-border-color-hover);".to_string()
                                        } else {
                                            "".to_string()
                                        })
                                        variant=Memo::new(move |_| if page.get() == nb {
                                            ButtonVariant::Outlined
                                        } else {
                                            ButtonVariant::Text
                                        })
                                        on_click = move |_| {
                                            if page.get() != nb {
                                                page.set(nb)
                                            }
                                            if let Some(callback) = on_change.as_ref() {
                                                callback.call(page.get())
                                            }
                                        }
                                        round=true
                                    >
                                        {nb}
                                    </Button>
                                </li>
                            }
                        } else {
                            view! {
                                <li>"..."</li>
                            }
                        }
                    }
                </For>
                <li>
                    <Button
                        size
                        on_click=on_click_next
                        variant=ButtonVariant::Text
                        icon=icondata_ai::AiRightOutlined disabled=no_next
                        circle=true
                    />
                </li>
            </ul>
        </nav>
    }
}

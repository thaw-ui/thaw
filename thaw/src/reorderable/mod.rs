mod reorderable;

use reorderable::*;

use leptos::{context::Provider, prelude::*};
use std::marker::PhantomData;
use thaw_utils::{class_list, mount_style};

fn default_bookend(_idx: usize) -> ViewFn {
    ViewFn::from(|| view! {})
}

#[component]
pub fn Reorderable<const COLUMNS: usize, PanelFn>(
    panel_order: [RwSignal<Vec<Oco<'static, str>>>; COLUMNS],
    panel_fn: PanelFn,
    #[prop(optional)] horizontal: bool,
    #[prop(optional, into)] container_class: MaybeProp<String>,
    #[prop(optional, into)] panel_class: MaybeProp<String>,
) -> impl IntoView
where
    PanelFn: Fn(Oco<'static, str>) -> ViewFn + Send + 'static + Clone,
{
    view! {
        <CustomReorderable
            panel_order
            panel_fn
            header_fn=default_bookend
            footer_fn=default_bookend
            horizontal
            container_class
            panel_class
        />
    }
}

#[component]
pub fn CustomReorderable<const COLUMNS: usize, PanelFn, HeaderFn, FooterFn>(
    panel_order: [RwSignal<Vec<Oco<'static, str>>>; COLUMNS],
    panel_fn: PanelFn,
    header_fn: HeaderFn,
    footer_fn: FooterFn,
    #[prop(optional)] horizontal: bool,
    #[prop(optional, into)] container_class: MaybeProp<String>,
    #[prop(optional, into)] panel_class: MaybeProp<String>,
) -> impl IntoView
where
    PanelFn: Fn(Oco<'static, str>) -> ViewFn + Send + 'static + Clone,
    HeaderFn: Fn(usize) -> ViewFn + Send + 'static + Clone,
    FooterFn: Fn(usize) -> ViewFn + Send + 'static + Clone,
{
    mount_style("reorderable", include_str!("./reorderable.css"));
    let ctx = ReorderContext::new(panel_order.clone(), horizontal);
    let panel_getter = StoredValue::new_local(panel_fn);
    let header_getter = StoredValue::new_local(header_fn);
    let footer_getter = StoredValue::new_local(footer_fn);
    // dummy_array is a compile-time way to let ColumnnWrapper know COLUMNS.
    let dummy_array: [PhantomData<bool>; COLUMNS] = ctx.column_node_refs.map(|_| PhantomData);
    let column_getter = move || {
        panel_order
            .iter()
            .enumerate()
            .map(|(idx, _)| (idx, dummy_array.clone()))
            .collect::<Vec<(usize, [PhantomData<bool>; COLUMNS])>>()
    };
    view! {
        <div class="reorderable" class=("horizontal", horizontal)>
            <Provider value=ctx>
                <For each=column_getter key=|(idx, _dummy)| idx.to_string() let:value>
                    <PanelContainer
                        index=value.0
                        dummy=value.1
                        panel_getter=panel_getter.clone()
                        class=container_class.clone()
                        header_getter=header_getter.clone()
                        footer_getter=footer_getter.clone()
                        panel_class=panel_class.clone()
                    />
                </For>
            </Provider>
        </div>
    }
}

#[component]
fn PanelContainer<const COLUMNS: usize, PanelFn, HeaderFn, FooterFn>(
    index: usize,
    dummy: [PhantomData<bool>; COLUMNS],
    panel_getter: StoredValue<PanelFn, LocalStorage>,
    header_getter: StoredValue<HeaderFn, LocalStorage>,
    footer_getter: StoredValue<FooterFn, LocalStorage>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] panel_class: MaybeProp<String>,
) -> impl IntoView
where
    PanelFn: Fn(Oco<'static, str>) -> ViewFn + Send + 'static + Clone,
    HeaderFn: Fn(usize) -> ViewFn + Send + 'static + Clone,
    FooterFn: Fn(usize) -> ViewFn + Send + 'static + Clone,
{
    let ctx = expect_context::<ReorderContext<COLUMNS>>();
    let node_ref = ctx.column_node_refs[index];
    let panel_order = ctx.panel_order[index];
    let panel_ids = move || {
        panel_order
            .read()
            .iter()
            .cloned()
            .collect::<Vec<Oco<'static, str>>>()
    };
    view! {
        <div
            node_ref=node_ref
            class=class_list!["reorderable-container", class]
            class=("horizontal", ctx.horizontal)
        >
            <ContainerHeader index horizontal=ctx.horizontal _dummy=dummy.clone()>
                {header_getter.with_value(|header_fn| header_fn(index).run())}
            </ContainerHeader>
            <For each=panel_ids key=|item| item.clone() let:id>
                <PanelWrapper id=id.clone() _dummy=dummy.clone() class=panel_class.clone()>
                    {panel_getter.with_value(|panel_fn| panel_fn(id.clone()).run())}
                </PanelWrapper>
            </For>
            <ContainerFooter horizontal=ctx.horizontal>
                {footer_getter.with_value(|footer_fn| footer_fn(index).run())}
            </ContainerFooter>
        </div>
    }
}

#[component]
fn ContainerHeader<const COLUMNS: usize>(
    index: usize,
    horizontal: bool,
    _dummy: [PhantomData<bool>; COLUMNS],
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = expect_context::<ReorderContext<COLUMNS>>();
    let (node_ref, hover_position) = ctx.generate_header(index);
    view! {
        <div
            node_ref=node_ref
            class="header item"
            class=("row-item", move || horizontal)
            class=("col-item", move || !horizontal)
            class=("item--after", move || hover_position.get().is_some())
        >
            {children()}
        </div>
    }
}

#[component]
fn ContainerFooter(horizontal: bool, children: ChildrenFn) -> impl IntoView {
    view! {
        <div
            class="footer item"
            class=("row-item", move || horizontal)
            class=("col-item", move || !horizontal)
        >
            {children()}
        </div>
    }
}

#[component]
fn PanelWrapper<const COLUMNS: usize>(
    id: Oco<'static, str>,
    _dummy: [PhantomData<bool>; COLUMNS],
    #[prop(optional, into)] class: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = expect_context::<ReorderContext<COLUMNS>>();
    let horizontal = ctx.horizontal;
    let PanelReorderEvents {
        node_ref,
        draggable,
        set_draggable,
        hover_position,
        on_dragstart,
        is_dragging,
        on_dragend,
        ..
    } = ctx.generate_panel(id);
    view! {
        <div
            node_ref=node_ref
            class=class_list!["item", class]
            class=("row-item", move || horizontal)
            class=("col-item", move || !horizontal)
            class=("dragging", move || is_dragging.get())
            class=(
                "item--before",
                move || matches!(hover_position.get(), Some(HoverPosition::Before)),
            )

            class=(
                "item--after",
                move || matches!(hover_position.get(), Some(HoverPosition::After)),
            )

            draggable=move || draggable.get().then_some("true")
            on:dragstart=on_dragstart
            on:dragend=on_dragend
            on:mousedown=move |_| set_draggable(true)
        >
            {children()}
        </div>
    }
}

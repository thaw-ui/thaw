use std::collections::HashMap;

use leptos::{
    ev,
    html::Div,
    prelude::*,
    tachys::dom::event_target,
};
use send_wrapper::SendWrapper;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::js_sys::Function;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HoverInfo {
    column_index: usize,
    panel: Option<HoveredPanel>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HoveredPanel {
    id: Oco<'static, str>,
    position: HoverPosition,
}

#[derive(Clone)]
pub struct ReorderContext<const COLUMNS: usize> {
    pub column_node_refs: [NodeRef<Div>; COLUMNS],
    pub column_refs: [Signal<Option<SendWrapper<web_sys::Element>>>; COLUMNS],
    pub panel_order: [RwSignal<Vec<Oco<'static, str>>>; COLUMNS],
    pub currently_dragged_panel: RwSignal<Option<Oco<'static, str>>>,
    pub hover_info: RwSignal<Option<HoverInfo>>,
    pub panels: RwSignal<HashMap<Oco<'static, str>, SendWrapper<web_sys::Element>>>,
    pub horizontal: bool,
    initialized: RwSignal<bool>,
}

impl<const COLUMNS: usize> ReorderContext<COLUMNS> {
    pub fn new(
        panel_array: [RwSignal<Vec<Oco<'static, str>>>; COLUMNS],
        horizontal: bool,
    ) -> ReorderContext<COLUMNS> {
        let column_node_refs: [NodeRef<Div>; COLUMNS] = panel_array.map(|_| NodeRef::new());
        let column_refs = column_node_refs.map(|column_ref| {
            Signal::derive(move || {
                column_ref
                    .get()
                    .map(|column_ref| SendWrapper::new(column_ref.into()))
            })
        });
        for container in panel_array.iter() {
            for id in container.write_untracked().iter_mut() {
                id.upgrade_inplace();
            }
        }
        let ctx = ReorderContext {
            column_node_refs,
            column_refs,
            horizontal,
            panel_order: panel_array.clone(),
            currently_dragged_panel: RwSignal::new(None),
            hover_info: RwSignal::new(None),
            panels: RwSignal::new(HashMap::new()),
            initialized: RwSignal::new(false),
        };
        ctx
    }

    pub fn initialize(&self) {
        Effect::new(move |mut last_on_dragend: Option<Function>| {
            if let Some(last_on_dragend) = last_on_dragend.take() {
                let _ = document().remove_event_listener_with_callback("dragend", &last_on_dragend);
            }

            let closure_ctx = expect_context::<ReorderContext<COLUMNS>>();
            let on_dragend: Function = Closure::wrap(Box::new(move |_ev: web_sys::MouseEvent| {
                if let Some((currently_dragged_panel, hover_info)) = closure_ctx
                    .currently_dragged_panel
                    .read_untracked()
                    .as_ref()
                    .zip(closure_ctx.hover_info.get_untracked())
                {
                    closure_ctx.reorder_panels(&currently_dragged_panel, hover_info);
                }
            }) as Box<dyn FnMut(_)>)
            .into_js_value()
            .dyn_into()
            .unwrap();

            document()
                .add_event_listener_with_callback("dragend", &on_dragend)
                .unwrap();

            on_cleanup({
                let on_dragend = SendWrapper::new(on_dragend.clone());
                move || {
                    let _ = document()
                        .remove_event_listener_with_callback("dragend", &on_dragend.take());
                }
            });

            on_dragend
        });
        self.initialized.set(true);
    }

    fn reorder_panels(&self, currently_dragged_panel: &str, hover_info: HoverInfo) {
        // Extract hover information
        let HoverInfo {
            column_index: to_col_index,
            panel: maybe_hovered_panel,
        } = hover_info;
        // Initialize variables to store the original position of the dragged panel
        let mut from_col_index = None;
        let mut from_row_index = None;

        // Find the column and row index of the currently dragged panel
        for (col_idx, col_signal) in self.panel_order.iter().enumerate() {
            let col_panels = col_signal.get_untracked();
            if let Some(row_idx) = col_panels
                .iter()
                .position(|panel_id| panel_id.as_str() == currently_dragged_panel)
            {
                from_col_index = Some(col_idx);
                from_row_index = Some(row_idx);
                break;
            }
        }

        // Proceed only if the dragged panel was found
        if let (Some(from_col_index), Some(from_row_index)) = (from_col_index, from_row_index) {
            // Get the target column's panels
            let to_col_signal = self.panel_order[to_col_index];
            let mut to_col_panels = to_col_signal.get_untracked();

            // Determine the insertion index
            let insert_row_index = match maybe_hovered_panel {
                Some(HoveredPanel {
                    id: hovered_panel_id,
                    position: hover_position,
                }) => {
                    // Find the index of the hovered panel in the target column
                    if let Some(hovered_row_index) = to_col_panels
                        .iter()
                        .position(|panel_id| panel_id.as_str() == hovered_panel_id)
                    {
                        // Determine the insertion index based on the hover position
                        let mut idx = match hover_position {
                            HoverPosition::Before => hovered_row_index,
                            HoverPosition::After => hovered_row_index + 1,
                        };

                        // Adjust the insertion index if moving within the same column
                        if from_col_index == to_col_index && from_row_index < idx {
                            idx -= 1;
                        }
                        idx
                    } else {
                        // If hovered panel is not found, insert at the end
                        to_col_panels.len()
                    }
                }
                None => {
                    // No hovered panel; insert at the end of the column
                    to_col_panels.len()
                }
            };

            // Remove the dragged panel from its original position
            let from_col_signal = self.panel_order[from_col_index];
            let mut from_col_panels = from_col_signal.get_untracked();
            from_col_panels.remove(from_row_index);

            if from_col_index == to_col_index {
                // Insert the panel into the same column at the new position
                from_col_panels.insert(
                    insert_row_index,
                    Oco::from(currently_dragged_panel.to_string()),
                );
                from_col_signal.set(from_col_panels);
            } else {
                // Write back the modified original column
                from_col_signal.set(from_col_panels);

                // Insert the panel into the new column
                to_col_panels.insert(
                    insert_row_index,
                    Oco::from(currently_dragged_panel.to_string()),
                );
                to_col_signal.set(to_col_panels);
            }
        }
    }

    /// Registers a panel with drag reordering for a given ID.
    pub fn generate_panel(
        &self,
        id: impl Into<Oco<'static, str>>,
    ) -> PanelReorderEvents<
        impl Fn(bool) + Copy,
        impl Fn(ev::DragEvent) + Clone,
        impl Fn(ev::DragEvent) + Clone,
    > {
        if !self.initialized.get_untracked() {
            self.initialize();
        }
        let mut id: Oco<'static, str> = id.into();
        id.upgrade_inplace();
        let node_ref = NodeRef::<Div>::new();

        let panels = self.panels.clone();
        Effect::new({
            let id = id.clone();
            move |_| match node_ref.get() {
                Some(node_ref) => {
                    panels
                        .write()
                        .insert(id.clone(), SendWrapper::new(node_ref.into()));
                }
                None => {
                    panels.write().remove(&id);
                }
            }
        });

        let panels = self.panels.clone();
        on_cleanup({
            let id = id.clone();
            move || {
                panels.write().remove(&id);
            }
        });

        let is_dragging = Signal::derive({
            let id = id.clone();
            let currently_dragged_panel = self.currently_dragged_panel.clone();
            move || currently_dragged_panel.read().as_deref() == Some(id.as_str())
        });
        let hover_position = Signal::derive({
            let id = id.clone();
            let currently_dragged_panel = self.currently_dragged_panel.clone();
            let panel_order = self.panel_order.clone();
            let hover_info = self.hover_info.clone();
            move || match &*hover_info.read() {
                Some(HoverInfo {
                    panel: Some(panel), ..
                }) => {
                    let currently_dragged_panel = currently_dragged_panel.read();
                    let Some(currently_dragged_panel) = &*currently_dragged_panel else {
                        return None;
                    };

                    let hovering_this_panel = panel.id == id.as_str();
                    let is_currently_dragged_panel = currently_dragged_panel == id.as_str();

                    let currently_dragged_panel_index =
                        panel_order
                            .iter()
                            .enumerate()
                            .find_map(|(column_index, column)| {
                                column
                                    .read()
                                    .iter()
                                    .position(|panel_id| panel_id == currently_dragged_panel)
                                    .map(|pos| (column_index, pos))
                            });
                    let hovering_neighbour_panel =
                        match (currently_dragged_panel_index, panel.position) {
                            (Some((column_index, panel_index)), HoverPosition::Before) => {
                                panel_order
                                    .get(column_index)
                                    .and_then(|column| {
                                        column
                                            .read()
                                            .get(panel_index + 1)
                                            .map(|below_id| below_id.as_str() == id)
                                    })
                                    .unwrap_or(false)
                            }
                            (Some((column_index, panel_index)), HoverPosition::After)
                                if panel_index > 0 =>
                            {
                                panel_order
                                    .get(column_index)
                                    .and_then(|column| {
                                        column
                                            .read()
                                            .get(panel_index - 1)
                                            .map(|below_id| below_id.as_str() == id)
                                    })
                                    .unwrap_or(false)
                            }
                            _ => false,
                        };
                    if hovering_this_panel
                        && !is_currently_dragged_panel
                        && !hovering_neighbour_panel
                    {
                        Some(panel.position)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        });

        let draggable = RwSignal::new(false);
        let set_draggable = move |can_drag: bool| {
            draggable.set(can_drag);
        };

        let on_dragover_cb: RwSignal<Option<Function>, LocalStorage> = RwSignal::new_local(None);

        let on_drag_start = {
            let id = id.clone();
            let currently_dragged_panel = self.currently_dragged_panel.clone();
            let column_refs = self.column_refs.clone();
            let panel_order = self.panel_order.clone();
            let horizontal = self.horizontal;
            let panels = self.panels.clone();
            let hover_info = self.hover_info.clone();
            move |ev: ev::DragEvent| {
                currently_dragged_panel.set(Some(id.clone()));
                let dragged_el = event_target::<web_sys::HtmlElement>(&ev);
                let mouse_x = ev.client_x() as f64;
                let mouse_y = ev.client_y() as f64;
                let rect = dragged_el.get_bounding_client_rect();
                // Calculate the center of the element
                let center_x = rect.x() + rect.width() / 2.0;
                let center_y = rect.y() + rect.height() / 2.0;
                // Calculate the offset from the mouse position to the center of the element
                let offset_x = mouse_x - center_x;
                let offset_y = mouse_y - center_y;

                // Necessary for firefox to emit drag events
                if let Some(data_transfer) = ev.data_transfer() {
                    let _ = data_transfer.set_data("text/plain", &id);
                }

                let on_dragover: Function = Closure::wrap(Box::new(move |ev: web_sys::DragEvent| {
                    ev.prevent_default();

                    let mouse_x = ev.client_x() as f64 - offset_x;
                    let mouse_y = ev.client_y() as f64 - offset_y;

                    let (closest_column, _) = column_refs.iter().enumerate().fold(
                        (None, f64::INFINITY),
                        |(column, closest_dist), (i, column_ref)| {
                            let Some(column_ref) = &*column_ref.read_untracked() else {
                                return (column, closest_dist);
                            };
                            let rect = column_ref.get_bounding_client_rect();
                            let dist = match horizontal {
                                false => {
                                    let center_x = rect.left() + rect.width() / 2.0;
                                    (mouse_x - center_x).abs()
                                }
                                true => {
                                    let center_y = rect.top() + rect.height() / 2.0;
                                    (mouse_y - center_y).abs()
                                }
                            };
                            if dist < closest_dist {
                                (Some((i, column_ref.clone())), dist)
                            } else {
                                (column, closest_dist)
                            }
                        },
                    );

                    if let Some((column_index, _)) = closest_column {
                        let (closest_panel, _) = panels.read_untracked().iter().fold(
                            (None, f64::INFINITY),
                            |(closest_panel, closest_dist), (panel_id, panel_ref)| {
                                let is_in_column = panel_order
                                    .get(column_index)
                                    .map(|column_panels| {
                                        column_panels.read_untracked().contains(panel_id)
                                    })
                                    .unwrap_or(false);
                                if !is_in_column {
                                    return (closest_panel, closest_dist);
                                }

                                let rect = panel_ref.get_bounding_client_rect();
                                let (dist, center) = match horizontal {
                                    false => {
                                        let center_y = rect.top() + rect.height() / 2.0;
                                        ((mouse_y - center_y).abs(), center_y)
                                    }
                                    true => {
                                        let center_x = rect.left() + rect.width() / 2.0;
                                        ((mouse_x - center_x).abs(), center_x)
                                    }
                                };
                                if dist < closest_dist {
                                    (Some((panel_id.clone(), panel_ref.clone(), center)), dist)
                                } else {
                                    (closest_panel, closest_dist)
                                }
                            },
                        );

                        let new_hover_info = if let Some((panel_id, _, center)) = closest_panel {
                            if !horizontal && mouse_y < center || horizontal && mouse_x < center {
                                Some(HoverInfo {
                                    column_index,
                                    panel: Some(HoveredPanel {
                                        id: panel_id,
                                        position: HoverPosition::Before,
                                    }),
                                })
                            } else {
                                Some(HoverInfo {
                                    column_index,
                                    panel: Some(HoveredPanel {
                                        id: panel_id,
                                        position: HoverPosition::After,
                                    }),
                                })
                            }
                        } else {
                            Some(HoverInfo {
                                column_index,
                                panel: None,
                            })
                        };

                        hover_info.maybe_update(move |hovered| {
                            if hovered != &new_hover_info {
                                *hovered = new_hover_info;
                                true
                            } else {
                                false
                            }
                        });
                    }
                }) as Box<dyn FnMut(_)>)
                .into_js_value()
                .dyn_into()
                .unwrap();

                document()
                    .add_event_listener_with_callback_and_bool("dragover", &on_dragover, false)
                    .unwrap();

                on_dragover_cb.set(Some(on_dragover));
            }
        };

        let currently_dragged_panel = self.currently_dragged_panel.clone();
        let hover_info = self.hover_info.clone();
        let on_drag_end = {
            let id = id.clone();
            move |_ev: ev::DragEvent| {
                if let Some(on_dragover) = on_dragover_cb.write().take() {
                    document()
                        .remove_event_listener_with_callback("dragover", &on_dragover)
                        .unwrap();
                }

                let id = id.clone();
                request_animation_frame(move || {
                    let mut current = currently_dragged_panel.write();
                    if current.as_deref() == Some(&id) {
                        hover_info.set(None);
                        draggable.set(false);
                        *current = None;
                    }
                });
            }
        };

        PanelReorderEvents {
            node_ref,
            is_dragging,
            hover_position,
            draggable: draggable.into(),
            set_draggable,
            on_dragstart: on_drag_start,
            on_dragend: on_drag_end,
        }
    }

    /// Registers a panel with drag reordering for a given ID.
    pub fn generate_header(
        &self,
        index: usize,
    ) -> (NodeRef<Div>, Signal<Option<HoverPosition>>) {
        if !self.initialized.get_untracked() {
            self.initialize();
        }
        let node_ref = NodeRef::<Div>::new();

        let hover_position = Signal::derive({
            let hover_info = self.hover_info.clone();
            move || match &*hover_info.read() {
                Some(HoverInfo {
                    panel: None,
                    column_index
                }) => if *column_index == index {
                    Some(HoverPosition::After)
                } else {
                    None
                }
                _ => None
            }
        });
        (node_ref, hover_position)
    }
}

/// Return value for [`use_drag_reorder`].
pub struct PanelReorderEvents<SetDraggable, OnDragStart, OnDragEnd>
where
    SetDraggable: Fn(bool) + Copy,
    OnDragStart: Fn(ev::DragEvent) + Clone,
    OnDragEnd: Fn(ev::DragEvent) + Clone,
{
    /// Node ref which should be assigned to the panel element.
    pub node_ref: NodeRef<Div>,
    /// Is this panel being dragged.
    pub is_dragging: Signal<bool>,
    /// The current position this panel is being hovered over.
    ///
    /// This is useful for styling. Typically you would have a line above or below this panel to indicate
    /// the dragged panel can be dropped.
    pub hover_position: Signal<Option<HoverPosition>>,
    /// Is the panel draggable.
    pub draggable: Signal<bool>,
    /// Enables/disables the panel to be draggable.
    pub set_draggable: SetDraggable,
    /// Callback which should be assigned to the `on:dragstart` event.
    pub on_dragstart: OnDragStart,
    /// Callback which should be assigned to the `on:dragend` event.
    pub on_dragend: OnDragEnd,
}

/// A hovering panels position either above or below.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoverPosition {
    Before,
    After,
}

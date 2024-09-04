use crate::{_aria::use_active_descendant, icon::ChevronDownRegularIcon, listbox::Listbox};
use leptos::{context::Provider, html, prelude::*};
use thaw_components::{Binder, Follower, FollowerPlacement, FollowerWidth};
use thaw_utils::{class_list, mount_style};

#[component]
pub fn TagPicker(
    #[prop(optional, into)] class: MaybeProp<String>,
    tag_picker_control: TagPickerControl,
    children: Children,
) -> impl IntoView {
    mount_style("tag-picker", include_str!("./tag-picker.css"));
    let TagPickerControl {
        children: control_children,
    } = tag_picker_control;
    let is_show_listbox = RwSignal::new(false);
    let trigger_ref = NodeRef::<html::Div>::new();
    let listbox_ref = NodeRef::<html::Div>::new();

    let (set_listbox, active_descendant_controller) =
        use_active_descendant(move |el| el.class_list().contains("thaw-tag-picker-option"));

    let tag_picker_injection = TagPickerInjection { is_show_listbox };

    view! {
        <Binder target_ref=trigger_ref>
            <div class=class_list!["thaw-tag-picker-control", class] node_ref=trigger_ref>
                <Provider value=tag_picker_injection>
                    {control_children()}
                </Provider>
                <span class="thaw-tag-picker-control__aside">
                    <span class="thaw-tag-picker-control__expand-icon">
                        <ChevronDownRegularIcon />
                    </span>
                </span>
            </div>
            <Follower
                slot
                show=is_show_listbox
                placement=FollowerPlacement::BottomStart
                width=FollowerWidth::MinTarget
            >
                <Provider value=tag_picker_injection>
                    <Listbox
                        open=is_show_listbox.read_only()
                        set_listbox
                        listbox_ref
                        class="thaw-tag-picker__listbox"
                    >
                        {children()}
                    </Listbox>
                </Provider>
            </Follower>
        </Binder>
    }
}

#[slot]
pub struct TagPickerControl {
    children: Children,
}

#[derive(Clone, Copy)]
pub(crate) struct TagPickerInjection {
    pub is_show_listbox: RwSignal<bool>,
}

impl TagPickerInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }
}

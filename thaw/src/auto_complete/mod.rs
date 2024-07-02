mod auto_complete_option;

pub use auto_complete_option::AutoCompleteOption;

use crate::{ComponentRef, ConfigInjection, Input, InputPrefix, InputRef, InputSuffix};
use leptos::*;
use thaw_components::{
    Binder, CSSTransition, Follower, FollowerPlacement, FollowerWidth, OptionComp,
};
use thaw_utils::{class_list, mount_style, Model, OptionalProp};

#[slot]
pub struct AutoCompletePrefix {
    children: Children,
}

#[slot]
pub struct AutoCompleteSuffix {
    children: Children,
}

#[component]
pub fn AutoComplete(
    #[prop(optional, into)] value: Model<String>,
    #[prop(optional, into)] placeholder: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] clear_after_select: MaybeSignal<bool>,
    #[prop(optional, into)] blur_after_select: MaybeSignal<bool>,
    #[prop(optional, into)] on_select: Option<Callback<String>>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(optional, into)] allow_free_input: bool,
    #[prop(optional, into)] invalid: MaybeSignal<bool>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional)] auto_complete_prefix: Option<AutoCompletePrefix>,
    #[prop(optional)] auto_complete_suffix: Option<AutoCompleteSuffix>,
    #[prop(optional)] comp_ref: ComponentRef<AutoCompleteRef>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    mount_style("auto-complete", include_str!("./auto-complete.css"));
    let config_provider = ConfigInjection::use_();
    let input_ref = ComponentRef::<InputRef>::new();

    let default_index = if allow_free_input { None } else { Some(0) };

    let select_option_index = create_rw_signal::<Option<usize>>(default_index);
    let menu_ref = create_node_ref::<html::Div>();
    let is_show_menu = create_rw_signal(false);
    let auto_complete_ref = create_node_ref::<html::Div>();
    let open_menu = move || {
        select_option_index.set(default_index);
        is_show_menu.set(true);
    };
    let allow_value = move |_| {
        if !is_show_menu.get_untracked() {
            open_menu();
        }
        true
    };

    let select_value = Callback::new(move |option_value: String| {
        if clear_after_select.get_untracked() {
            value.set(String::new());
        } else {
            value.set(option_value.clone());
        }
        if let Some(on_select) = on_select {
            on_select.call(option_value);
        }
        if allow_free_input {
            select_option_index.set(None);
        }
        is_show_menu.set(false);
        if blur_after_select.get_untracked() {
            if let Some(input_ref) = input_ref.get_untracked() {
                input_ref.blur();
            }
        }
    });

    // we unset selection index whenever options get changed
    // otherwise e.g. selection could move from one item to
    // another staying on the same index
    //create_effect(move |_| {
    //options.track();
    //select_option_index.set(default_index);
    //});

    //let on_keydown = move |event: ev::KeyboardEvent| {
    //if !is_show_menu.get_untracked() {
    //return;
    //}
    //let key = event.key();
    //if key == *"ArrowDown" {
    //select_option_index.update(|index| {
    //if *index == Some(options.with_untracked(|options| options.len()) - 1) {
    //*index = default_index
    //} else {
    //*index = Some(index.map_or(0, |index| index + 1))
    //}
    //});
    //} else if key == *"ArrowUp" {
    //select_option_index.update(|index| {
    //match *index {
    //None => *index = Some(options.with_untracked(|options| options.len()) - 1),
    //Some(0) => {
    //if allow_free_input {
    //*index = None
    //} else {
    //*index = Some(options.with_untracked(|options| options.len()) - 1)
    //}
    //}
    //Some(prev_index) => *index = Some(prev_index - 1),
    //};
    //});
    //} else if key == *"Enter" {
    //event.prevent_default();
    //let option_value = options.with_untracked(|options| {
    //let index = select_option_index.get_untracked();
    //match index {
    //None if allow_free_input => {
    //let value = value.get_untracked();
    //(!value.is_empty()).then_some(value)
    //}
    //Some(index) if options.len() > index => {
    //let option = &options[index];
    //Some(option.value.clone())
    //}
    //_ => None,
    //}
    //});
    //if let Some(option_value) = option_value {
    //select_value(option_value);
    //}
    //}
    //};
    input_ref.on_load(move |_| {
        comp_ref.load(AutoCompleteRef { input_ref });
    });

    view! {
            <Binder target_ref=auto_complete_ref>
                <div
                    class=class_list!["thaw-auto-complete", class.map(| c | move || c.get())]
                    ref=auto_complete_ref
    //                on:keydown=on_keydown
                >
                    <Input
                        attrs
                        value
                        placeholder
                        disabled
                        invalid
                        on_focus=move |_| open_menu()
                        on_blur=move |_| is_show_menu.set(false)
                        allow_value
                        comp_ref=input_ref
                    >
                        <InputPrefix if_=auto_complete_prefix.is_some() slot>

                            {if let Some(auto_complete_prefix) = auto_complete_prefix {
                                Some((auto_complete_prefix.children)())
                            } else {
                                None
                            }}

                        </InputPrefix>
                        <InputSuffix if_=auto_complete_suffix.is_some() slot>

                            {if let Some(auto_complete_suffix) = auto_complete_suffix {
                                Some((auto_complete_suffix.children)())
                            } else {
                                None
                            }}

                        </InputSuffix>
                    </Input>
                </div>
                <Follower
                    slot
                    show=is_show_menu
                    placement=FollowerPlacement::BottomStart
                    width=FollowerWidth::Target
                >
                    <Provider value=AutoCompleteInjection(value, select_value)>
                        <CSSTransition
                            node_ref=menu_ref
                            name="fade-in-scale-up-transition"
                            appear=is_show_menu.get_untracked()
                            show=is_show_menu
                            let:display
                        >
                            <div
                                class="thaw-config-provider thaw-auto-complete__listbox"
                                style=move || display.get()
                                data-thaw-id=config_provider.id().clone()
                                ref=menu_ref
                                role="listbox"
                            >

                                <OptionComp value=children let:children>
                                    {children()}
                                </OptionComp>
                            </div>
                        </CSSTransition>
                    </Provider>
                </Follower>
            </Binder>
        }
}

#[derive(Clone, Copy)]
pub(crate) struct AutoCompleteInjection(pub Model<String>, pub Callback<String>);

impl AutoCompleteInjection {
    pub fn use_() -> Self {
        expect_context()
    }

    pub fn is_selected(&self, key: &String) -> bool {
        self.0.with(|value| value == key)
    }

    pub fn select_value(&self, key: String) {
        self.1.call(key.clone());
    }
}

#[derive(Clone)]
pub struct AutoCompleteRef {
    input_ref: ComponentRef<InputRef>,
}

impl AutoCompleteRef {
    pub fn focus(&self) {
        if let Some(input_ref) = self.input_ref.get_untracked() {
            input_ref.focus();
        }
    }

    pub fn blur(&self) {
        if let Some(input_ref) = self.input_ref.get_untracked() {
            input_ref.blur();
        }
    }
}

use crate::{
    Button, ButtonAppearance, ButtonSize, Popover, PopoverAppearance, PopoverPosition, PopoverSize,
    PopoverTrigger, PopoverTriggerType,
};
use leptos::prelude::*;
use thaw_utils::{mount_style, BoxCallback, Model};

/// A popover that asks for confirmation before executing an action.
///
/// ```rust
/// <Popconfirm
///     title="Delete the task"
///     description="Are you sure to delete this task?"
///     on_confirm=Callback::new(move |_| delete())
/// >
///     <Button>"Delete"</Button>
/// </Popconfirm>
/// ```
#[component]
pub fn Popconfirm<T: AddAnyAttr + IntoView + Send + 'static>(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Title shown in the popover.
    #[prop(optional, into)]
    title: String,
    /// Description shown below the title.
    #[prop(optional, into)]
    description: String,
    /// Called when the user clicks the confirm button.
    #[prop(into)]
    on_confirm: Callback<()>,
    /// Called when the user clicks the cancel button.
    #[prop(optional, into)]
    on_cancel: Option<Callback<()>>,
    /// Configures the position of the popover.
    #[prop(optional)]
    position: PopoverPosition,
    /// A popover can appear styled with brand or inverted.
    #[prop(optional, into)]
    appearance: MaybeProp<PopoverAppearance>,
    /// Action that displays the popconfirm. Defaults to Click.
    #[prop(default = PopoverTriggerType::Click)]
    trigger_type: PopoverTriggerType,
    #[prop(optional, into)] size: Signal<PopoverSize>,
    #[prop(optional, into)] on_open: Option<BoxCallback>,
    #[prop(optional, into)] on_close: Option<BoxCallback>,
    /// Text for the confirm button.
    #[prop(default = "Yes")]
    ok_text: &'static str,
    /// Text for the cancel button.
    #[prop(default = "No")]
    cancel_text: &'static str,
    /// The trigger element (e.g. a Button).
    children: TypedChildren<T>,
) -> impl IntoView {
    mount_style("popconfirm", include_str!("./popconfirm.css"));

    let is_open: Model<bool> = RwSignal::new(false).into();

    let on_yes = move |_| {
        is_open.set(false);
        on_confirm.run(());
    };

    let on_no = move |_| {
        is_open.set(false);
        if let Some(cb) = &on_cancel {
            cb.run(());
        }
    };

    let on_open = on_open.unwrap_or_else(|| BoxCallback::new(|| {}));
    let on_close = on_close.unwrap_or_else(|| BoxCallback::new(|| {}));

    view! {
        <Popover
            class=class
            appearance=appearance
            trigger_type=trigger_type
            position=position
            size=size
            open=is_open
            on_open=on_open
            on_close=on_close
            popover_trigger=PopoverTrigger { children }
        >
            {(!title.is_empty()).then(|| view! {
                <div class="thaw-popconfirm__title">{title}</div>
            })}
            {(!description.is_empty()).then(|| view! {
                <div class="thaw-popconfirm__description">{description}</div>
            })}
            <div class="thaw-popconfirm__actions">
                <Button size=ButtonSize::Small appearance=ButtonAppearance::Secondary on_click=on_no>
                    {cancel_text}
                </Button>
                <Button size=ButtonSize::Small appearance=ButtonAppearance::Primary on_click=on_yes>
                    {ok_text}
                </Button>
            </div>
        </Popover>
    }
}

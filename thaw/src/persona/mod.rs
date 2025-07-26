mod types;

pub use types::*;

use crate::Avatar;
use leptos::{either::Either, prelude::*};
use thaw_components::{If, Then};
use thaw_utils::class_list;

#[cfg(feature = "manganis")]
const _: manganis::Asset = manganis::asset!("/src/persona/persona.css");

#[component]
pub fn Persona(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    /// The name of the person or entity represented by the Persona.
    /// When primary_text is not provided, this will be used as the default value for primaryText.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// The size of a Persona and its text.
    #[prop(optional, into)]
    size: Signal<PersonaSize>,
    /// The vertical alignment of the text relative to the avatar.
    #[prop(optional, into)]
    text_alignment: Signal<PersonaTextAlignment>,
    /// The position of the text relative to the avatar.
    #[prop(optional, into)]
    text_position: Signal<PersonaTextPosition>,
    #[prop(optional, into)] avatar_src: MaybeProp<String>,
    /// The first line of text in the Persona, larger than the rest of the lines.
    #[prop(optional)]
    persona_primary_text: Option<PersonaPrimaryText>,
    /// The second line of text in the Persona.
    #[prop(optional)]
    persona_secondary_text: Option<PersonaSecondaryText>,
    /// The third line of text in the Persona.
    #[prop(optional)]
    persona_tertiary_text: Option<PersonaTertiaryText>,
    /// The fourth line of text in the Persona.
    #[prop(optional)]
    persona_quaternary_text: Option<PersonaQuaternaryText>,
) -> impl IntoView {
    #[cfg(not(feature = "manganis"))]
    thaw_utils::mount_style("persona", include_str!("./persona.css"));

    let text_position_before =
        Memo::new(move |_| text_position.get() == PersonaTextPosition::Before);

    let style = move || {
        let css_var = match size.get() {
            PersonaSize::ExtraSmall => "spacingHorizontalSNudge",
            PersonaSize::Small => "spacingHorizontalS",
            PersonaSize::Medium => "spacingHorizontalS",
            PersonaSize::Large => "spacingHorizontalMNudge",
            PersonaSize::ExtraLarge => "spacingHorizontalMNudge",
            PersonaSize::Huge => "spacingHorizontalM",
        };

        let mut s = format!("--thaw-persona__avatar-spacing: var(--{css_var});");

        style.with(|style| {
            if let Some(style) = style.as_ref() {
                s.push_str(style);
            }
        });

        s
    };

    let avatar_size = Memo::new(move |_| size.get().as_avatar_size());

    view! {
        <div
            class=class_list![
                "thaw-persona",
                move || format!("thaw-persona--{}", text_alignment.get().as_str()),
                move || format!("thaw-persona--{}", text_position.get().as_str()),
                move || format!("thaw-persona--{}", size.get().as_str()),
                class
            ]
            style=style
        >
            <If cond=Signal::derive(move || !text_position_before.get())>
                <Then slot>
                    <Avatar
                        class="thaw-persona__avatar"
                        name=name
                        src=avatar_src
                        size=avatar_size
                    />
                </Then>
            </If>
            {if let Some(text) = persona_primary_text {
                Either::Left(
                    view! { <span class="thaw-persona__primary-text">{(text.children)()}</span> },
                )
            } else {
                Either::Right(move || {
                    if let Some(name) = name.get() {
                        Either::Left(
                            view! { <span class="thaw-persona__primary-text">{name}</span> },
                        )
                    } else {
                        Either::Right(())
                    }
                })
            }}
            {if let Some(text) = persona_secondary_text {
                Either::Left(
                    view! { <span class="thaw-persona__secondary-text">{(text.children)()}</span> },
                )
            } else {
                Either::Right(())
            }}
            {if let Some(text) = persona_tertiary_text {
                Either::Left(
                    view! { <span class="thaw-persona__tertiary-text">{(text.children)()}</span> },
                )
            } else {
                Either::Right(())
            }}
            {if let Some(text) = persona_quaternary_text {
                Either::Left(
                    view! { <span class="thaw-persona__quaternary-text">{(text.children)()}</span> },
                )
            } else {
                Either::Right(())
            }}

            <If cond=text_position_before>
                <Then slot>
                    <Avatar
                        class="thaw-persona__avatar"
                        name=name
                        src=avatar_src
                        size=avatar_size
                    />
                </Then>
            </If>
        </div>
    }
}

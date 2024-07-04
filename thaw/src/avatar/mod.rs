use leptos::prelude::*;
use thaw_components::OptionComp;
use thaw_utils::{class_list, mount_style, OptionalProp, StoredMaybeSignal};

#[component]
pub fn Avatar(
    /// The Avatar's image.
    #[prop(optional, into)]
    src: Option<MaybeSignal<String>>,
    /// The name of the person or entity represented by this Avatar.
    #[prop(optional, into)]
    name: Option<MaybeSignal<String>>,
    /// Custom initials.
    #[prop(optional, into)]
    initials: Option<MaybeSignal<String>>,
    /// The avatar can have a circular or square shape.
    #[prop(optional, into)]
    shape: MaybeSignal<AvatarShape>,
    /// Size of the avatar in pixels.
    #[prop(optional, into)]
    size: Option<MaybeSignal<u8>>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
) -> impl IntoView {
    mount_style("avatar", include_str!("./avatar.css"));

    let style = move || {
        let size = size?.get();

        let mut style = format!("width: {0}px; height: {0}px;", size);

        if let Some(font_size) = match size {
            0..=24 => Some(100),
            25..=28 => Some(200),
            29..=40 => None,
            41..=56 => Some(400),
            57..=96 => Some(500),
            97..=128 => Some(600),
            _ => Some(600),
        } {
            style.push_str(&format!("font-size: var(--fontSizeBase{});", font_size))
        }

        Some(style)
    };

    let is_show_default_icon = src.is_none() && initials.is_none() && name.is_none();
    let name: Option<StoredMaybeSignal<_>> = name.map(|n| n.into());

    view! {
        <span
            class=class_list!["thaw-avatar", move || format!("thaw-avatar--{}", shape.get().as_str()), class.map(| c | move || c.get())]
            style=move || style().unwrap_or_default()
            role="img"
            aria-label=move || name.as_ref().map(|n| n.get())
        >
            {
                move || {
                    if let Some(initials) = initials.as_ref().map_or_else(|| name.as_ref().map(|n| initials_name(n.get())), |i| Some(i.get())) {
                        view! {
                            <span class="thaw-avatar__initials">
                                {initials}
                            </span>
                        }.into()
                    } else {
                        None
                    }
                }
            }
            <OptionComp value=src let:src>
                <img src=move || src.get() class="thaw-avatar__image"/>
            </OptionComp>
            {
                if is_show_default_icon {
                    view! {
                        <span aria-hidden="true" class="thaw-avatar__icon">
                            <svg fill="currentColor" aria-hidden="true" width="1em" height="1em" viewBox="0 0 20 20">
                                <path d="M10 2a4 4 0 1 0 0 8 4 4 0 0 0 0-8ZM7 6a3 3 0 1 1 6 0 3 3 0 0 1-6 0Zm-2 5a2 2 0 0 0-2 2c0 1.7.83 2.97 2.13 3.8A9.14 9.14 0 0 0 10 18c1.85 0 3.58-.39 4.87-1.2A4.35 4.35 0 0 0 17 13a2 2 0 0 0-2-2H5Zm-1 2a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1c0 1.3-.62 2.28-1.67 2.95A8.16 8.16 0 0 1 10 17a8.16 8.16 0 0 1-4.33-1.05A3.36 3.36 0 0 1 4 13Z" fill="currentColor"></path>
                            </svg>
                        </span>
                    }.into()
                } else {
                    None
                }
            }
        </span>
    }
}

// TODO
fn initials_name(name: String) -> String {
    name.split_at(2).0.to_string().to_ascii_uppercase()
}

#[derive(Default, Clone)]
pub enum AvatarShape {
    #[default]
    Circular,
    Square,
}

impl AvatarShape {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Circular => "circular",
            Self::Square => "square",
        }
    }
}

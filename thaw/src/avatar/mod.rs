use leptos::{either::Either, prelude::*};
use thaw_components::OptionComp;
use thaw_utils::{class_list, mount_style};

#[component]
pub fn Avatar(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// The Avatar's image.
    #[prop(optional, into)]
    src: MaybeProp<String>,
    /// The name of the person or entity represented by this Avatar.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// Custom initials.
    #[prop(optional, into)]
    initials: MaybeProp<String>,
    /// The avatar can have a circular or square shape.
    #[prop(optional, into)]
    shape: Signal<AvatarShape>,
    /// Size of the avatar in pixels.
    #[prop(optional, into)]
    size: MaybeProp<u8>,
) -> impl IntoView {
    mount_style("avatar", include_str!("./avatar.css"));

    let style = move || {
        let size = size.get()?;

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

    let name = StoredValue::new(name);
    let src = StoredValue::new(src);
    let initials = StoredValue::new(initials);
    let is_show_default_icon = Memo::new(move |_| {
        if name.with_value(|n| n.with(|n| n.is_some())) {
            false
        } else if src.with_value(|s| s.with(|s| s.is_some())) {
            false
        } else if initials.with_value(|i| i.with(|i| i.is_some())) {
            false
        } else {
            true
        }
    });

    view! {
        <span
            class=class_list![
                "thaw-avatar",
                move || format!("thaw-avatar--{}", shape.get().as_str()),
                class
            ]
            style=move || style().unwrap_or_default()
            role="img"
            aria-label=move || name.with_value(|n| n.get())
        >
            {move || {
                let mut initials = initials.with_value(|i| i.get());
                if initials.is_none() {
                    name.with_value(|name| {
                        if let Some(name) = name.get() {
                            initials = Some(initials_name(name));
                        }
                    });
                }
                view! {
                    <OptionComp value=initials let:initials>
                        <span class="thaw-avatar__initials">{initials}</span>
                    </OptionComp>
                }
            }}
            {move || {
                let src = src.with_value(|s| s.get());
                view! {
                    <OptionComp value=src let:src>
                        <img src=src class="thaw-avatar__image" />
                    </OptionComp>
                }
            }}
            {move || {
                if is_show_default_icon.get() {
                    Either::Left(
                        view! {
                            <span aria-hidden="true" class="thaw-avatar__icon">
                                <svg
                                    fill="currentColor"
                                    aria-hidden="true"
                                    width="1em"
                                    height="1em"
                                    viewBox="0 0 20 20"
                                >
                                    <path
                                        d="M10 2a4 4 0 1 0 0 8 4 4 0 0 0 0-8ZM7 6a3 3 0 1 1 6 0 3 3 0 0 1-6 0Zm-2 5a2 2 0 0 0-2 2c0 1.7.83 2.97 2.13 3.8A9.14 9.14 0 0 0 10 18c1.85 0 3.58-.39 4.87-1.2A4.35 4.35 0 0 0 17 13a2 2 0 0 0-2-2H5Zm-1 2a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1c0 1.3-.62 2.28-1.67 2.95A8.16 8.16 0 0 1 10 17a8.16 8.16 0 0 1-4.33-1.05A3.36 3.36 0 0 1 4 13Z"
                                        fill="currentColor"
                                    ></path>
                                </svg>
                            </span>
                        },
                    )
                } else {
                    Either::Right(())
                }
            }}
        </span>
    }
}

fn initials_name(name: String) -> String {
    let initials: Vec<_> = name
        .split_whitespace()
        .filter_map(|word| {
            word.chars()
                .next()
                .map(|c| c.to_uppercase().collect::<String>())
        })
        .collect();

    match initials.as_slice() {
        [first, .., last] => format!("{first}{last}"),
        [first] => first.clone(),
        [] => String::new(),
    }
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

#[test]
fn test_initials_name() {
    assert_eq!(initials_name("Jane Doe".into()), "JD".to_string());
    assert_eq!(initials_name("Ben".into()), "B".to_string());
    assert_eq!(
        initials_name("ÇFoo Bar 1Name too ÉLong".into()),
        "ÇÉ".to_string()
    );
    assert_eq!(initials_name("ﬄ ß".into()), "FFLSS".to_string());
    assert_eq!(initials_name("".into()), "".to_string());
    assert_eq!(initials_name("山".into()), "山".to_string());
}

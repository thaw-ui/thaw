#[cfg(not(feature = "nightly"))]
use leptos::{
    prelude::{MaybeProp, Memo},
    reactive_graph::traits::Get,
};
use leptos::{
    prelude::{Oco, RenderEffect, RwSignal},
    reactive_graph::traits::{Update, With, WithUntracked},
    tachys::renderer::{types, Rndr},
};
use std::collections::HashSet;
#[cfg(not(feature = "ssr"))]
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct ClassList {
    value: RwSignal<HashSet<Oco<'static, str>>>,
    #[cfg(not(feature = "ssr"))]
    effects_oco: Vec<Arc<RenderEffect<Oco<'static, str>>>>,
    #[cfg(not(feature = "ssr"))]
    effects_option_oco: Vec<Arc<RenderEffect<Option<Oco<'static, str>>>>>,
    #[cfg(not(feature = "ssr"))]
    effects_bool: Vec<Arc<RenderEffect<bool>>>,
}

impl ClassList {
    pub fn new() -> Self {
        Default::default()
    }

    #[allow(unused_mut)]
    pub fn add(mut self, value: impl IntoClass) -> Self {
        let class = value.into_class();
        match class {
            Class::None => (),
            Class::String(name) => {
                self.value.update(move |set| {
                    set.insert(name);
                });
            }
            Class::FnString(f) => {
                #[cfg(feature = "ssr")]
                {
                    let name = f();
                    self.value.update(|set| {
                        set.insert(name);
                    });
                }
                #[cfg(not(feature = "ssr"))]
                {
                    let effect = RenderEffect::new(move |old_name| {
                        let name = f();
                        if let Some(old_name) = old_name {
                            if old_name != name {
                                self.value.update(|set| {
                                    set.remove(&old_name);
                                    set.insert(name.clone());
                                });
                            }
                        } else {
                            self.value.update(|set| {
                                set.insert(name.clone());
                            });
                        }
                        name
                    });
                    self.effects_oco.push(effect.into());
                }
            }
            Class::FnOptionString(f) => {
                #[cfg(feature = "ssr")]
                {
                    if let Some(name) = f() {
                        self.value.update(|set| {
                            set.insert(name);
                        });
                    }
                }
                #[cfg(not(feature = "ssr"))]
                {
                    let effect = RenderEffect::new(move |old_name| {
                        let name = f();
                        if let Some(old_name) = old_name {
                            if old_name != name {
                                self.value.update(|set| match (old_name, name.clone()) {
                                    (None, Some(name)) => {
                                        set.insert(name);
                                    }
                                    (Some(old_name), None) => {
                                        set.remove(&old_name);
                                    }
                                    (Some(old_name), Some(name)) => {
                                        set.remove(&old_name);
                                        set.insert(name);
                                    }
                                    _ => {}
                                });
                            }
                        } else {
                            if let Some(name) = name.clone() {
                                self.value.update(|set| {
                                    set.insert(name.clone());
                                });
                            }
                        }
                        name
                    });
                    self.effects_option_oco.push(effect.into());
                }
            }
            Class::Fn(name, f) => {
                #[cfg(feature = "ssr")]
                {
                    let new = f();
                    if new {
                        self.value.update(|set| {
                            set.insert(name);
                        });
                    }
                }
                #[cfg(not(feature = "ssr"))]
                {
                    let effect = RenderEffect::new(move |old| {
                        let name = name.clone();
                        let new = f();
                        if old.is_none() {
                            if new {
                                self.value.update(|set| {
                                    set.insert(name);
                                });
                            }
                        } else if old.as_ref() != Some(&new) {
                            self.value.update(|set| {
                                if new {
                                    set.insert(name);
                                } else {
                                    set.remove(&name);
                                }
                            });
                        }
                        new
                    });
                    self.effects_bool.push(effect.into());
                }
            }
        }

        self
    }

    fn write_class_string(&self, class: &mut String) {
        self.value.with(|set| {
            set.iter().enumerate().for_each(|(index, name)| {
                if name.is_empty() {
                    return;
                }
                if index != 0 {
                    class.push(' ');
                }
                class.push_str(name)
            });
        });
    }
}

impl leptos::tachys::html::class::IntoClass for ClassList {
    type AsyncOutput = Self;
    type State = RenderEffect<(types::Element, String)>;
    type Cloneable = Self;
    type CloneableOwned = Self;

    fn html_len(&self) -> usize {
        self.value.with_untracked(|set| {
            let mut len = 0;
            set.iter().enumerate().for_each(|(index, name)| {
                if name.is_empty() {
                    return;
                }
                if index != 0 {
                    len += 1;
                }
                len += name.len();
            });

            len
        })
    }

    fn to_html(self, class: &mut String) {
        self.write_class_string(class);
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &types::Element) -> Self::State {
        let el = el.to_owned();
        RenderEffect::new(move |prev| {
            let mut class = String::new();
            self.write_class_string(&mut class);

            if let Some(state) = prev {
                let (el, prev_class) = state;
                if class != prev_class {
                    Rndr::set_attribute(&el, "class", &class);
                    (el, class)
                } else {
                    (el, prev_class)
                }
            } else {
                if !class.is_empty() {
                    if !FROM_SERVER {
                        Rndr::set_attribute(&el, "class", &class);
                    }
                }
                (el.clone(), class)
            }
        })
    }

    fn build(self, el: &types::Element) -> Self::State {
        let el = el.to_owned();
        RenderEffect::new(move |prev| {
            let mut class = String::new();
            self.write_class_string(&mut class);
            if let Some(state) = prev {
                let (el, prev_class) = state;
                if class != prev_class {
                    Rndr::set_attribute(&el, "class", &class);
                    (el, class)
                } else {
                    (el, prev_class)
                }
            } else {
                if !class.is_empty() {
                    Rndr::set_attribute(&el, "class", &class);
                }
                (el.clone(), class)
            }
        })
    }

    fn rebuild(self, state: &mut Self::State) {
        let prev = state.take_value();
        *state = RenderEffect::new_with_value(
            move |prev| {
                if let Some(state) = prev {
                    let mut class = String::new();
                    self.write_class_string(&mut class);
                    let (el, prev_class) = state;
                    if class != *prev_class {
                        Rndr::set_attribute(&el, "class", &class);
                        (el, class)
                    } else {
                        (el, prev_class)
                    }
                } else {
                    unreachable!()
                }
            },
            prev,
        );
    }

    fn into_cloneable(self) -> Self::Cloneable {
        self
    }

    fn into_cloneable_owned(self) -> Self::CloneableOwned {
        self
    }

    fn dry_resolve(&mut self) {}

    async fn resolve(self) -> Self::AsyncOutput {
        self
    }
}

pub enum Class {
    None,
    String(Oco<'static, str>),
    FnString(Box<dyn Fn() -> Oco<'static, str>>),
    FnOptionString(Box<dyn Fn() -> Option<Oco<'static, str>>>),
    Fn(Oco<'static, str>, Box<dyn Fn() -> bool>),
}

pub trait IntoClassValue {
    fn into_class_value(self) -> Option<Oco<'static, str>>;
}

impl IntoClassValue for String {
    fn into_class_value(self) -> Option<Oco<'static, str>> {
        Some(self.into())
    }
}

impl IntoClassValue for Option<String> {
    fn into_class_value(self) -> Option<Oco<'static, str>> {
        self.map(|v| v.into())
    }
}

pub trait IntoClass {
    fn into_class(self) -> Class;
}

impl IntoClass for String {
    fn into_class(self) -> Class {
        Class::String(self.into())
    }
}

impl IntoClass for &'static str {
    fn into_class(self) -> Class {
        Class::String(self.into())
    }
}

impl<T, U> IntoClass for T
where
    T: Fn() -> U + 'static,
    U: IntoClassValue,
{
    fn into_class(self) -> Class {
        Class::FnOptionString(Box::new(move || (self)().into_class_value()))
    }
}

impl<T, U> IntoClass for Option<T>
where
    T: Fn() -> U + 'static,
    U: ToString,
{
    fn into_class(self) -> Class {
        if let Some(f) = self {
            Class::FnString(Box::new(move || f().to_string().into()))
        } else {
            Class::None
        }
    }
}

#[cfg(not(feature = "nightly"))]
impl IntoClass for MaybeProp<String> {
    fn into_class(self) -> Class {
        Class::FnOptionString(Box::new(move || self.get().map(|c| Oco::from(c))))
    }
}

impl<T> IntoClass for (&'static str, T)
where
    T: Fn() -> bool + 'static,
{
    fn into_class(self) -> Class {
        Class::Fn(self.0.into(), Box::new(self.1))
    }
}

impl IntoClass for (&'static str, bool) {
    fn into_class(self) -> Class {
        if self.1 {
            Class::String(self.0.into())
        } else {
            Class::None
        }
    }
}

#[cfg(not(feature = "nightly"))]
impl IntoClass for (&'static str, Memo<bool>) {
    fn into_class(self) -> Class {
        Class::Fn(self.0.into(), Box::new(move || self.1.get()))
    }
}

impl<T> IntoClass for (String, T)
where
    T: Fn() -> bool + 'static,
{
    fn into_class(self) -> Class {
        Class::Fn(self.0.into(), Box::new(self.1))
    }
}

#[macro_export]
macro_rules! class_list {
    ($($name:expr),+) => {
        {
            use $crate::class_list::ClassList;
            ClassList::new()$(.add($name))+
        }
    };
}

// TODO
// #[cfg(test)]
// mod tests {
//     use leptos::reactive_graph::Run;

//     #[test]
//     fn macro_class_list() {
//         let rt = create_runtime();
//         let class_list = class_list!("aa", ("bb", || true), move || "cc");
//         if let Attribute::Fn(f) = class_list.into_attribute() {
//             if let Attribute::String(class) = f() {
//                 assert!(class.contains("aa"));
//                 assert!(class.contains("bb"));
//                 assert!(class.contains("cc"));
//             }
//         }
//         rt.dispose();
//     }
// }

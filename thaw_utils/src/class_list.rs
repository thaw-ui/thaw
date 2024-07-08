#[cfg(not(feature = "ssr"))]
use leptos::prelude::RenderEffect;
use leptos::{
    prelude::{MaybeProp, Memo, Oco, RwSignal},
    reactive_graph::traits::{Get, Update, With, WithUntracked},
    tachys::renderer::DomRenderer,
};
use std::collections::HashSet;

#[derive(Clone)]
pub struct ClassList(RwSignal<HashSet<Oco<'static, str>>>);

impl ClassList {
    pub fn new() -> Self {
        Self(RwSignal::new(HashSet::new()))
    }

    pub fn add(self, value: impl IntoClass) -> Self {
        let class = value.into_class();
        match class {
            Class::None => (),
            Class::String(name) => {
                self.0.update(move |set| {
                    set.insert(name);
                });
            }
            Class::FnString(f) => {
                #[cfg(feature = "ssr")]
                {
                    let name = f();
                    self.0.update(|set| {
                        set.insert(name);
                    });
                }
                #[cfg(not(feature = "ssr"))]
                let _ = RenderEffect::new(move |old_name| {
                    let name = f();
                    if let Some(old_name) = old_name {
                        if old_name != name {
                            self.0.update(|set| {
                                set.remove(&old_name);
                                set.insert(name.clone());
                            });
                        }
                    } else {
                        self.0.update(|set| {
                            set.insert(name.clone());
                        });
                    }
                    name
                });
            }
            Class::FnOptionString(f) => {
                #[cfg(feature = "ssr")]
                {
                    if let Some(name) = f() {
                        self.0.update(|set| {
                            set.insert(name);
                        });
                    }
                }
                #[cfg(not(feature = "ssr"))]
                let _ = RenderEffect::new(move |old_name| {
                    let name = f();
                    if let Some(old_name) = old_name {
                        if old_name != name {
                            self.0.update(|set| match (old_name, name.clone()) {
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
                            self.0.update(|set| {
                                set.insert(name.clone());
                            });
                        }
                    }
                    name
                });
            }
            Class::Fn(name, f) => {
                #[cfg(feature = "ssr")]
                {
                    let new = f();
                    self.0.update(|set| {
                        if new {
                            set.insert(name);
                        }
                    });
                }
                #[cfg(not(feature = "ssr"))]
                let _ = RenderEffect::new(move |old| {
                    let name = name.clone();
                    let new = f();
                    if old.is_none() {
                        if new {
                            self.0.update(|set| {
                                set.insert(name);
                            });
                        }
                    } else if old.as_ref() != Some(&new) {
                        self.0.update(|set| {
                            if new {
                                set.insert(name);
                            } else {
                                set.remove(&name);
                            }
                        });
                    }
                    new
                });
            }
        }

        self
    }

    fn to_class_string(self, class: &mut String) {
        self.0.with(|set| {
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

impl<R> leptos::tachys::html::class::IntoClass<R> for ClassList
where
    R: DomRenderer,
{
    type AsyncOutput = Self;
    type State = (R::Element, String);
    type Cloneable = Self;
    type CloneableOwned = Self;

    fn html_len(&self) -> usize {
        self.0.with_untracked(|set| {
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
        self.to_class_string(class);
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &R::Element) -> Self::State {
        let class_list = R::class_list(el);
        let mut class = String::new();
        self.to_class_string(&mut class);

        if !FROM_SERVER {
            R::add_class(&class_list, &class);
        }
        (el.clone(), class)
    }

    fn build(self, el: &R::Element) -> Self::State {
        let mut class = String::new();
        self.to_class_string(&mut class);
        if !class.is_empty() {
            R::set_attribute(el, "class", &class);
        }
        (el.clone(), class)
    }

    fn rebuild(self, state: &mut Self::State) {
        let mut class = String::new();
        self.to_class_string(&mut class);
        let (el, prev_class) = state;
        if class != *prev_class {
            R::set_attribute(el, "class", &class);
        }
        *prev_class = class;
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
    U: ToString,
{
    fn into_class(self) -> Class {
        Class::FnString(Box::new(move || (self)().to_string().into()))
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

impl IntoClass for (&'static str, Memo<bool>) {
    fn into_class(self) -> Class {
        Class::Fn(self.0.into(), Box::new(move || self.1.get()))
    }
}

impl IntoClass for MaybeProp<String> {
    fn into_class(self) -> Class {
        Class::FnOptionString(Box::new(move || self.get().map(|c| Oco::from(c))))
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

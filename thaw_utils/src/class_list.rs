#[cfg(not(feature = "ssr"))]
use leptos::create_render_effect;
use leptos::{
    Attribute, IntoAttribute, MaybeProp, Memo, Oco, RwSignal, SignalGet, SignalUpdate, SignalWith,
};
use std::{collections::HashSet, rc::Rc};

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
                create_render_effect(move |old_name| {
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
                create_render_effect(move |old_name| {
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
                create_render_effect(move |old| {
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
}

impl IntoAttribute for ClassList {
    fn into_attribute(self) -> Attribute {
        Attribute::Fn(Rc::new(move || {
            self.0.with(|set| {
                let mut class = String::new();
                set.iter().enumerate().for_each(|(index, name)| {
                    if name.is_empty() {
                        return;
                    }
                    if index != 0 {
                        class.push(' ');
                    }
                    class.push_str(name)
                });
                class.into_attribute()
            })
        }))
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        self.into_attribute()
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

#[cfg(test)]
mod tests {
    use leptos::{create_runtime, Attribute, IntoAttribute};

    #[test]
    fn macro_class_list() {
        let rt = create_runtime();
        let class_list = class_list!("aa", ("bb", || true), move || "cc");
        if let Attribute::Fn(f) = class_list.into_attribute() {
            if let Attribute::String(class) = f() {
                assert!(class.contains("aa"));
                assert!(class.contains("bb"));
                assert!(class.contains("cc"));
            }
        }
        rt.dispose();
    }
}

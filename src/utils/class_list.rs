use leptos::{
    create_render_effect, Attribute, IntoAttribute, Oco, RwSignal, SignalUpdate, SignalWith,
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
            Class::String(name) => {
                self.0.update(move |set| {
                    set.insert(name);
                });
            }
            Class::FnString(f) => {
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
            Class::Fn(name, f) => {
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
    String(Oco<'static, str>),
    FnString(Box<dyn Fn() -> Oco<'static, str>>),
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

impl<T> IntoClass for (&'static str, T)
where
    T: Fn() -> bool + 'static,
{
    fn into_class(self) -> Class {
        Class::Fn(self.0.into(), Box::new(self.1))
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

macro_rules! class_list {
    ($($name:expr),+) => {
        {
            use crate::utils::class_list::ClassList;
            ClassList::new()$(.add($name))+
        }
    };
}
pub(crate) use class_list;

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

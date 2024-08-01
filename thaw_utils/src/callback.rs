use std::{ops::Deref, sync::Arc};

pub struct BoxCallback(Box<dyn Fn() + Send + Sync + 'static>);

impl BoxCallback {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        Self(Box::new(f))
    }
}

impl Deref for BoxCallback {
    type Target = Box<dyn Fn() + Send + Sync + 'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F> From<F> for BoxCallback
where
    F: Fn() + Send + Sync + 'static,
{
    fn from(value: F) -> Self {
        Self::new(value)
    }
}

pub struct BoxOneCallback<A, Return = ()>(Box<dyn Fn(A) -> Return + Send + Sync + 'static>);

impl<A, Return> BoxOneCallback<A, Return> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(A) -> Return + Send + Sync + 'static,
    {
        Self(Box::new(f))
    }
}

impl<A, Return> Deref for BoxOneCallback<A, Return> {
    type Target = Box<dyn Fn(A) -> Return + Send + Sync + 'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F, A, Return> From<F> for BoxOneCallback<A, Return>
where
    F: Fn(A) -> Return + Send + Sync + 'static,
{
    fn from(value: F) -> Self {
        Self::new(value)
    }
}

#[derive(Clone)]
pub struct ArcCallback(Arc<dyn Fn() + Send + Sync + 'static>);

impl ArcCallback {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        Self(Arc::new(f))
    }
}

impl Deref for ArcCallback {
    type Target = Arc<dyn Fn() + Send + Sync + 'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F> From<F> for ArcCallback
where
    F: Fn() + Send + Sync + 'static,
{
    fn from(value: F) -> Self {
        Self::new(value)
    }
}

#[derive(Clone)]
pub struct ArcOneCallback<A, Return = ()>(Arc<dyn Fn(A) -> Return + Send + Sync + 'static>);

impl<A, Return> ArcOneCallback<A, Return> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(A) -> Return + Send + Sync + 'static,
    {
        Self(Arc::new(f))
    }
}

impl<A, Return> Deref for ArcOneCallback<A, Return> {
    type Target = Arc<dyn Fn(A) -> Return + Send + Sync + 'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F, A, Return> From<F> for ArcOneCallback<A, Return>
where
    F: Fn(A) -> Return + Send + Sync + 'static,
{
    fn from(value: F) -> Self {
        Self::new(value)
    }
}

#[derive(Clone)]
pub struct ArcTwoCallback<A, B>(Arc<dyn Fn(A, B) + Send + Sync + 'static>);

impl<A, B> ArcTwoCallback<A, B> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(A, B) + Send + Sync + 'static,
    {
        Self(Arc::new(f))
    }
}

impl<A, B> Deref for ArcTwoCallback<A, B> {
    type Target = Arc<dyn Fn(A, B) + Send + Sync + 'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F, A, B> From<F> for ArcTwoCallback<A, B>
where
    F: Fn(A, B) + Send + Sync + 'static,
{
    fn from(value: F) -> Self {
        Self::new(value)
    }
}

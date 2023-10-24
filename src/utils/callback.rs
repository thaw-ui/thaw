use leptos::StoredValue;
use std::{fmt, future::Future, pin::Pin, rc::Rc};

pub struct AsyncCallback<In: 'static, Out: 'static = ()>(
    #[allow(clippy::complexity)] StoredValue<Rc<dyn Fn(In) -> Pin<Box<dyn Future<Output = Out>>>>>,
);

impl<In> fmt::Debug for AsyncCallback<In> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt.write_str("AsyncCallback")
    }
}

impl<In, Out> Clone for AsyncCallback<In, Out> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<In, Out> Copy for AsyncCallback<In, Out> {}

impl<In, Out> AsyncCallback<In, Out> {
    pub fn new<F, Fu>(f: F) -> Self
    where
        F: Fn(In) -> Fu + 'static,
        Fu: Future<Output = Out> + 'static,
    {
        let f = Rc::new(move |input: In| {
            let fut = f(input);
            Box::pin(fut) as Pin<Box<dyn Future<Output = Out>>>
        });
        Self(StoredValue::new(f))
    }

    pub async fn call(&self, input: In) -> Out {
        let f = self.0.get_value();
        f(input).await
    }
}

impl<F, In, Fu, Out> From<F> for AsyncCallback<In, Out>
where
    F: Fn(In) -> Fu + 'static,
    Fu: Future<Output = Out> + 'static,
{
    fn from(f: F) -> AsyncCallback<In, Out> {
        AsyncCallback::new(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::AsyncCallback;
    use leptos::create_runtime;

    struct NoClone {}

    #[test]
    fn clone_async_callback() {
        let rt = create_runtime();
        let callback = AsyncCallback::new(move |_no_clone: NoClone| async { NoClone {} });
        let _cloned = callback.clone();
        rt.dispose();
    }

    #[test]
    fn async_callback_from() {
        let rt = create_runtime();
        let _callback: AsyncCallback<(), String> = (|()| async { "test".to_string() }).into();
        rt.dispose();
    }

    #[test]
    fn async_callback_from_html() {
        let rt = create_runtime();
        use leptos::{
            html::{HtmlElement, H1},
            *,
        };

        let _callback: AsyncCallback<String, HtmlElement<H1>> = (|x: String| async move {
            view! {
                <h1>{x}</h1>
            }
        })
        .into();
        rt.dispose();
    }
}

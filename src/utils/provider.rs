/// https://github.com/leptos-rs/leptos/issues/2038
use leptos::*;

#[component]
pub fn Provider<T>(value: T, children: Children) -> impl IntoView
where
    T: Clone + 'static,
{
    run_as_child(move || {
        provide_context(value);
        children()
    })
}

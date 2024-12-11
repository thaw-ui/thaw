use leptos::prelude::*;

#[component]
pub fn ChevronDownRegularIcon() -> impl IntoView {
    view! {
        <svg fill="currentColor" aria-hidden="true" width="1em" height="1em" viewBox="0 0 20 20">
            <path
                d="M15.85 7.65c.2.2.2.5 0 .7l-5.46 5.49a.55.55 0 0 1-.78 0L4.15 8.35a.5.5 0 1 1 .7-.7L10 12.8l5.15-5.16c.2-.2.5-.2.7 0Z"
                fill="currentColor"
            ></path>
        </svg>
    }
}

#[component]
pub fn InfoRegularIcon() -> impl IntoView {
    view! {
        <svg
            fill="currentColor"
            aria-hidden="true"
            width="1em"
            height="1em"
            viewBox="0 0 16 16"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M8.5 7.5a.5.5 0 1 0-1 0v3a.5.5 0 0 0 1 0v-3Zm.25-2a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0ZM8 1a7 7 0 1 0 0 14A7 7 0 0 0 8 1ZM2 8a6 6 0 1 1 12 0A6 6 0 0 1 2 8Z"
                fill="currentColor"
            ></path>
        </svg>
    }
}

#[component]
pub fn Checkmark12FilledIcon() -> impl IntoView {
    view! {
        <svg
            fill="currentColor"
            aria-hidden="true"
            width="12"
            height="12"
            viewBox="0 0 12 12"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M9.76 3.2c.3.29.32.76.04 1.06l-4.25 4.5a.75.75 0 0 1-1.08.02L2.22 6.53a.75.75 0 0 1 1.06-1.06l1.7 1.7L8.7 3.24a.75.75 0 0 1 1.06-.04Z"
                fill="currentColor"
            ></path>
        </svg>
    }
}

#[component]
pub fn Checkmark16FilledIcon() -> impl IntoView {
    view! {
        <svg
            fill="currentColor"
            aria-hidden="true"
            width="16"
            height="16"
            viewBox="0 0 16 16"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M14.05 3.49c.28.3.27.77-.04 1.06l-7.93 7.47A.85.85 0 0 1 4.9 12L2.22 9.28a.75.75 0 1 1 1.06-1.06l2.24 2.27 7.47-7.04a.75.75 0 0 1 1.06.04Z"
                fill="currentColor"
            ></path>
        </svg>
    }
}
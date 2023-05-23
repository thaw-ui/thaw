use leptos::*;
use leptos_router::{Outlet, use_route, use_router};
use melt_ui::*;

#[component]
pub fn ComponentsPage(cx: Scope) -> impl IntoView {
    let router = use_router(cx);
    let route = use_route(cx);
    let selected = create_rw_signal(cx, String::from(""));
    create_effect(cx, move |_| {
        let path = route.original_path();
        let path2 = route.path();
        log!("{:?} {}", path, path2);
    });
    view! {cx,
        <div class="components-page-box">
            <aside>
                <Menu selected>
                    <MenuItem key="slider" label="slider" />
                </Menu>
            </aside>
            <main>
                <Outlet />
            </main>
        </div>
    }
}

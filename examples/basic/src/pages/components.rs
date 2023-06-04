use leptos::*;
use leptos_router::{use_location, use_navigate, Outlet};
use melt_ui::*;
use regex::Regex;

#[component]
pub fn ComponentsPage(cx: Scope) -> impl IntoView {
    let loaction = use_location(cx);
    let navigate = use_navigate(cx);
    let selected = create_rw_signal(cx, String::from(""));
    create_effect(cx, move |_| {
        let pathname = loaction.pathname.get();

        let re = Regex::new(r"^/components/(.+)$").unwrap();
        let Some(caps) = re.captures(&pathname) else {
            return;
        };
        let Some(path) = caps.get(1) else {
            return;
        };
        let path = path.as_str().to_string();
        selected.set(path);
    });

    create_effect(cx, move |value| {
        let selected = selected.get();
        if value.is_some() {
            _ = navigate(&format!("/components/{selected}"), Default::default());
        }
        selected
    });
    view! {cx,
        <div class="components-page-box">
            <aside>
                <Menu selected>
                    <MenuItem key="menu" label="menu" />
                    <MenuItem key="slider" label="slider" />
                    <MenuItem key="tabbar" label="tabbar" />
                    <MenuItem key="input" label="input" />
                    <MenuItem key="image" label="image" />
                    <MenuItem key="modal" label="madal" />
                    <MenuItem key="nav-bar" label="nav-bar" />
                </Menu>
            </aside>
            <main>
                <Outlet />
            </main>
        </div>
    }
}

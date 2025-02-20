use leptos::prelude::window;
use tabster::{create_tabster, types::TabsterCoreProps, Tabster};

/// Tries to get a tabster instance on the current window or creates a new one
/// Since Tabster is single instance only, feel free to call this hook to ensure Tabster exists if necessary
pub fn use_tabster() -> Tabster {
    let window = window();
    let tabseter = create_tabster(
        window,
        TabsterCoreProps {
            control_tab: Some(false),
            get_parent: None,
            auto_root: None,
        },
    );
    tabseter
}

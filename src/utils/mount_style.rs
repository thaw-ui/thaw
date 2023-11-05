use leptos::document;

pub fn mount_style(id: &str, content: &str) {
    let head = document().head().expect("head no exist");
    let style = head
        .query_selector(&format!("style[csr-id=\"thaw-{id}\"]"))
        .expect("query style element error");

    if style.is_some() {
        return;
    }

    let style = document()
        .create_element("style")
        .expect("create style element error");
    _ = style.set_attribute("csr-id", &format!("thaw-{id}"));
    style.set_text_content(Some(content));

    _ = head.append_child(&style);
}

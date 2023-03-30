use leptos::document;

pub fn mount_style<'a, F>(id: &'a str, f: F) -> &str
where
    F: FnOnce() -> (&'a str, &'a str),
{
    let head = document().head().expect("head no exist");
    let style = head
        .query_selector(&format!("style[csr-id=\"melt-{id}\"]"))
        .expect("query style element error");

    let (class_name, content) = f();
    if style.is_some() {
        return class_name;
    }

    let style = document()
        .create_element("style")
        .expect("create style element error");
    _ = style.set_attribute("csr-id", &format!("melt-{id}"));
    style.set_text_content(Some(content));

    _ = head.append_child(&style);

    class_name
}

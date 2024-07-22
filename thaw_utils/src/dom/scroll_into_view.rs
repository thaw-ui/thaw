use super::get_scroll_parent;
use web_sys::HtmlElement;

pub fn scroll_into_view(el: &HtmlElement) {
    if let Some(parent) = get_scroll_parent(el) {
        let parent_rect = parent.get_bounding_client_rect();
        let el_rect = el.get_bounding_client_rect();
        if el_rect.y() < parent_rect.y() {
            el.scroll_into_view_with_bool(true);
        } else if el_rect.y() + el_rect.height() > parent_rect.y() + parent_rect.height() {
            el.scroll_into_view_with_bool(false);
        }
    }
}

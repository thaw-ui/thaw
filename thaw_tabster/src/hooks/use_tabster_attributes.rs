use tabster::{get_tabster_attribute_plain, types, TABSTER_ATTRIBUTE_NAME};

/// Hook that returns tabster attributes while ensuring tabster exists
pub fn use_tabster_attributes(props: &types::TabsterAttributeProps) -> types::TabsterDOMAttribute {
    // A tabster instance is not necessary to generate tabster attributes
    // but calling the hook will ensure that a tabster instance exists internally and avoids consumers doing the same
    // useTabster();
    let str_attr = get_tabster_attribute_plain(&props);
    (TABSTER_ATTRIBUTE_NAME.to_string(), str_attr)
}
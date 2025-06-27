use linkml_meta::{poly_containers::MapRef, uri, uriorcurie, Prefix};

pub fn curie2uri<'a>(
    curie_or_uri: impl Into<uriorcurie>,
    prefixes: impl MapRef<'a, String, Prefix>,
) -> Option<uri> {
    let curie_or_uri: String = curie_or_uri.into();
    // check if its already an uri
    if curie_or_uri.contains("://") {
        return Some(curie_or_uri.to_string());
    }
    // check if it is of form prefix:ext
    if let Some((prefix, ext)) = curie_or_uri.split_once(':') {
        if let Some(prefix) = prefixes.get(&prefix.to_string()) {
            return Some(format!("{}{}", prefix.prefix_reference, ext));
        } else {
            return None;
        }
    } else {
        return None;
    }
}

// https://mastodon.social/@ssg@hachyderm.io/111688726604960018
pub fn remove_query_string(query_str: &str, key: &str) -> String {
    const SEPARATOR: &str = "&";

    let key_eq = format!("{key}=");

    query_str
        .split(SEPARATOR)
        .filter(|p| !p.starts_with(&key_eq))
        .collect::<Vec<&str>>()
        .join(SEPARATOR)
}

pub fn remove_query_string_with_strip_prefix(query_str: &str, key: &str) -> String {
    const SEPARATOR: &str = "&";

    query_str
        .split(SEPARATOR)
        .filter(|p| {
            if let Some(p) = p.strip_prefix(key) {
                !p.starts_with('=')
            } else {
                true
            }
        })
        .collect::<Vec<&str>>()
        .join(SEPARATOR)
}

// query_str
//     .split(SEPARATOR)
//     .with_value(|| format!("&key=")) {
//     .filter_map(|(p, v)| if !p.starts_with(v) p)
//     .collect::<Vec<&str>>()
//     .join(SEPARATOR)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_found() {
        let query_str = "abc&cd=ef&world";
        let expected = "abc&world";
        let result = remove_query_string(query_str, &"cd");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_key_not_found() {
        let query_str = "abc&cd=ef&world";
        let result = remove_query_string(query_str, &"ef");

        assert_eq!(result, query_str);
    }

    #[test]
    fn test_empty_input_str() {
        let query_str = "";
        let expected = "";
        let result = remove_query_string(query_str, &"cd");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_fn_strip_prefix() {
        let query_str = "abc&cd=ef&world";
        let expected = remove_query_string(query_str, &"cd");
        let actual = remove_query_string_with_strip_prefix(query_str, &"cd");
        assert_eq!(actual, expected);

        let expected = remove_query_string(query_str, &"ef");
        let actual = remove_query_string_with_strip_prefix(query_str, &"ef");
        assert_eq!(actual, expected);

        let result = remove_query_string("", &"cd");
        assert_eq!(result, "");
    }
}

use regex::Regex;

lazy_static! {
    static ref SNAKE_CASE_REGEX: Regex =
        Regex::new(r#"[A-Z]{2,}|[A-Z]?[a-z]+[0-9]*|[A-Z]|[0-9]+"#).unwrap();
}

pub fn to_snake_case(input: &str) -> String {
    let parts: Vec<String> = SNAKE_CASE_REGEX
        .find_iter(input)
        .map(|m| m.as_str().to_ascii_lowercase())
        .collect();
    parts.join("_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case_camel_case() {
        assert_eq!(to_snake_case("rowCount"), "row_count");
    }

    #[test]
    fn test_to_snake_case_upper_camel_case() {
        assert_eq!(to_snake_case("RowCount"), "row_count");
    }

    #[test]
    fn test_to_snake_case_upper_case() {
        assert_eq!(to_snake_case("ROW_COUNT"), "row_count");
    }

    #[test]
    fn test_to_snake_case_one_upper_case() {
        assert_eq!(to_snake_case("Q_OBJECT"), "q_object");
    }

    #[test]
    fn test_to_snake_case_snake_case() {
        assert_eq!(to_snake_case("row_count"), "row_count");
    }

    #[test]
    fn test_to_snake_number_1() {
        assert_eq!(to_snake_case("arg0"), "arg0");
    }

    #[test]
    fn test_to_snake_case_mixed_case_1() {
        assert_eq!(to_snake_case("QOBJECT_Ref"), "qobject_ref");
    }
}

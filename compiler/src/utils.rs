use std::mem::replace;

#[derive(Eq, PartialEq, Copy, Clone)]
enum CaseParserState {
    Sep,
    Upper,
    Lower,
}

struct CaseParser {
    prefix: String,
    part: String,

    parts: Vec<String>,
    underscores: usize,
    state: CaseParserState,
}

impl CaseParser {
    pub fn new() -> Self {
        Self {
            prefix: String::new(),
            part: String::new(),
            parts: vec![],
            underscores: 0,
            state: CaseParserState::Sep,
        }
    }

    fn finish_part(&mut self) {
        if self.parts.is_empty() {
            self.prefix = "_".repeat(self.underscores);
        }
        self.underscores = 0;
        self.parts.push(replace(&mut self.part, String::new()));
    }

    pub fn feed(&mut self, c: char) {
        let old_state = self.state;

        self.state = if c == '_' {
            if old_state != CaseParserState::Sep {
                self.finish_part();
            }
            self.underscores += 1;
            CaseParserState::Sep
        } else if c.is_ascii_uppercase() {
            if old_state == CaseParserState::Lower {
                self.finish_part();
            }
            self.part.push(c.to_ascii_lowercase());
            CaseParserState::Upper
        } else {
            self.part.push(c);
            CaseParserState::Lower
        };
    }

    pub fn finish(&mut self) -> String {
        let suffix = if self.state == CaseParserState::Sep {
            "_".repeat(self.underscores)
        } else {
            self.finish_part();
            String::new()
        };

        if suffix.is_empty() && self.prefix.is_empty() {
            self.parts.join("_")
        } else {
            format!("{}{}{}", self.prefix, self.parts.join("_"), suffix)
        }
    }
}

pub fn to_snake_case(input: &str) -> String {
    let mut parser = CaseParser::new();
    for c in input.chars() {
        parser.feed(c);
    }
    parser.finish()
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
    fn test_to_snake_case_number_upper_camel_case() {
        assert_eq!(to_snake_case("RowV4Count"), "row_v4_count");
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
    fn test_underscore() {
        assert_eq!(to_snake_case("_"), "_");
    }

    #[test]
    fn test_prefix_underscore() {
        assert_eq!(to_snake_case("__arg0"), "__arg0");
    }

    #[test]
    fn test_suffix_underscore() {
        assert_eq!(to_snake_case("arg0__"), "arg0__");
    }

    #[test]
    fn test_to_snake_case_mixed_case_1() {
        assert_eq!(to_snake_case("QOBJECT_Ref"), "qobject_ref");
    }
}

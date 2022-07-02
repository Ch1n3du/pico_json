#[cfg(test)]
mod tests {
    use crate::{model::*, parser::*};

    #[test]
    fn parses_number() {
        parse_json_str("42").unwrap();
    }

    #[test]
    fn parses_bool() {
        parse_json_str("true").unwrap();
    }

    #[test]
    fn parses_string() {
        parse_json_str("\"The old world is dead and the new world is struggling to be born now is the time of monsters.\"").unwrap();
    }

    #[test]
    fn parses_arrays() {
        parse_json_str("[1, 2, 3, 4, 5]").unwrap();
    }

    #[test]
    fn parses_json_object() {
        parse_json_str("[1, 2, 3, 4, 5]").unwrap();
    }
}

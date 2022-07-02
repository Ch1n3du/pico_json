use pico_json::parser::{parse_json_str,parse_json_file};

fn main() {
    println!("{}", parse_json_str("[1, 2, 3, 4, 5]").unwrap());
    println!("{}", parse_json_file("test.json").unwrap());
}

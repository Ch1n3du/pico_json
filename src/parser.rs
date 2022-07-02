use std::fs;

use peg::{error::ParseError, str::LineCol};

use crate::model::*;

pub fn parse_json_str(source: &str) -> Result<Json, ParseError<LineCol>> {
    json::json(source)
}

pub fn parse_json_file(file_path: &str) -> Result<Json, ParseError<LineCol>> {
    let source = fs::read_to_string(file_path).unwrap();
    parse_json_str(source.as_str())
}

peg::parser!(
    grammar json() for str {
        pub rule json() -> Json
          = _* v:(container() / primitive()) _* { v }

        rule container() -> Json
          = c:(json_array() / json_object())

        rule json_object() -> Json
          = "{" _* head:member()? _* rest:("," _* v:member() { v })* _* "}" {
            let mut json_object = JsonObject::from_vec(rest);

            if head.is_some() {
              json_object.insert(head.unwrap());
            }

            Json::Container(Container::JsonObject(json_object))
        }

        rule member() -> (String, Json)
          = k:string_literal() _* ":" _* v:json() { (k, v) }

        // "[" head ("," elem)* "]"
        rule json_array() -> Json
          = "[" _* head:json()? _* tail:("," _* elem:json() { elem })* _* "]" {
            let arr: Vec<Json> = if head.is_none() {
              vec![]
            } else {
              vec![head.unwrap()]
                .into_iter()
                .chain(tail.into_iter())
                .collect()
            };

            Json::Container(Container::JsonArray(JsonArray::new(arr)))
        }

        rule primitive() -> Json
          = p:(bool_primitive() / number_primitive() / string_primitive()) {
            Json::Primitive(p)
        }

        rule bool_primitive() -> Primitive
          = b:$("true" / "false") {
            Primitive::Boolean(b == "true")
        }

        rule number_primitive() -> Primitive
          = n:number_literal() { Primitive::Number(n) }

        rule string_primitive() -> Primitive
          = s:string_literal() {
            Primitive::String(s)
        }

        rule string_literal() -> String
          // = "\"" s:[^ '"']* !['\\']['\"'] {
          = "\"" s:[^ '"']* !['\\']['\"'] {
            String::from_iter(s)
        }

        rule number_literal() -> i64
          = n:$(['0'..='9']+) {?
            n.parse().or(Err("Error parsing number literal."))
        }

        rule _ = [' ' | '\n']
    }
);

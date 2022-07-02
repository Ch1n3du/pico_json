use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Primitive {
    Number(i64),
    String(String),
    Boolean(bool),
}

impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Primitive::*;

        match self {
            Number(n) => write!(f, "{}", n),
            String(s) => write!(f, "\"{}\"", s),
            Boolean(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Debug)]
pub struct JsonObject {
    items: HashMap<String, Json>,
}

impl JsonObject {
    pub fn from_vec(items: Vec<(String, Json)>) -> JsonObject {
        let mut map: HashMap<String, Json> = HashMap::new();
        for (k, v) in items {
            map.insert(k, v);
        }

        JsonObject { items: map }
    }

    pub fn insert(&mut self, v: (String, Json)) {
        self.items.insert(v.0, v.1);
    }
}

impl Display for JsonObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut acc = String::new();
        for (k, v) in self.items.iter() {
            acc = format!("{}  \"{}\": {},\n", acc, k, v);
        }

        write!(f, "{{ \n{}}}", acc)
    }
}


#[derive(Debug)]
pub struct JsonArray {
    items: Vec<Json>,
}

impl JsonArray {
    pub fn new(items: Vec<Json>) -> JsonArray {
        JsonArray { items }
    }
}

impl Display for JsonArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut acc = match self.items.len() {
            0 => String::from("["),
            1 => String::from(format!("[{}", self.items[0])),
            _ => String::from(format!("[ {}", self.items[0]))
        };

        for elem in self.items.iter() {
            acc = format!("{}, {}", acc, elem);
        }

        write!(f, "{}]", acc)
    }
}

#[derive(Debug)]
pub enum Container {
    JsonObject(JsonObject),
    JsonArray(JsonArray),
}

impl Display for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Container::*;

        match self {
            JsonObject(obj) => write!(f, "{}", obj),
            JsonArray(arr) => write!(f, "{}", arr),
        }
    }
}

#[derive(Debug)]
pub enum Json {
    Primitive(Primitive),
    Container(Container),
}

impl Display for Json {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Json::*;

        match self {
            Primitive(p) => write!(f, "{}", p),
            Container(c) => write!(f, "{}", c),
        }
    }
}
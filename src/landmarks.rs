use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref LANDMARKS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("James", vec!["user", "admin"]);
        map.insert("Jim", vec!["user"]);
        map
    };
}
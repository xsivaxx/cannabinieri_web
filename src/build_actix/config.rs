use std::fs;
use serde_json::Value;

pub const LOC_FILE: &str = "./lang.json";

// functions executed at runtime
lazy_static! {
    pub static ref LOC : Value = init_lang();
}

// open LOC file as json Value
fn init_lang() -> Value {
    let lang_str = fs::read_to_string( LOC_FILE ).expect("error reading file to string");

    serde_json::from_str(&lang_str).expect("init_lang(): Can't parse translations file")
}
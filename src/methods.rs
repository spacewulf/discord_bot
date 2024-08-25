use std::fs;
use std::io;

pub fn rem_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next_back();
    chars.next_back();
    

    return chars.as_str();
}

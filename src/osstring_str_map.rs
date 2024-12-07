use std::ffi::{OsStr, OsString};

fn foo() {
    let s1: OsString = OsString::from("Hello there");
    if s1.to_str().map(|s| s == "Hello there").unwrap_or_default() {
        println!("Matches!");
    }

    let s2 = OsStr::new("Hello there");
    if s1.to_str().map(|s| s != s2).unwrap_or_default() {
        println!("Does not match!");
    }
}

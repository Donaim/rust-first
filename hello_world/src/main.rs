#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

struct Obj {
    index: i32
}

extern crate hello_lib;
use hello_lib::zoo::*;

fn main() {
    let duck = duck::new("Patrick".to_string());
    duck.poke();
    duck.ask("name");
    duck.ask("3.14");
    duck.ask("meaning");
}
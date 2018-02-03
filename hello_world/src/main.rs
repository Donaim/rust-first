#![allow(unused_variables)]
#![allow(unused_imports)]

struct Obj {
    index: i32
}

extern crate hello_lib;
use hello_lib::hello_namespace;

fn main() {
    // let x = 5;
    // let y = x * 2;
    // println!("HELLO");
    // println!("lul");

    // let o = Obj { index: 10};
    hello_namespace::say_hello();
}
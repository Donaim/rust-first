#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate sharer;
use sharer::*;
extern crate libloading;
use libloading::{Symbol, Library};

#[cfg(unix)]
const LIBPATH: &'static str = "/home/d0naim/dev/learn/rust-first/dynamic_test_lib/target/debug/libdynamic_test_lib.so";
#[cfg(windows)]
const LIBPATH: &'static str = "C:\\Users\\d0naim\\Documents\\dev\\rust-first\\dynamic_test_lib\\target\\debug\\dynamic_test_lib.dll";

#[repr(C)] // can share even without this attribute
pub struct ObjT {
    a: i32
}

fn load_lib() {
    let lib = Library::new(LIBPATH).unwrap();
    unsafe {
        let f: Symbol<unsafe extern fn() -> &'static Sharable> = lib.get(b"test_identity_struct\0").unwrap();
        let x = f.into_type::<unsafe extern fn() -> &'static ObjT>();
        println!("{}", x().a);

        println!("all fine");
    }
}



fn main() {
    load_lib();
}
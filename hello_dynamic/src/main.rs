#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::mem;

extern crate sharer;
use sharer::*;
extern crate libloading;
use libloading::{Symbol, Library};


pub trait TestTrait {
    fn work(&self);
}

#[derive(Debug)]
pub struct TestObj47 {
    name: String,
}

impl TestTrait for TestObj47 {
    fn work(&self) {
        println!("working...");
    }
}

fn convert_ptr<T, R>(trait_ref: T) -> &'static R
{
    /*
        use:
            let a = TestObj47{name: "KEK".to_string()};
            let b = &a as &TestTrait;

            let c = convert_ptr::<&TestTrait, TestObj47>(b);
    */
    unsafe {
        let trait_ref_arr = mem::transmute::<&T, [u8; 64 / 8]>(&trait_ref);
        let obj2: &'static R = mem::transmute::<[u8; 64 / 8], &R>(trait_ref_arr);
        return obj2;
    }
}

fn trait_to_struct() {
    let obj = &TestObj47{name: "Hello".to_string()};
    unsafe {
        let trait_ref: &TestTrait = obj as &TestTrait;

        let obj_ref_arr = mem::transmute::<&TestObj47, [u8; 64 / 8]>(obj);
        println!("{:?} = \t{:?}", stringify!(obj_ref_arr), obj_ref_arr);
        let trait_ref_arr = mem::transmute::<&TestTrait, [u8; 128 / 8]>(trait_ref);
        println!("{:?} = \t{:?}", stringify!(trait_ref_arr), trait_ref_arr);
        // this shows that first 8 bytes of trait adress are the same as obj adress

        let mut obj_ref_arr2 : [u8; 64/8] = [0; 64/8];
        for i in 0..8 {
            obj_ref_arr2[i] = trait_ref_arr[i];
        }
        println!("{:?} = \t{:?}", stringify!(obj_ref_arr2), obj_ref_arr2);

        let obj2 = mem::transmute::<[u8; 64 / 8], &TestObj47>(obj_ref_arr2);
        println!("{:?} = \t\t{:?}", stringify!(obj2), obj2);
    }
}


const LIBPATH: &'static str = "/home/d0naim/dev/learn/rust-first/dynamic_test_lib/target/release/libdynamic_test_lib.so";

fn load_lib() {
    let lib = Library::new(LIBPATH).unwrap();
    unsafe {
        let f: Symbol<unsafe extern fn() -> &'static Sharable> = lib.get(b"test_get_trait\0").unwrap();
        let sh = f();

        println!("all fine");
    }
}

fn main() {
    // trait_to_struct();

    // let a = TestObj47{name: "KEK".to_string()};
    // let b = &a as &TestTrait;

    // let c = convert_ptr::<&TestTrait, TestObj47>(b);

    load_lib();
}
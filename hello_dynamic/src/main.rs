#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::mem;

extern crate sharer;
use sharer::*;
extern crate libloading;
use libloading::{Symbol, Library};

extern crate dynamic;
use dynamic::*;


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
pub trait LulTrait {
}


fn convert_ptr<T, R>(trait_ref: T) -> &'static R
{
    /*
        usage:
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

fn convert_ptr_ez<T, R>(trait_ref: T) -> &'static R
{
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


const LIBPATH: &'static str = "/home/d0naim/dev/learn/rust-first/dynamic_test_lib/target/debug/libdynamic_test_lib.so";

#[repr(C)] // can shar even without this attribute
pub struct ObjT {
    a: i32
}

use std::ops::Deref;
fn deref<'a, A, T>(x: Symbol<A>) -> &T {
    unsafe {
        // Additional reference level for a dereference on `deref` return value.
        mem::transmute(&x.pointer)
    }
}

fn load_lib() {
    let lib = Library::new(LIBPATH).unwrap();
    unsafe {
        // let f: Symbol<unsafe extern fn() -> fn(i32) -> i32> = lib.get(b"get_func\0").unwrap();
        let f: Symbol<unsafe extern fn() -> &'static Sharable> = lib.get(b"test_identity_struct\0").unwrap();
        // let ff = f();
        // let x = ff(2);
        // println!("f(2) = {}", x);
        // println!("{:?}", f().name());
        // println!("{}", f().a);

        println!("all fine");
    }
}



fn main() {
    load_lib();
    // let z = get_sharable();

    // let x = Dynamic::new(TestObj47{name: "zulul".to_string()});
    // println!("x = {:?}", x);
    // println!("x as i32 = {:?}", x.downcast_ref::<TestObj47>());
}
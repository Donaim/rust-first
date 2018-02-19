#![crate_type="dylib"] // FIXME: should become a cdylib in due time
#![allow(dead_code)]

extern crate sharer;
use sharer::*;

pub struct Obj123<'a> {
    name: &'a str,
}
impl <'a> Obj123<'a> {
    fn hello(&self) {
        println!("{:?} says hello", self.name);
    }
}

pub trait ObjTrait {
    fn hitrait(&self) {
        println!("objtrait: hi");
    }
}
impl <'a> ObjTrait for Obj123<'a> {

}

impl <'a> Sharable for Obj123<'a> {
    // fn name(&self) -> &'static str {
    //     return "Name is: Obj123"
    // }
}

#[no_mangle]
pub extern "C" fn get_func() -> (fn(i32) -> i32) {
    return |x| x * x
}

#[no_mangle]
pub extern "C" fn get_lul_obj() -> Lul {
    return Lul::new()
}
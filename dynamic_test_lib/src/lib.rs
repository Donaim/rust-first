#![allow(dead_code)]
#![crate_type = "dylib"]

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

impl <'a> Sharable for Obj123<'a> { }

#[no_mangle]
pub extern "C" fn test_get_trait() -> &'static Sharable {
    return &(Obj123::<'static>{name: "some name"}) as &'static Sharable
}

pub fn get_trait<'a>() -> &'a Sharable {
    return &(Obj123::<'a>{name: "some name"}) as &'a Sharable;
}
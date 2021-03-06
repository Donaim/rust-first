#![crate_type="dylib"] // FIXME: should become a cdylib in due time
#![allow(dead_code)]

extern crate sharer;
use sharer::*;

#[repr(C)] // can shar even without this attribute
pub struct ObjT {
    a: i32
}
impl Sharable for ObjT {
    fn name(&self) -> &'static str {
        return "Named somehow"
    }
}

#[no_mangle]
pub extern "C" fn test_identity_struct() -> &'static Sharable {
    &ObjT{a: 10}
}
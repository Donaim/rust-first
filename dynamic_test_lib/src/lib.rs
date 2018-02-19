#![crate_type="dylib"] // FIXME: should become a cdylib in due time
#![allow(dead_code)]

extern crate sharer;
use sharer::*;

#[no_mangle]
pub extern "C" fn test_identity_struct() -> Lul {
    Lul::new()
}
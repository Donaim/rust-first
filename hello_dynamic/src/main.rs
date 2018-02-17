#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

pub trait TestTrait {
    fn work(&self);
}
pub trait FalseTrait {
    // add code here
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

fn trait_to_struct() {
    unsafe {
        let obj = &TestObj47{name: "Hello".to_string()};
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

fn main() {
    trait_to_struct();
}
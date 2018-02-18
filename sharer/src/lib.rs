#![allow(dead_code)]

pub fn get_sharable<'a>() -> &'a Sharable {
    return &Lul{id: 10}
}

pub trait Sharable {
    fn name(&self) -> &'static str {
        return "Unnamed"
    }
}
struct Lul {
    id: i32,
} 

impl Sharable for Lul {
    
}
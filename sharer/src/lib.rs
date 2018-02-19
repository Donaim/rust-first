#![allow(dead_code)]

pub fn get_sharable<'a>() -> &'a Sharable {
    return &Lul{id: 10}
}

pub trait Sharable {
    fn name(&self) -> &'static str {
        return "Unnamed"
    }
}

pub struct Lul {
    id: i32,
}
impl Lul {
    pub fn new() -> Lul {
        Lul {id: -1}
    }
}

impl Sharable for Lul {
    
}
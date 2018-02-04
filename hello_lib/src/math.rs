#![allow(unused_variables)]
#![allow(dead_code)]

pub struct Natural {
    n: i32,
}
impl Natural {
    pub fn new(n: i32) -> Self { Natural{n: n} }
}

impl Into<Natural> for i32 {
    fn into(self) -> Natural {
        if self > 0 { return Natural{n: self} }
        else { panic!("NOT CONVERABLE TO NATURAL NUMBER!!"); }
    }
}

pub trait IsPrime {
    fn is_prime(&self) -> bool; 
}
impl IsPrime for Natural {
    fn is_prime(&self) -> bool {
        for i in 1..self.n {
            if self.n % i == 0 { return false }
        }
        return true
    }
}
impl IsPrime for i32 {
    fn is_prime(&self) -> bool {
        let x: Natural = (*self).into();
        return x.is_prime()
    }
}

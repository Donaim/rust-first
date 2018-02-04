#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug)]
pub struct Natural {
    x: i32,
}
impl Natural {
    pub fn new(n: i32) -> Self 
    {
        if n > 0 { Natural{x: n} }
        else { panic!("Integer is negative -> cannot convert to Natural"); }
    }
    pub fn newt(n: i32) -> Option<Self> 
    { 
        if n > 0 { Some(Natural{x: n}) }
        else { None }
    }
    pub fn n(&self) -> i32 { self.x }
}

pub trait IsPrime {
    fn is_prime(&self) -> bool; 
}
impl IsPrime for Natural {
    fn is_prime(&self) -> bool {
        for i in 2..self.x {
            if self.x % i == 0 { return false }
        }
        return true
    }
}
impl Into<Natural> for i32 {
    fn into(self) -> Natural { Natural::new(self) }
}
// impl IsPrime for i32 {
//     fn is_prime(&self) -> bool {
//         let x: Natural = Natural::newt(*self).unwrap();
//         return x.is_prime()
//     }
// }

pub fn primes_before_n<T: Into<Natural>>(x: T) -> i32 {
    let nat = x.into();
    let mut count = 0;
    for i in 1..nat.x {
        if Natural::new(i).is_prime() { count += 1; }
    }
    return count
} 
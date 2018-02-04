#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

struct Obj {
    index: i32
}

extern crate hello_lib;
use hello_lib::zoo::*;
use hello_lib::math::*;

fn main() {
    println!("{:?}", is_prime(-1));
    println!("{:?}", is_prime(10));
    println!("{:?}", is_prime(7));

    println!("Primes before {} is: {}", -3, primes_before_n(-3));
    println!("Primes before {} is: {}", 100, primes_before_n(100));
}
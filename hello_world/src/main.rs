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
    // let duck = duck::new("Patrick".to_string());
    // duck.poke();
    // duck.ask("name");
    // duck.ask("3.14");
    // duck.ask("meaning");

    println!("{:?}", is_prime(-1));

    // let x = 10;
    // if let Some(z) = Natural::newt(x) {
    //     print!("{:?} is ", z);
    //     if z.is_prime() { println!("a prime"); }
    //     else { println!("not a prime"); }
    //     println!("Primes before {} is: {:?}", x, primes_before_n(x));
    // }
    // else {
    //     println!("wrong num: {}, must be > 0", x);
    // }

    // println!("{}", (-1).is_prime());
}
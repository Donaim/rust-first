#![allow(dead_code)]

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

pub fn get_trait<'a>() -> &'a ObjTrait {
    return &(Obj123::<'a>{name: "some name"}) as &'a ObjTrait;
}
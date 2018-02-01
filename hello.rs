
extern crate rary;

mod namespace {
	pub struct Obj {
		pub name: i32
	}

	impl Obj {
		pub fn new(num: i32) -> Obj {
			Obj{name: num}
		}
	}
}

fn typeid<T: std::any::Any>(_: &T) {
    println!("{:?}", std::any::TypeId::of::<T>());
}

fn main() {
	// let x = namespace::Obj::new(3);
	// let y = namespace::Obj{name: 10};

	// println!("HELLO");

	let x = vec![1, 2, 3];

	rary::public_function();
	rary::indirect_access();

	// typeid(&x);
}

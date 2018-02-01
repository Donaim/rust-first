
struct Obj {
	name: i32
}

impl Obj {
	fn new(num: i32) -> Obj {
		Obj{name: num}
	}
}

fn main() {
	let x = Obj::new(3);

	println!("HELLO");
}

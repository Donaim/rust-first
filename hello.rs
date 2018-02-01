
extern crate rary;
#[cfg(all(feature="winit", feature="glium"))] #[macro_use] extern crate conrod;
#[cfg(all(feature="winit", feature="glium"))] mod support;


fn main() {
	println!("Conrod here!!!!");

    use conrod;
    use conrod::backend::glium::glium;
    use conrod::backend::glium::glium::Surface;

	const WIDTH: u32 = 800;
	const HEIGHT: u32 = 600;

	// Build the window.
	let mut events_loop = glium::glutin::EventsLoop::new();
	let window = glium::glutin::WindowBuilder::new()
		.with_title("Canvas")
		.with_dimensions(WIDTH, HEIGHT);
	let context = glium::glutin::ContextBuilder::new()
		.with_vsync(true)
		.with_multisampling(4);
	let display = glium::Display::new(window, context, &events_loop).unwrap();
}

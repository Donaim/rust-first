pub mod math;

pub mod zoo {
    pub mod duck {
        pub fn new(name: String) -> Duck {
            Duck { name: name }
        }

        pub struct Duck {
            pub name: String,
        }

        impl Duck {
            pub fn poke(&self) {
                println!("Quack");
            }
            pub fn ask(&self, question: &str) {
                match question {
                    "name" => println!("My name is {:?}", self.get_name()),
                    "meaning" => println!("42"),
                    _ => {
                        if let Some(x) = question.to_string().parse::<f32>().ok() {
                            self.calculate(x as f32);
                        }
                        else {
                            eprintln!("Duck does not know the answer !!!! ");
                        }
                    }
                };
            }
            fn calculate(&self, cube: f32) {
                println!("I think that {} cubed is {}", cube, cube * cube * cube);
            }
        }
        pub trait DucksMemory {
            fn get_name(&self) -> String;
        }

        impl DucksMemory for Duck {
            fn get_name(&self) -> String {
                self.name.to_string()
            }
        }
    }
}
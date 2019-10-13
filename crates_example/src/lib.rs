pub mod geometry {

    fn init_canvas() {
        println!("Initializing canvas");
    }

    pub mod shapes {
        #[derive(Debug)]
        pub struct Square {
            side: u32
        }

        impl Square {
            pub fn area(&self) -> u32 {
                return self.side * self.side;
            }

            pub fn perimeter(&self) -> u32 {
                return self.side * 4;
            }

            pub fn new(side: u32) -> Square {
                return Square { side: side };
            }

            pub fn draw(&self) {
                super::init_canvas();
                println!("Draw square");
            }
        }

        #[derive(Debug)]
        pub struct Circle {
            radius: f64
        }

        impl Circle {
            pub fn area(&self) -> f64 {
                return 3.14 * self.radius * self.radius;
            }

            pub fn perimeter(&self) -> f64 {
                return 2.0 * 3.14 * self.radius;
            }

            pub fn new(radius: f64) -> Circle {
                return Circle { radius: radius };
            }
        }
    }
}



#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    // Method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function: does not take &self
    fn square(side: u32) -> Rectangle {
        Rectangle { width: side, height: side }
    }
}

// Tuple structs
struct Point(u32, u32);

fn main() {
    // Creating a Rectangle
    let rect1 = Rectangle{width: 2, height: 4};
    println!("Area of {:?} is {}", rect1, rect1.area());

    // Another way of creating a rectangle
    let width = 4;
    let height = 9;
    let rect2 = Rectangle { width: width, height: height };
    println!("Area of {:?} is {}", rect2, rect2.area());

    // Another way: field init shorthand
    let width = width + 1;
    let height = height + 1;
    let rect3 = Rectangle { width, height }; //5 X 10
    println!("Area of {:?} is {}", rect3, rect3.area()); 

    // Struct update syntax
    let rect4 = Rectangle { width: 10, ..rect3 }; // 10 X 10
    println!("Area of {:?} is {}", rect4, rect4.area()); 

    // Struct update syntax again
    let rect5 = Rectangle { height: 5, ..rect3 }; // 5 X 5
    println!("Area of {:?} is {}", rect5, rect5.area());

    // Using tuple structs
    let x1y1 = Point(0, 5); //height is 5
    let x2y2 = Point(9, 0); //width is 9
    let rect6 = Rectangle { width: x2y2.0 - x1y1.0, height: x1y1.1 - x2y2.1 };
    println!("Area of {:?} is {}", rect6, rect6.area());

    // Pretty printing
    println!("{:#?}", rect6);

    println!("Can {:?} hold {:?}? {}", rect4, rect5, rect4.can_hold(&rect5));
    println!("Can {:?} hold {:?}? {}", rect3, rect4, rect3.can_hold(&rect4));

    // Use associated function
    let rect7 = Rectangle::square(19);
    println!("Area of {:?} is {}", rect7, rect7.area());
}

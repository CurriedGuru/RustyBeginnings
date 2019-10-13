use crates_example::geometry::shapes::Circle;

fn main() {
    let c = Circle::new(2.0);
    println!("For circle {:?}, perimeter = {}, area = {}", c, c.perimeter(), c.area());    
}

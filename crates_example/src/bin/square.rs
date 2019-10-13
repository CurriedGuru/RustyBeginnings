use crates_example::geometry::shapes::Square;

fn main() {
    let c = Square::new(2);
    println!("For square {:?}, perimeter = {}, area = {}", c, c.perimeter(), c.area());    
}

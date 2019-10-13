use crates_example::geometry::shapes::Square as GSquare;

fn main() {
    let c = GSquare::new(2);
    println!("For square {:?}, perimeter = {}, area = {}", c, c.perimeter(), c.area());    
}

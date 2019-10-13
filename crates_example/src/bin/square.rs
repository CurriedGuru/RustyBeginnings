use crates_example::geometry::shapes::Square as GSquare;
use  crates_example::geometry::Rng;

fn main() {
    let side = rand::thread_rng().gen_range(1, 10);
    let c = GSquare::new(side);
    println!("For square {:?}, perimeter = {}, area = {}", c, c.perimeter(), c.area());    
}

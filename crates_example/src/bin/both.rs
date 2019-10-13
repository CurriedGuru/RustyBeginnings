use crates_example::geometry::{Rng, shapes::Square, shapes::Circle};

fn main() {
    let c = Circle::new(2.0);
    println!("For circle {:?}, perimeter = {}, area = {}", c, c.perimeter(), c.area());

    let side = rand::thread_rng().gen_range(1, 10);
    let s = Square::new(side);
    println!("For square {:?}, perimeter = {}, area = {}", s, s.perimeter(), s.area());
}

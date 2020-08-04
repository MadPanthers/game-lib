mod memory;
use memory::{colour, position, Memory};

fn main() {
    let mut m = Memory::new(10, 10);
    m.set(
        position::Position { x: 1, y: 1 },
        colour::Colour::new(0, 0, 0),
    );

    println!("{:?}", m.get(position::Position { x: 1, y: 1 }));

    // println!("{:?}", m);
}

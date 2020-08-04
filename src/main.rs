mod memory;

fn main() {
    let m = memory::Memory::new(10, 10);

    println!("{:?}", m);
}

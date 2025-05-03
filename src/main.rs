use math;

fn main() {
    println!("From the math submodule: {}", math::add(20, 20));
    println!("From the math submodule's divide: {:?}", math::divide(10, 3));
}

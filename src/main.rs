use math;

fn main() 
{
    println!("From the math addition submodule: {}", math::add(20, 20));
    println!("From the math submodule's divide: {:?}", math::divide(10, 3));
    println!("From the math submodule's subtract: {}", math::subtract(10, 3));
    println!("From the math submodule's multiplication: {}", math::multiplication(10, 3));


}

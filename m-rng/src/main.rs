use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    
    for i in 1..20{
        let n1: u8 = rng.random();
        println!("random number u8: {} ", n1);
    };
    
    println!("Hello, world!");
}

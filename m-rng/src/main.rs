use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::rng();

    let mut matrix: Vec<Vec<u8>> = vec![];
    let mut v1: Vec<u8> = vec![];
    let mut v2: Vec<u8> = vec![];
    let mut v3: Vec<u8> = vec![];

    // num Input

    /*
    println!("Enter a name:");

    let mut guess = String::new();
    
    let s: String = guess.to_string();

    io::stdin().read_line(&mut guess).expect("failed to readline");

    let number: i32 = s.parse().expect("Not a valid number");
    let k: i32 = 3 + number; 

    print!("You entered {}", k);
    */
    
    // ends here

    for _i in 1..10{
        let n1: u8 = rng.random();
        v1.push(n1);
    };

    for _i in 1..10{
        let n1: u8 = rng.random();
        v2.push(n1);
    };

    for _i in 1..10{
        let n1: u8 = rng.random();
        v3.push(n1);
    };

    matrix.push(v1);
    matrix.push(v2);
    matrix.push(v3);

    println!("\n matrix {:?}", matrix);

    println!("Hello, world!");
}

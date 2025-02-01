use rand::Rng; 
use std::{any::type_name_of_val, io};

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

   
    let mut input = String::new();

    println!("enter a number: ");

    io::stdin()
        .read_line(&mut input)
        .expect("failed to readline");

    let num = input.trim().parse::<usize>().expect("please enter a number");

    println!("you entered: {}", num); 

    println!(type_name_of_val(num));

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
    
    /* 
    let mut rng = rand::rng();

    let mut rand_arr: Vec<u8> = vec![];

    let mut input = String::new();
    println!("Enter an index (0-{}):", values.len() - 1);

    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse::<usize>() {
        Ok(index) if index < values.len() => {
            let array_select: u8 = rng.random_range(1..3);
    
            rand_arr.push(matrix[array_select[input]]);
        }
        Ok(_) => println!("Error: Index out of bounds!"),
        Err(_) => println!("Error: Please enter a valid number."),
    }

    for i in 0..20{
               rand_arr.push(matrix[array_select[num]]);                 
    } */

    println!("\nmatrix {:?}", matrix);

    println!("Hello, world!");

}

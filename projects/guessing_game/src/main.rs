use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");

    println!("Guessing game begins");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
    
    println!("Please enter your input");
    let mut guess = String::new();
    io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(abc) => abc,
        Err(_) => continue,
    };
    
    println!("You guessed {}", guess);

    let apples = 5;
    println!("Apples {}",apples);
    // apples = apples + 1; //Immutable variable, cant assign to the same name
    // println!("Apples {}",apples);

    // println!("The secret number is: {}", secret_number);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {println!("Too equal");
                            break;}
    } //end match
    } //end loop
} //end main

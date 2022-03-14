use std::io; // std library to use io operations (taking user input)
use rand::Rng; // Random Number generator using dependency / crate
use std::cmp::Ordering; // std library to use cmp(compare function)

fn main() {
    println!("Guess The Number Game\n");
    
    // Generating a random Number
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Secret number is {}", secret_number);
    println!("Secret Number has been generated between 1 and 100\n");
    
    loop {
        //Taking User Input
        println!("What's your guess? (use ^C to quit)");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line\n");
        
        // Parsing and converting input to integer from string
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        // Comparing user input to random number generated
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small\n"),
            Ordering::Greater => println!("Too Big\n"),
            Ordering::Equal => {
                println!("Congrats you guessed the Number!");
                break;
            },
        }
    }
}

use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::io::Write;

fn main() {
    println!("Guess the number!");

    
    // let secret_number = rand::thread_rng().gen_range(1..101);
    // let secret_number = rand::thread_rng().range(1..101);
    let secret_number = rand::rng().random_range(1..101);

    // println!("The secret number is: {}", secret_number);
    
    loop {
        print!("Please input your guess: ");
        let mut guess = String::new();

        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin().read_line(&mut guess).
        expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You gussed: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            } 
        }
    }
}
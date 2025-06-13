use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::io::Write;

fn main() {
    println!();
    println!("Guess the number between 1 and 100 (inclusive).");
    println!("__________________");
    println!();
    
    
    // let secret_number = rand::thread_rng().gen_range(1..101);
    // let secret_number = rand::thread_rng().range(1..101);
    let secret_number = rand::rng().random_range(1..101);
    
    // println!("The secret number is: {}", secret_number);
    
    let mut count = 0;
    loop {
        
        print!("Please input your guess: ");
        let mut guess = String::new();

        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin().read_line(&mut guess).
        expect("Failed to read line");
    
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }
        
        // println!("You gussed: {}", guess);
        
        count += 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                println!("__________________");
            },
            Ordering::Greater => {
                println!("Too big!");
                println!("__________________");
            }, 
            Ordering::Equal => {
                println!();
                println!("You won!, in {count} tries");
                break;
            } 
        }
    }
}
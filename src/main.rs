use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    let mut guess_count: u32 = 0;

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        guess_count = guess_count + 1;

        println!("{}º guess", guess_count);

        println!("Please input your guess.");
        
        let mut guess = String::new();
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("You needed {} guesses. Can you do better?", guess_count);
                break;
            },
        }
    }
}

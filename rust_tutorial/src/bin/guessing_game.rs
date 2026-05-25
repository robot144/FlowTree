use std::io;

fn main() {
    let secret = rand::random::<u32>() % 100 + 1;

    loop {
        println!("Guess a number (1-100):");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        if guess < secret {
            println!("Your guess was too small");
        } else if guess > secret {
            println!("Your guess was too large");
        } else {
            println!("Correct!");
            break;
        }
    }
}

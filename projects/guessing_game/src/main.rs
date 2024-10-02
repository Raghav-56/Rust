use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

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

/*

PS D:\Programming\Rust\projects\guessing_game> cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target\debug\guessing_game.exe`
Guess the number!
Please input your guess.
50
You guessed 50

Too small!
75
You guessed 75

Too big!
63
You guessed 63

Too small!
69
You guessed 69

You win!
*/

use std::cmp::Ordering;
use rand::Rng;
use std::io;
use std::io::Write;

const MAX_ATTEMPTS: usize = 7;
const RANGE: (u32, u32) = (1, 101);


fn main() {
    let mut arr = [0; MAX_ATTEMPTS - 1];
    let mut attempts = 0;
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(RANGE.0, RANGE.1);
    // 💖 U+1F496

    let sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
    println!("{} The secret number is: {}", sparkle_heart, secret_number);

    let (mut low, mut high) = (RANGE.0, RANGE.1);

    while attempts < MAX_ATTEMPTS {
        let next_try = low + (high - low) / 2;
        println!("ℹ️  Attempts left: {} Last guesses: {:?}", MAX_ATTEMPTS - attempts, &arr[0..attempts]);
        print!("🤔 Please input your guess [{}]: ", next_try);
        io::stdout().flush().expect("Error");

        let mut guess = String::new();

        std::io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => match guess.trim().len() {
                0 => next_try,
                _ => {
                    println!("Error: {:?}", e);
                    continue;
                }
            },
        };
        println!("💬 You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("⬆  Too small!");
                low = guess;
            }
            Ordering::Greater => {
                println!("⬇  Too big!");
                high = guess;
            }
            Ordering::Equal => {
                println!("{0}{0}{0} You win! {0}{0}{0}", sparkle_heart);
                break;
            }
        }
        attempts += 1;
        if attempts == MAX_ATTEMPTS {
            println!("😡😡😡 You lost! 😡😡😡");
            break;
        }
        arr[attempts - 1] = guess;
    }
}

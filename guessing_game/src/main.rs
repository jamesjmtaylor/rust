use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let answer = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess (1-100).");
        //`let` is a constant. `mut` indicates mutability (effectively becomes a var)
         //`::` indicates a static function (as opposed to `.` which is a concrete function)
        let mut guess = String::new();

        //`&` indicates pass-by-reference (default is pass-by-value)
        //References immutable by default, hence repeat of `mut`
        io::stdin()
            .read_line(&mut guess) 
            .expect("Failed to read line");
            
        //NOTE: Rust allows name shadowing, so this could be just `guess`
        //`parse()` converts a string to the type that of your new variable (`u32`).
        //`match` is a switch statement for possible cases (called `arms`)
        //This match handles a good parse result `OK` and a parse exception`Err`.
        //`_` is a catchall for all exception types.
        let guessInt : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guessInt.cmp(&answer) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
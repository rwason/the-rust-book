use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // generate a random number
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // `mut` keyword creates a mutable variable
        // `::new` indicates the `new` is an associated function of the String type (i.e. a static method)
        let mut guess = String::new(); // creates a mutable variable due to `mut` keyword

        // `&mut guess` indicates a mutable reference to the variable guess
        // `read_line` returns a io::Result which is an enum
        // io::Result is an enum which can take on an Ok and Err variant
        // `expect` panics if the Result is an Err value
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // shadow previous value of `guess`
        // set the type for `guess` to be u32
        // trim whitespace/newlines, parse string into the type specified
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // format string
        println!("You guessed: {}", guess);

        // match is effectively a switch statement
        // each "case" is called an arm that can have an associated code block to execute
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You guessed it!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
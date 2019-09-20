pub fn run() {
    println!("variables!");

    // constant declaration
    const MAX_U8: u8 = 255;
    const MAX_I8: i8 = 127;
    println!("max u8: {}, max i8: {}", MAX_U8, MAX_I8);

    // shadowing variables using let
    // let creates a new variable so we can assign a "new" value and type
    let x = 5;
    // invalid operation due to x being immutable
    // x = 3;
    let x = x + 1;
    let x = x * 2;
    println!("the value of x is: {}", x);

    /*
    invalid operation -- can't update the type of a mutable variable (unless let is used)

    let mut spaces = " ";
    spaces = spaces.len();
    */
}
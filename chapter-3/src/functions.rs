pub fn run() {
    println!("functions!");

    another_function(5);

    let x = plus_one(5);
    another_function(x);
}

fn another_function(x: i32) {
    println!("input: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
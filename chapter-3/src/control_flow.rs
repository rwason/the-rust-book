pub fn run() {
    println!("control flows!");

    is_number("5");
    is_number("a");

    is_greater_than_five(5);
    is_greater_than_five(6);

    count_to(5);

    let arr = [100, 99, 98, 97, 96];
    iter(arr);
}

fn is_number(x: &str) {
    let num = x.parse::<i32>();

    if num.is_ok() {
        println!("the input {} was a number!", x);
    } else {
        println!("the input {} was not a number!", x);
    }
}

fn is_greater_than_five(x: i32) -> bool {
    let ret = if x > 5 {
        true
    } else {
        false
    };

    println!("{}", ret);
    ret
}

fn count_to(x: i32) {
    let mut counter = 0;

    loop {
        counter = counter + 1;
        println!("{}", counter);

        if counter == x {
            break;
        }
    }
}

fn iter(x: [i32; 5]) {
    let mut index = 0;
    for element in x.iter() {
        println!("the value at index {}: {}", index, element);
        index = index + 1;
    }
}
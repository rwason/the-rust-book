pub fn run() {
    println!("data types!");

    // four scalar types: integers, floating point, booleans, characters

    let a = 5;
    let b: i8 = -127;
    let c = 0b111_0000;
    let d = 2.0;
    let e: bool = true;
    let f: char = 'c';

    println!("{} {} {} {} {} {}", a, b, c, d, e, f);

    // two compound types: tuples and arrays

    let tup_a: (i32, f64, u8) = (500, 6.4, 1);
    println!("tuple contents: {} {} {}", tup_a.0, tup_a.1, tup_a.2);

    let tup_b = (500, 6.4, 1);
    let (x, y, z) = tup_b;
    println!("the value of x, y, z: {} {} {}", x, y, z);

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("the first three elements are: {} {} {}", arr[0], arr[1], arr[2]);

    // can initialize array with the same value for size N as follows
    let arr = [0; 5];
    println!("first and last element: {} {}", arr[0], arr[4]);

    // runtime error if index out of bounds for array
    //println!("{}", arr[5]);


}
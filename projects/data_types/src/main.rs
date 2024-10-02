use std::io; // Import the io module

fn main() {
    println!("Hello, world!");

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{guess}");

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x = {x} and y = {y}");

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("{sum}, {difference}, {product}, {quotient}, {truncated}, {remainder}");

    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("{t}, {f}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{c}, {z}, {heart_eyed_cat}");

    let tup: (i32, f32, u8) = (500, 6.4, 1); // creates a tuple and binds it to the variable `tup`.
    let (tx, ty, tz) = tup; // _destructuring_ using a pattern with let to take tup single tuple into multiple separate variables.
    println!("{tx}, {ty}, {tz}");
    // We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access.
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{five_hundred}, {six_point_four}, {one}");

    let _arr1: [i32; 5] = [1, 2, 3, 4, 5];
    // println!("{arr1[0]}, {arr1[1]}, {arr1[2]}, {arr1[3]}, {arr1[4]}");

    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let _arr2 = [3; 5];
    // println!("{arr2[0]}, {arr2[1]}, {arr2[2]}, {arr2[3]}, {arr2[4]}");

    let arrt = [1, 2, 3, 4, 5];

    println!("Please enter an array index: ");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arrt[index];

    println!("The value of the element at index {index} is: {element}");
}

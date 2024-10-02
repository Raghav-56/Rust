fn main() {
    println!("Hello, world!");

    condition();

    repeat();
}

fn condition() {
    let number1 = 3;
    if number1 < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number2 = 3;
    if number2 != 0 {
        println!("number was something other than zero");
    }

    let number3 = 6;
    if number3 % 4 == 0 {
        println!("number3 is divisible by 4");
    } else if number3 % 3 == 0 {
        println!("number3 is divisible by 3");
    } else if number3 % 2 == 0 {
        println!("number3 is divisible by 2");
    } else {
        println!("number3 is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn repeat() {
    loop {
        println!("again!");
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    for number in 1..4 {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

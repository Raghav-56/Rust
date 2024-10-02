fn main() {
    let mut x = 5;
    println!("x = {x}");
    x = 6;
    println!("x = {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // x = THREE_HOURS_IN_SECONDS;
    println!("x = {THREE_HOURS_IN_SECONDS}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    println!("{spaces}");
    let spaces = spaces.len();
    println!("{spaces}");
}

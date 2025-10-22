fn main() {
    // Mutable variable example
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constant example
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // Shadowing example
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // Shadowing with different types
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {spaces}");

    let mut z = 10;
    println!("The value of z is: {z}");
    // z = "20";
    println!("The value of z is: {z}");
}
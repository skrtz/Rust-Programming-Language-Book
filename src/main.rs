fn main() {
    // This block creates a new scope 
    // Demonstrating expressions and statements
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    // Demonstrating a function that returns a value
    let number = five();
    println!("The value returned by five() is: {number}");

    let incremented = plus_one(number);
    println!("the value returned by plus_one is: {incremented}")
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
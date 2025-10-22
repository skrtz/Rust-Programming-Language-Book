fn main() {
    // This line parses the string "42" into a u32 integer.
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The u32 number is: {}", guess);

    // Integer types show allocated in memory
    let a: u8 = 255; // 8-bit unsigned integer
    let b: i8 = -128; // 8-bit signed integer
    let c: u16 = 65_535; // 16-bit unsigned integer
    let d: i16 = -32_768; // 16-bit signed integer
    let e: u32 = 4_294_967_295; // 32-bit unsigned integer
    let f: i32 = -2_147_483_648; // 32-bit signed integer
    let g: u64 = 18_446_744_073_709_551_615; // 64-bit unsigned integer
    let h: i64 = -9_223_372_036_854_775_808; // 64-bit signed integer
    let i: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455; // 128-bit unsigned integer
    let j: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728; // 128-bit signed integer
    let k: isize = -9_223_372_036_854_775_808; // Pointer-sized signed integer
    let l: usize = 18_446_744_073_709_551_615; // Pointer-sized unsigned integer    
    println!(
        "Integers: a: {}, b: {}, c: {}, d: {}, e: {}, f: {}, g: {}, h: {}, i: {}, j: {}, k: {}, l: {}",
        a, b, c, d, e, f, g, h, i, j, k, l
    );  

    // Floating point types
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("Floating point x: {}, Floating point y: {}", x, y);

    // Numeric operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    println!(
        "Sum: {}, Difference: {}, Product: {}, Quotient: {}, Truncated: {}, Remainder: {}",
        sum, difference, product, quotient, truncated, remainder
    );

    // Boolean types
    let t = true;
    let f: bool = false;

    println!("Boolean t: {}, Boolean f: {}", t, f);

    // Character types
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("Characters: {}, {}, {}", c, z, heart_eyed_cat);
}

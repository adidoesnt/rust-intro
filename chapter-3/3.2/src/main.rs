fn main() {
    // scalar types: single value
    // integers, floating-point numbers, booleans, chars
    // integers: 8 to 128 bits, also arch for 32 or 64 bit systems
    // unsigned: 0 to 2^n - 1
    let _x: u8 = 1;
    let _x: u16 = 1;
    let _x: u32 = 1;
    let _x: u64 = 1;
    let _x: u128 = 1;
    let _x: usize = 1;
    // signed: -2^(n-1) to 2^(n-1) - 1, 2's complement
    let _x: i8 = 1;
    let _x: i16 = 1;
    let _x: i32 = 1;
    let _x: i64 = 1;
    let _x: i128 = 1;
    let _x: isize = 1;
    // integer defaults to i32
    // overflows will cause panic in debug mode
    // in release mode, Rust will perform two's complement wrapping

    // floating-point numbers: f32, f64
    let _x: f32 = 1.0;
    let _x: f64 = 1.0; // default

    // boolean: true, false
    let _t: bool = true; // explicit type annotation

    // char: single quotes, 4 bytes, Unicode Scalar Value
    const EMOJI: char = '😻';
    println!("emoji: {EMOJI}");

    // tuples have fixed length, can contain different types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring
    println!("x: {x}, y: {y}, z: {z}");
    let z = tup.2; // access by index
    println!("z: {z}");

    let mut tup2: (u32, u32, u32) = (1, 2, 3);
    println!("tup2: {}, {}, {}", tup2.0, tup2.1, tup2.2);
    tup2.0 = 4; // mutating
    println!("tup2: {}, {}, {}", tup2.0, tup2.1, tup2.2);

    let _arr: [u32; 3] = [0, 1, 2]; // fixed length, same type
    let _arr2 = [0; 3]; // [0, 0, 0]
    // rust throws runtime error if index out of bounds, memory safety
}
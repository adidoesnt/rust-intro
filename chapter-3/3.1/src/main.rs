fn main() {
    let _x: u32 = 1;
    let mut y: u32 = 2;
    println!("x: {_x}, y: {y}");
    // const z: u32 = x * y; error
    // x = 2; // error
    y = 3;
    // z = 4; // error
    println!("x: {_x}, y: {y}");
}
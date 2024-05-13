fn main() {
    let x: i32 = 5; // statement - perform an action, do not return a value
    let y: i32 = {
        let x: i32 = 1;
        x + 1 // removing the semicolon makes this valid
    }; // expression - evaluate to a resulting value
    let z = not_main(x, y); // expression
    println!("The value of z is {}", z);
}

fn not_main(x: i32, y: i32) -> i32 {
    let z: i32 = x + y;
    return z;
}

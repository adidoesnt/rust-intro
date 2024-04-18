fn main() {
    println!("Hello, world!");
    not_main();
    let y = also_not_main(42);
    println!("The value of y is {}", y);
}

fn not_main() {
    println!("This is not printed from the main function!");
}

fn also_not_main(x: i32) -> i32 {
    // let y = (let z = 4); // This line will not compile (let z = 4 is a statement)
    let y: i32 = { 
        let z: i32 = 4;
        z
    }; // This line will compile (this block is an expression)
    println!(
        "This is also not printed from the main function, but the value of x is {}",
        x
    );
    y // alternatively, you can write "return y;"
}

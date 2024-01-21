fn main() {
    let x: i32 = 7;
    if x < 5 {
        println!("x is less than 5");
    } else if x == 5 {
        println!("x is equal to 5");
    } else {
        println!("x is greater than 5");
    } // multiple conditions checked with else if

    let y: i32 = 3;
    if y != 0 {
        println!("y is not 0");
    } else {
        println!("y is 0");
    }

    let w = true;
    let z = if w { 5 } else { 6 }; // ternary statement
    println!("The value of z is {}", z);

    let mut i: i32 = 0;
    'label1: loop { // loop labels must begin with a single quote
        println!("looping");
        i += 1;
        if i >= 9 {
            break 'label1;
        }
    }
    let mut j = 0;
    let a = 'label2: loop {
        println!("looping again");
        j += 1;
        if j >= 9 {
            break 'label2 j * 2; // possible to return a value from a loop
        }
    };
    println!("The value of a is {}", a);

    let mut k: i32 = 0;
    while k < 10 {
        println!("while loop");
        k += 1;
    }

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let mut l: usize = 0; // must use usize for array index
    while l < 5 {
        println!("The value of arr[{}] is {}", l, arr[l]);
        l += 1;
    }

    for m in 0..5 { // range is exclusive of the last value
        println!("The value of arr[{}] is {}", m, arr[m]);
    }

    for element in arr {
        println!("The value of element is {}", element);
    }
}

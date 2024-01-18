fn main() {
    // let, const and mut
    let _x: u32 = 1;
    let mut y: u32 = 2;
    println!("x: {_x}, y: {y}");
    // const z: u32 = x * y; error
    // x = 2; // error
    y = 3;
    // z = 4; // error
    println!("x: {_x}, y: {y}");

    // shadowing
    let x: u32 = 10;
    let x = x * 2;
    {
        let x = x * 2;
        println!("x: {x}");
    }
    println!("x: {x}");

    // shadowing can be used to change a variable's type
    let _word = "word";
    println!("word: {_word}");
    let _word = _word.len();
    println!("word: {_word}");
    // let mut word2 = "word2";
    // println!("word2: {word2}");
    // word2 = word2.len(); // error
}
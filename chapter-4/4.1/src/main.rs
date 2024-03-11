fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // the String type is mutable
    println!("{}", s); // This will print `hello, world!`
}

use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..101);
    if random_number < 50 {
        println!("The random number is less than 50");
    } else if random_number == 50 {
        println!("The random number is equal to 50");
    } else {
        println!("The random number is greater than 50");
    } 
}

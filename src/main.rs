use std::io;
use rand::Rng;
fn main() {
let secret_number = rand::thread_rng().gen_range(1..100);
loop {
println!("Guess a number ");
let mut input = String::new();
io::stdin()
.read_line(&mut input).
expect("Error To read message");
let inputnumber:i32 = input.trim().parse().expect("Errr");
println!("You Guessed :{}",input);
if inputnumber == secret_number {
    println!("Nice Guess , Game over");
    break;
} else if inputnumber > secret_number {
    println!("Get Down")
} else if inputnumber < secret_number {
    println!("Get Up")
} else {
    println!("Eror")
}
}
}
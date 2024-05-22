use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Hello, this is a guessing game.");
    println!("I have chosen a number between 1 and 100.");
    println!("With each of your answers, I will tell you if your number is higher or lower than the one I have chosen.");
    let secret_number = rand::thread_rng().gen_range(1..100);
    loop{
        println!("Write your number.");
        let mut guessed_number = String::new();
        io::stdin().read_line(&mut guessed_number);
        let guessed_number: u32 = match guessed_number.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Write a number from 1 to 100.");
                continue;
            }
        };
        match guessed_number.cmp(&secret_number)
        {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {println!("you won!"); break;}
        }
    }
}

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("give the number to guess");

    let secretnumber = rand::thread_rng().gen_range(1..=100);

    println!("the secret number :{secretnumber}");

    loop {
        println!("input your guess");

        let mut guess = String::new();
        //expect is a enum
        //readline is used to read the input from the user
        io::stdin().read_line(&mut guess).expect("failed");

        //we will add line rust alows the shadow of the previoius one
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("the number is :{guess}");
        //the ordering type is another enum
        match guess.cmp(&secretnumber) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}

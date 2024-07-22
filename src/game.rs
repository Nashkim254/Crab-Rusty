use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

pub fn run() {
    let mut rng = thread_rng();
    
    

    loop {
        println!("Guess a number between 1 and 10");
        println!("Please input your guess:");
        let i = rng.gen_range(1..10);
        println!(" Your random is:{}", i);
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!(" Your guess is:{}", guess);

        let guess: u32 = guess.trim().parse().unwrap();

        match guess.cmp(&i) {
            Ordering::Less => println!("Less guess"),
            Ordering::Greater => println!("Greater guess"),
            Ordering::Equal => println!("You win"),
        }
    }
}

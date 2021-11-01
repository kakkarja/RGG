use rand::Rng;
use std::io;

pub mod guessing {
    
    pub trait Guess {
        fn new(num: u32) -> Numb {
            Numb { num }
        }

        fn ckeo(&self);
    }
    
    pub struct Numb {
        pub num: u32,
    }

    impl Guess for Numb {
        fn ckeo(&self) {
            if self.num % 2 == 0 {
                println!("\nSecret number is an even number!");
            } else {
                println!("\nSecret number is an odd number!");
            }
        }
    }
}

pub fn contag() -> bool {
    println!("\nDo you still want to play the game? ('y' to play again!) ");

    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");
    if answer.trim().to_lowercase() == "y" {
        true
    } else {
        println!("Game finished!");
        false
    }
}

pub fn sect() -> guessing::Numb {
    guessing::Numb {
        num: rand::thread_rng().gen_range(1..=100),
    }
}
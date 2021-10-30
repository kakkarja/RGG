use guessing_game::guessing::Numb;
use guessing_game::{contag, sect};
use std::{cmp::Ordering, io};

fn run() {
    let mut secret_number = sect();
    secret_number.ckeo();
    println!("You have 10 chances to guess!\n");
    let mut trial = 1;
    while trial <= 10 {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = Numb::new(match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Must be integer!");
                continue;
            }
        });

        if guess.num == 0 {
            println!("Cannot be 0!");
            continue;
        } else if guess.num > 100 {
            println!("Cannot exceed 100!");
            continue;
        }

        println!("You guessed: {} chance: {}", guess.num, trial);

        match guess.num.cmp(&secret_number.num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!(
                    "You win! guess {} == secret number {:#?}",
                    guess.num, secret_number.num
                );

                if contag() {
                    trial = 1;
                    secret_number = sect();
                    secret_number.ckeo();
                    println!("You have 10 chances to guess!\n");
                    continue;
                } else {
                    break;
                }
            }
        }

        trial += 1;
        if trial == 11 {
            println!(
                "You have exceeded the guessing! secret number is {:#?}",
                secret_number.num
            );

            if contag() {
                trial = 1;
                secret_number = sect();
                secret_number.ckeo();
                println!("You have 10 chances to guess!\n");
                continue;
            } else {
                break;
            }
        }
    }
}

pub fn main() {
    println!("Game start!");
    run();
}

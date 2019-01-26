extern crate unnamed;

use unnamed::{
    outcome::Outcome,
    play::Play,
};
use std::io;

fn get_play(nth: &str) -> Play {
    let mut input = String::new();
    loop {
        println!("Enter {} play", nth);
        io::stdin().read_line(&mut input).expect(&format!("Error reading {} play", nth));
        match Play::from_char(input.trim()) {
            Some(play) => break play,
            None => {
                println!("Invalid play char: {}", input);
                input.truncate(0)
            },
        }
    }
}

fn main() {
    let mut n = 1;
    loop {
        println!("Turn {}:", n);

        let first_play = get_play("first");
        println!("{:?}", first_play);
        println!("");

        let second_play = get_play("second");
        println!("{:?}", second_play);
        println!("");

        let outcome = first_play.fight(&second_play);
        println!("Outcome: {:?}", outcome);

        match outcome {
            Outcome::Draw => {
                n += 1;
                println!("")
            },
            _ => break,
        }
    }
}

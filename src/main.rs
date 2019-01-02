use std::io;

mod outcome {
    #[derive(Debug)]
    pub enum Outcome {
        Win,
        Loss,
        Draw,
    }
}

use self::outcome::Outcome;

mod play {
    use super::outcome::Outcome;

    #[derive(Debug)]
    pub enum Play {
        Rock,
        Paper,
        Scissors,
    }

    impl Play {
        pub fn from_char(c: &str) -> Option<Play> {
            return match c {
                "r" => Some(Play::Rock),
                "p" => Some(Play::Paper),
                "s" => Some(Play::Scissors),
                _ => None,
            };
        }

        pub fn fight(&self, other: &Play) -> Outcome {
            match (self, other) {
                (Play::Rock, Play::Scissors) => Outcome::Win,
                (Play::Paper, Play::Rock) => Outcome::Win,
                (Play::Scissors, Play::Paper) => Outcome::Win,
                (Play::Rock, Play::Paper) => Outcome::Loss,
                (Play::Paper, Play::Scissors) => Outcome::Loss,
                (Play::Scissors, Play::Rock) => Outcome::Loss,
                (Play::Rock, Play::Rock) => Outcome::Draw,
                (Play::Paper, Play::Paper) => Outcome::Draw,
                (Play::Scissors, Play::Scissors) => Outcome::Draw,
            }
        }
    }
}

use self::play::Play;

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

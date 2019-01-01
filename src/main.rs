use std::io;

mod outcome {
    #[derive(Debug)]
    pub enum Outcome {
        Win,
        Loss,
        Draw,
    }

    // impl Outcome {
    //     pub fn display(&self) -> &str {
    //         match self {
    //             Outcome::Win => "Win",
    //             Outcome::Loss => "Loss",
    //             Outcome::Draw => "Draw",
    //         }
    //     }
    // }    
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
        // pub fn display(&self) -> &str {
        //     return match self {
        //         Play::Rock => "Rock",
        //         Play::Paper => "Paper",
        //         Play::Scissors => "Scissors",
        //     };
        // }

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

        // let mut first_input = String::new();
        // let first_play = loop {
        //     println!("Enter first play:");
        //     io::stdin().read_line(&mut first_input).expect("Error reading first play");
        //     match Play::from_char(first_input.trim()) {
        //         Some(play) => break play,
        //         None => {
        //             println!("Invalid play char: {}", first_input);
        //             first_input.truncate(0)
        //         },
        //     };
        // };
        let first_play = get_play("first");
        println!("{:?}", first_play);
        println!("");

        let second_play = get_play("second");
        // let second_play = loop {
        //     println!("Enter second play:");
        //     io::stdin().read_line(&mut second_input).expect("Error reading second play");
        //     match Play::from_char(second_input.trim()) {
        //         Some(play) => break play,
        //         None => {
        //             println!("Invalid play char: {}", second_input);
        //             second_input.truncate(0)
        //         }
        //     };
        // };
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

    // match io::stdin().read_line(&mut input) {
    //     Ok(_) => {
    //         println!("{}", Play::from_char(input.trim()).display());
    //     },
    //     Err(error) => println!("error: {}", error),
    // }
}

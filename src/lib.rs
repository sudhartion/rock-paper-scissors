pub mod outcome {
    #[derive(Debug)]
    pub enum Outcome {
        Win,
        Loss,
        Draw,
    }
}

pub mod play {
    use crate::outcome::Outcome;

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


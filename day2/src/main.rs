use std::fs;

// The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors
// second column is our play - X for Rock, Y for Paper, and Z for Scissors

// The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
// plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

#[derive(Debug)]
struct Match<'a> {
    our_move: &'a str,
    outcome: u32,
}

impl<'a> Match<'a> {
    pub fn new(moves: (&'a str, &'a str)) -> Self {
        let oponent_move = match moves.0.trim() {
            "A" => "rock",
            "B" => "paper",
            "C" => "scissors",
            _ => panic!("Unmatched case {:?}", moves.0),
        };
        let our_move = match moves.1.trim() {
            "X" => match oponent_move {
                "rock" => "scissors",
                "paper" => "rock",
                "scissors" => "paper",
                _ => panic!("Unmatched case {:?}", moves.0),
            },
            "Y" => match oponent_move {
                "rock" => "rock",
                "paper" => "paper",
                "scissors" => "scissors",
                _ => panic!("Unmatched case {:?}", moves.0),
            },
            "Z" => match oponent_move {
                "rock" => "paper",
                "paper" => "scissors",
                "scissors" => "rock",
                _ => panic!("Unmatched case {:?}", moves.0),
            },
            _ => panic!("Unmatched case {:?}", moves.1),
        };

        let outcome = match moves.1.trim() {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => panic!("Unmatched case {:?}", moves.1),
        };

        {
            Match { our_move, outcome }
        }
    }
    fn calculate_score(&self) -> u32 {
        match *self {
            Match {
                our_move: "rock",
                outcome: x,
            } => 1 + x,
            Match {
                our_move: "paper",
                outcome: x,
            } => 2 + x,
            Match {
                our_move: "scissors",
                outcome: x,
            } => 3 + x,
            _ => panic!("Unmached case, {:?}", self),
        }
    }
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("File contents should be read");
    let split: Vec<&str> = file_contents.split("\n").collect();

    // dbg!(&split);

    let mut total: u32 = 0;
    for row in split {
        let players_match = Match::new(row.split_at(1));
        let score = players_match.calculate_score();
        total += score;
        // dbg!(score);
    }

    /*
    "Anyway, the second column says how the round needs to end:
    X means you need to lose,
    Y means you need to end the round in a draw,
    and Z means you need to win. Good luck!"
    */

    println!("Total: {:?}", total);
}

use std::fs;

// The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors
// second column is our play - X for Rock, Y for Paper, and Z for Scissors

// The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
// plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

#[derive(Debug)]
struct Match<'a> {
    oponent_move: &'a str,
    our_move: &'a str,
}

impl<'a> Match<'a> {
    pub fn new(moves: (&'a str, &'a str)) -> Self {
        let player_1_move = match moves.0.trim() {
            "A" => "rock",
            "B" => "paper",
            "C" => "scissors",
            _ => panic!("Unmatched case {:?}", moves.0),
        };
        let player_2_move = match moves.1.trim() {
            "X" => "rock",
            "Y" => "paper",
            "Z" => "scissors",
            _ => panic!("Unmatched case {:?}", moves.1),
        };

        {
            Match {
                oponent_move: player_1_move,
                our_move: player_2_move,
            }
        }
    }
    fn calculate_score(&self) -> u32 {
        match *self {
            Match {
                oponent_move: "rock",
                our_move: "rock",
            } => 1 + 3,
            Match {
                oponent_move: "rock",
                our_move: "paper",
            } => 2 + 6,
            Match {
                oponent_move: "rock",
                our_move: "scissors",
            } => 3 + 0,

            Match {
                oponent_move: "paper",
                our_move: "rock",
            } => 1 + 0,
            Match {
                oponent_move: "paper",
                our_move: "paper",
            } => 2 + 3,
            Match {
                oponent_move: "paper",
                our_move: "scissors",
            } => 3 + 6,

            Match {
                oponent_move: "scissors",
                our_move: "rock",
            } => 1 + 6,
            Match {
                oponent_move: "scissors",
                our_move: "paper",
            } => 2 + 0,
            Match {
                oponent_move: "scissors",
                our_move: "scissors",
            } => 3 + 3,

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

    println!("Total: {:?}", total);
}

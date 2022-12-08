use std::fs;

// The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors
// second column is our play - X for Rock, Y for Paper, and Z for Scissors

// The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
// plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

#[derive(Debug)]

enum Move {
    Rock,
    Paper,
    Scissors,
}
#[derive(Debug)]
struct Match {
    our_move: Move,
    outcome: u32,
}

impl Match {
    pub fn new(moves: (&str, &str)) -> Self {
        let oponent_move = match moves.0.trim() {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Unmatched case {:?}", moves.0),
        };
        let our_move = match moves.1.trim() {
            "X" => match oponent_move {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
            "Y" => match oponent_move {
                Move::Rock => Move::Rock,
                Move::Paper => Move::Paper,
                Move::Scissors => Move::Scissors,
            },
            "Z" => match oponent_move {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
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
                our_move: Move::Rock,
                outcome: x,
            } => 1 + x,
            Match {
                our_move: Move::Paper,
                outcome: x,
            } => 2 + x,
            Match {
                our_move: Move::Scissors,
                outcome: x,
            } => 3 + x,
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

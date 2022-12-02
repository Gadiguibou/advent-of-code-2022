use std::{error::Error, str::FromStr};

#[derive(Clone, Copy)]
enum OpponentChoice {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for OpponentChoice {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(OpponentChoice::Rock),
            "B" => Ok(OpponentChoice::Paper),
            "C" => Ok(OpponentChoice::Scissors),
            _ => Err("Invalid choice".into()),
        }
    }
}

enum MyChoice {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for MyChoice {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(MyChoice::Rock),
            "Y" => Ok(MyChoice::Paper),
            "Z" => Ok(MyChoice::Scissors),
            _ => Err("Invalid choice".into()),
        }
    }
}

fn match_points(mine: MyChoice, opponent: OpponentChoice) -> u64 {
    let choice_points = match mine {
        MyChoice::Rock => 1,
        MyChoice::Paper => 2,
        MyChoice::Scissors => 3,
    };

    enum Result {
        Win,
        Draw,
        Lose,
    }

    let result = match (mine, opponent) {
        (MyChoice::Rock, OpponentChoice::Rock) => Result::Draw,
        (MyChoice::Rock, OpponentChoice::Paper) => Result::Lose,
        (MyChoice::Rock, OpponentChoice::Scissors) => Result::Win,
        (MyChoice::Paper, OpponentChoice::Rock) => Result::Win,
        (MyChoice::Paper, OpponentChoice::Paper) => Result::Draw,
        (MyChoice::Paper, OpponentChoice::Scissors) => Result::Lose,
        (MyChoice::Scissors, OpponentChoice::Rock) => Result::Lose,
        (MyChoice::Scissors, OpponentChoice::Paper) => Result::Win,
        (MyChoice::Scissors, OpponentChoice::Scissors) => Result::Draw,
    };

    let result_points = match result {
        Result::Win => 6,
        Result::Draw => 3,
        Result::Lose => 0,
    };

    choice_points + result_points
}

enum DesiredOutcome {
    Win,
    Draw,
    Lose,
}

impl FromStr for DesiredOutcome {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(DesiredOutcome::Lose),
            "Y" => Ok(DesiredOutcome::Draw),
            "Z" => Ok(DesiredOutcome::Win),
            _ => Err("Invalid choice".into()),
        }
    }
}

fn choice_to_make(opponent_choice: OpponentChoice, desired_outcome: DesiredOutcome) -> MyChoice {
    match (opponent_choice, desired_outcome) {
        (OpponentChoice::Rock, DesiredOutcome::Lose) => MyChoice::Scissors,
        (OpponentChoice::Rock, DesiredOutcome::Draw) => MyChoice::Rock,
        (OpponentChoice::Rock, DesiredOutcome::Win) => MyChoice::Paper,
        (OpponentChoice::Paper, DesiredOutcome::Lose) => MyChoice::Rock,
        (OpponentChoice::Paper, DesiredOutcome::Draw) => MyChoice::Paper,
        (OpponentChoice::Paper, DesiredOutcome::Win) => MyChoice::Scissors,
        (OpponentChoice::Scissors, DesiredOutcome::Lose) => MyChoice::Paper,
        (OpponentChoice::Scissors, DesiredOutcome::Draw) => MyChoice::Scissors,
        (OpponentChoice::Scissors, DesiredOutcome::Win) => MyChoice::Rock,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_string = std::fs::read_to_string("input.txt")?;

    let strategy_guide: Vec<(&str, &str)> = input_string
        .lines()
        .map(|l| {
            l.split_once(' ')
                .unwrap_or_else(|| panic!("No whitespace in line: {l}"))
        })
        .collect();

    let part_1: u64 = strategy_guide
        .iter()
        .map(
            |(opponent_choice, my_choice)| -> (OpponentChoice, MyChoice) {
                (
                    opponent_choice
                        .parse()
                        .unwrap_or_else(|e| panic!("Invalid opponent choice: {e}")),
                    my_choice
                        .parse()
                        .unwrap_or_else(|e| panic!("Invalid choice: {e}")),
                )
            },
        )
        .map(|(opponent_choice, my_choice)| match_points(my_choice, opponent_choice))
        .sum();

    println!("Part 1: {part_1}");

    let part_2: u64 = strategy_guide
        .iter()
        .map(
            |(opponent_choice, desired_outcome)| -> (OpponentChoice, DesiredOutcome) {
                (
                    opponent_choice
                        .parse()
                        .unwrap_or_else(|e| panic!("Invalid opponent choice: {e}")),
                    desired_outcome
                        .parse()
                        .unwrap_or_else(|e| panic!("Invalid desired outcome: {e}")),
                )
            },
        )
        .map(
            |(opponent_choice, desired_outcome)| -> (OpponentChoice, MyChoice) {
                (
                    opponent_choice,
                    choice_to_make(opponent_choice, desired_outcome),
                )
            },
        )
        .map(|(opponent_choice, my_choice)| match_points(my_choice, opponent_choice))
        .sum();

    println!("Part 2: {part_2}");

    Ok(())
}

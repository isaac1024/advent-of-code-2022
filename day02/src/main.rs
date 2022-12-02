use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::iter::Iterator;
use std::str::FromStr;
use clap::{arg, command};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref WINNER_MAP: HashMap<GameOptions, GameOptions> = vec![
        (GameOptions::Rock, GameOptions::Scissors),
        (GameOptions::Paper, GameOptions::Rock),
        (GameOptions::Scissors, GameOptions::Paper),
    ].into_iter().collect();

    static ref LOSER_MAP: HashMap<GameOptions, GameOptions> = vec![
        (GameOptions::Rock, GameOptions::Paper),
        (GameOptions::Paper, GameOptions::Scissors),
        (GameOptions::Scissors, GameOptions::Rock),
    ].into_iter().collect();
}

#[derive(Eq, PartialEq, Hash)]
enum GameOptions {
    Rock,
    Paper,
    Scissors
}

impl GameOptions {
    fn self_score(&self) -> i32 {
        match self {
            GameOptions::Rock => 1,
            GameOptions::Paper => 2,
            GameOptions::Scissors => 3,
        }
    }

    fn score(&self, other: &GameOptions) -> i32 {
        let score = self.self_score();

        if self.win(other) {
            return score + 6;
        }

        if self.draw(other) {
            return score + 3;
        }

        score
    }

    fn win(&self, other: &GameOptions) -> bool {
        return WINNER_MAP[self] == *other
    }

    fn draw(&self, other: &GameOptions) -> bool {
        return self == other
    }
}

impl FromStr for GameOptions {
    type Err = ();

    fn from_str(str_game_option: &str) -> Result<Self, Self::Err> {
        match str_game_option {
            "A" | "X" => Ok(GameOptions::Rock),
            "B" | "Y" => Ok(GameOptions::Paper),
            "C" | "Z" => Ok(GameOptions::Scissors),
            _ => Err(()),
        }
    }
}

struct GameResult {
    opponent: GameOptions,
    your: GameOptions
}

impl GameResult {
    fn score(&self) -> i32 {
        self.your.score(&self.opponent)
    }

    fn score2(&self) -> i32 {
        let score: i32;
        let game_option: &GameOptions;
        match self.your {
            GameOptions::Rock => {
                score = 0;
                game_option = &WINNER_MAP[&self.opponent];
            },
            GameOptions::Paper => {
                score = 3;
                game_option = &self.opponent;
            },
            GameOptions::Scissors => {
                score = 6;
                game_option = &LOSER_MAP[&self.opponent];
            },
        };

        score + game_option.self_score()
    }
}

impl FromStr for GameResult {
    type Err = ();

    fn from_str(game_result: &str) -> Result<Self, Self::Err> {
        let result: Vec<&str> = game_result.split_whitespace().collect();

        if result.len() != 2 {
            return Err(());
        }

        let opponent = GameOptions::from_str(result[0])?;
        let your = GameOptions::from_str(result[1])?;

        return Ok(GameResult{opponent, your})
    }
}

fn main() {
    let matches = command!()
        .args([
            arg!(-f --file <FILE> "Input file path"),
            arg!(-e --expected <EXPECTED> "Expected value"),
            arg!(-o --one "Part one"),
            arg!(-t --two "Part two")
        ])
        .get_matches();

    let path = matches.get_one::<String>("file").expect("File argument is required");
    let expected = matches.get_one::<String>("expected").and_then(|expected| {
        expected.parse::<i32>().ok()
    });
    let part_one = matches.get_one::<bool>("one").unwrap().to_owned();
    let part_two = matches.get_one::<bool>("two").unwrap().to_owned();

    if part_one {
        println!("Part one");
        run_part_one(path, expected);
    }

    if part_two {
        println!("Part two");
        run_part_two(path, expected);
    }


    println!("Finish");
}

fn run_part_one(path: &String, expected: Option<i32>) {
    match read_file(path) {
        Ok(game_results) => print_part_one_result(game_results, expected),
        Err(error) => println!("{}", error)
    }
}

fn read_file(path: &String) -> Result<Vec<GameResult>, Error> {
    fs::read_to_string(path).and_then(|input_content| {
        let game_results: Vec<GameResult> = input_content.lines()
            .map(|line| GameResult::from_str(line).unwrap())
            .collect();

        Ok(game_results)
    })
}

fn print_part_one_result(game_results: Vec<GameResult>, expected: Option<i32>) {
    let score = calculate_score(game_results);
    match expected {
        Some(expected_value) => {
            if expected_value == score {
                println!("Result is ok");
            } else {
                println!("Result is not ok");
            }
        },
        None => println!("{}", score)
    }
}

fn calculate_score(game_results: Vec<GameResult>) -> i32 {
    game_results.iter()
        .map(|game_result| game_result.score())
        .sum()
}

fn run_part_two(path: &String, expected: Option<i32>) {
    match read_file(path) {
        Ok(game_results) => print_part_two_result(game_results, expected),
        Err(error) => println!("{}", error)
    }
}

fn print_part_two_result(game_results: Vec<GameResult>, expected: Option<i32>) {
    let score = calculate_score2(game_results);
    match expected {
        Some(expected_value) => {
            if expected_value == score {
                println!("Result is ok");
            } else {
                println!("Result is not ok");
            }
        },
        None => println!("{}", score)
    }
}

fn calculate_score2(game_results: Vec<GameResult>) -> i32 {
    game_results.iter()
        .map(|game_result| game_result.score2())
        .sum()
}

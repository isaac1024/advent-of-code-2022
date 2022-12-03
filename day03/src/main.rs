use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::iter::Iterator;
use clap::{arg, command};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CHAR_TO_INT: HashMap<char, i32> = vec![
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
        ('A', 27),
        ('B', 28),
        ('C', 29),
        ('D', 30),
        ('E', 31),
        ('F', 32),
        ('G', 33),
        ('H', 34),
        ('I', 35),
        ('J', 36),
        ('K', 37),
        ('L', 38),
        ('M', 39),
        ('N', 40),
        ('O', 41),
        ('P', 42),
        ('Q', 43),
        ('R', 44),
        ('S', 45),
        ('T', 46),
        ('U', 47),
        ('V', 48),
        ('W', 49),
        ('X', 50),
        ('Y', 51),
        ('Z', 52),
    ].into_iter().collect();
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
    // let part_two = matches.get_one::<bool>("two").unwrap().to_owned();

    if part_one {
        println!("Part one");
        run_part_one(path, expected);
    }

    // if part_two {
    //     println!("Part two");
    //     run_part_two(path, expected);
    // }


    println!("Finish");
}

fn run_part_one(path: &String, expected: Option<i32>) {
    match read_file(path) {
        Ok(rucksacks) => print_part_one_result(rucksacks, expected),
        Err(error) => println!("{}", error)
    }
}

fn read_file(path: &String) -> Result<Vec<String>, Error> {
    fs::read_to_string(path).and_then(|input_content| {
        Ok(input_content.lines().map(|line| format!("{}", line)).collect())
    })
}

fn print_part_one_result(rucksacks: Vec<String>, expected: Option<i32>) {
    let score = items_checker(rucksacks);
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

fn items_checker(rucksacks: Vec<String>) -> i32 {
    rucksacks.iter()
        .map(|rucksack| {
            let (item1, item2) = rucksack.split_at(rucksack.len() / 2);
            let mut value = 0;
            let mut characters: Vec<char> = vec![];
            for character in item1.chars() {
                if item2.contains(character) && !characters.contains(&character) {
                    value += CHAR_TO_INT[&character];
                    characters.push(character)
                }
            }

            value
        })
        .sum()
}

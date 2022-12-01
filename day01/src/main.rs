use std::fs;
use std::io::Error;
use clap::{arg, command};

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
        Ok(carried_elves) => print_part_one_result(carried_elves, expected),
        Err(error) => println!("{}", error)
    }
}

fn print_part_one_result(carried_elves: Vec<Vec<i32>>, expected: Option<i32>) {
    match get_max_carried_calories(carried_elves) {
        Some(max_carried_calories) => {
            match expected {
                Some(expected_value) => {
                    if expected_value == max_carried_calories {
                        println!("Result is ok");
                    } else {
                        println!("Result is not ok");
                    }
                },
                None => println!("{}", max_carried_calories)
            }
        },
        None => println!("Error max_carried_calories")
    }
}

fn get_max_carried_calories(carried_elves: Vec<Vec<i32>>) -> Option<i32> {
    carried_elves.iter()
        .map(|carried_elf| carried_elf.iter().sum())
        .reduce(i32::max)
}

fn run_part_two(path: &String, expected: Option<i32>) {
    match read_file(path) {
        Ok(carried_elves) => print_part_two_result(carried_elves, expected),
        Err(error) => println!("{}", error)
    }
}

fn print_part_two_result(carried_elves: Vec<Vec<i32>>, expected: Option<i32>) {
    let three_max_carried_calories = get_three_max_carried_calories(carried_elves);

    match expected {
        Some(expected_value) => {
            if expected_value == three_max_carried_calories {
                println!("Result is ok");
            } else {
                println!("Result is not ok");
            }
        },
        None => println!("{}", three_max_carried_calories)
    }
}

fn get_three_max_carried_calories(carried_elves: Vec<Vec<i32>>) -> i32 {
    let mut sum_carried_elves= carried_elves.iter()
        .map(|carried_elf| carried_elf.iter().sum())
        .collect::<Vec<i32>>();
    sum_carried_elves.sort();
    sum_carried_elves.reverse();
    sum_carried_elves.into_iter().take(3).sum()
}

fn read_file(path: &String) -> Result<Vec<Vec<i32>>, Error> {
    fs::read_to_string(path).and_then(|input_content| {
        let mut carried_elves: Vec<Vec<i32>> = vec![];
        let mut carried_elf: Vec<i32> = vec![];
        for input_content_line in input_content.lines() {
            if input_content_line.len() == 0 {
                carried_elves.push(carried_elf.clone());
                carried_elf.clear();
            } else {
                if let Ok(calories) = input_content_line.parse::<i32>() {
                    carried_elf.push(calories);
                }
            }
        }
        carried_elves.push(carried_elf);

        Ok(carried_elves)
    })
}

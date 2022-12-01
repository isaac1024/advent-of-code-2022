use std::fs;
use clap::{arg, command};

fn main() {
    let matches = command!()
        .args([
            arg!(-f --file <FILE> "Input file path"),
            arg!(-e --expected <EXPECTED> "Expected value")
        ])
        .get_matches();

    let path = matches.get_one::<String>("file").expect("File argument is required");
    let expected = matches.get_one::<String>("expected").and_then(|expected| {
        expected.parse::<i32>().ok()
    });

    run(path, expected);

    println!("Finish");
}


fn run(path: &String, expected: Option<i32>) {
    let carried_elves = fs::read_to_string(path).and_then(|input_content| {
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
    });

    match carried_elves {
        Ok(carried_elves) => print_result(carried_elves, expected),
        Err(error) => println!("{}", error)
    }
}

fn print_result(carried_elves: Vec<Vec<i32>>, expected: Option<i32>) {
    let max_carried_calories = get_max_carried_calories(carried_elves);

    match max_carried_calories {
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

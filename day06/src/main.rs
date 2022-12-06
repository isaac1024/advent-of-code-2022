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
        Ok(x) => print_part_one_result(x, expected),
        Err(error) => println!("{}", error)
    }
}

fn read_file(path: &String) -> Result<String, Error> {
    fs::read_to_string(path)
}

fn print_part_one_result(pair_elves_range_section_ids: String, expected: Option<i32>) {
    let items_checker = items_checker(pair_elves_range_section_ids);
    match expected {
        Some(expected_value) => {
            if expected_value == items_checker {
                println!("Result is ok");
            } else {
                println!("Result is not ok");
            }
        },
        None => println!("{}", items_checker)
    }
}

fn items_checker(items_checker: String) -> i32 {
    for x in 0..items_checker.len() - 4 {
        let sub_datastream = &items_checker[x..x+4].chars().collect::<Vec<char>>();
        let mut aux = sub_datastream.clone();
        let mut ok = true;
        while aux.len() > 1 {
            let char = aux.remove(0);
            if aux.contains(&char) {
                ok = false;
                break
            }
        }
        if ok == true {
            return (x+4) as i32;
        }
    }

    0
}

fn run_part_two(path: &String, expected: Option<i32>) {
    match read_file(path) {
        Ok(x) => print_part_two_result(x, expected),
        Err(error) => println!("{}", error)
    }
}

fn print_part_two_result(pair_elves_range_section_ids: String, expected: Option<i32>) {
    let items_checker = items_checker2(pair_elves_range_section_ids);
    match expected {
        Some(expected_value) => {
            if expected_value == items_checker {
                println!("Result is ok");
            } else {
                println!("Result is not ok");
            }
        },
        None => println!("{}", items_checker)
    }
}

fn items_checker2(items_checker: String) -> i32 {
    for x in 0..items_checker.len() - 14 {
        let sub_datastream = &items_checker[x..x+14].chars().collect::<Vec<char>>();
        let mut aux = sub_datastream.clone();
        let mut ok = true;
        while aux.len() > 1 {
            let char = aux.remove(0);
            if aux.contains(&char) {
                ok = false;
                break
            }
        }
        if ok == true {
            return (x+14) as i32;
        }
    }

    0
}

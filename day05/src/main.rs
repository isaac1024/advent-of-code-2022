use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::iter::Iterator;
use clap::{arg, command};
use regex::Regex;

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
    let expected = matches.get_one::<String>("expected");
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

fn run_part_one(path: &String, expected: Option<&String>) {
    match read_file(path) {
        Ok(x) => print_part_one_result(x, expected),
        Err(error) => println!("{}", error)
    }
}

fn read_file(path: &String) -> Result<(HashMap<i32, Vec<char>>, Vec<(i32, i32, i32)>), Error> {
    fs::read_to_string(path).and_then(|input_content| {
        let mut moves: Vec<(i32, i32, i32)> = Vec::new();
        let mut char_map: HashMap<i32, Vec<char>> = HashMap::new();
        let mut set_char_map = true;
        let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        input_content.lines().for_each(|line| {
            if line.len() < 1 {
                return;
            }
            if line.as_bytes()[1] == b'1' {
                set_char_map = false;
            }

            if set_char_map {
                for i in 0..(line.len() + 1) / 4 {
                    let index = (i + 1) as i32;
                    if char_map.len() < i+1 {
                        char_map.insert(index, Vec::new());
                    }

                    let character = line.as_bytes()[i*4+1] as char;
                    if character.is_alphabetic() {
                        char_map.get_mut(&index).unwrap().push(character);
                    }
                }
            } else {
                for cap in re.captures_iter(line) {
                    moves.push((cap[1].parse::<i32>().unwrap(), cap[2].parse::<i32>().unwrap(), cap[3].parse::<i32>().unwrap()));
                }
            }
        });

        Ok((char_map, moves))
    })
}

fn print_part_one_result(pair_elves_range_section_ids: (HashMap<i32, Vec<char>>, Vec<(i32, i32, i32)>), expected: Option<&String>) {
    let items_checker = items_checker(pair_elves_range_section_ids);
    match expected {
        Some(expected_value) => {
            if expected_value == &items_checker {
                println!("Result is ok");
            } else {
                println!("Result is not ok");
            }
        },
        None => println!("{}", items_checker)
    }
}

fn items_checker(items_checker: (HashMap<i32, Vec<char>>, Vec<(i32, i32, i32)>)) -> String {
    let mut char_map = items_checker.0;
    let moves = items_checker.1;

    for x in moves {
        for _i in 0..x.0 {
            let chars = char_map.get_mut(&x.1).unwrap().remove(0);
            let n = char_map.get_mut(&x.2).unwrap();
            n.reverse();
            n.push(chars);
            n.reverse();
        }
    }

    let mut result = String::new();

    for x in 1..char_map.len()+1 {
        let character = char_map.get(&(x as i32)).unwrap()[0];
        result.push(character);
    }

    result
}

fn run_part_two(path: &String, expected: Option<&String>) {
    match read_file(path) {
        Ok(x) => print_part_two_result(x, expected),
        Err(error) => println!("{}", error)
    }
}

fn print_part_two_result(pair_elves_range_section_ids: (HashMap<i32, Vec<char>>, Vec<(i32, i32, i32)>), expected: Option<&String>) {
    let items_checker = items_checker2(pair_elves_range_section_ids);
    match expected {
        Some(expected_value) => {
            if expected_value == &items_checker {
                println!("Result is ok");
            } else {
                println!("Result is not ok");
            }
        },
        None => println!("{}", items_checker)
    }
}

fn items_checker2(items_checker: (HashMap<i32, Vec<char>>, Vec<(i32, i32, i32)>)) -> String {
    let mut char_map = items_checker.0;
    let moves = items_checker.1;

    for x in moves {
        let mut j: Vec<char> = Vec::new();
        for _i in 0..x.0 {
            let chars = char_map.get_mut(&x.1).unwrap().remove(0);
            j.push(chars);
        }

        let n = char_map.get_mut(&x.2).unwrap();
        n.reverse();
        j.reverse();
        j.iter().for_each(|i| {
            n.push(*i);
        });
        n.reverse();
    }

    let mut result = String::new();

    for x in 1..char_map.len()+1 {
        let character = char_map.get(&(x as i32)).unwrap()[0];
        result.push(character);
    }

    result
}

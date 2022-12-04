use std::fs;
use std::io::Error;
use std::iter::Iterator;
use clap::{arg, command};

struct PairElvesRangeSectionIds {
    elf_one: (i32, i32),
    elf_two: (i32, i32),
}

impl PairElvesRangeSectionIds {
    fn elf_one_contains_elf_two(&self) -> bool {
        self.elf_one.0 <= self.elf_two.0 && self.elf_one.1 >= self.elf_two.1
    }

    fn elf_two_contains_elf_one(&self) -> bool {
        self.elf_two.0 <= self.elf_one.0 && self.elf_two.1 >= self.elf_one.1
    }

    fn check_overlaps(&self) ->bool {
        if self.elf_one.0 < self.elf_two.0 {
            return self.elf_one.1 >= self.elf_two.0
        }

        if self.elf_one.0 > self.elf_two.0 {
            return self.elf_two.1 >= self.elf_one.0;
        }

        true
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
        Ok(pair_elves_range_section_ids) => print_part_one_result(pair_elves_range_section_ids, expected),
        Err(error) => println!("{}", error)
    }
}

fn read_file(path: &String) -> Result<Vec<PairElvesRangeSectionIds>, Error> {
    fs::read_to_string(path).and_then(|input_content| {
        Ok(input_content.lines().map(|line| {
            let elves_range: Vec<&str> = line.split(",").collect();
            let first_elf_range: Vec<i32> = elves_range[0].split("-").map(|x| x.parse::<i32>().unwrap()).collect();
            let second_elf_range: Vec<i32> = elves_range[1].split("-").map(|x| x.parse::<i32>().unwrap()).collect();

            PairElvesRangeSectionIds{
                elf_one: (first_elf_range[0], first_elf_range[1]),
                elf_two: (second_elf_range[0], second_elf_range[1])
            }

        }).collect())
    })
}

fn print_part_one_result(pair_elves_range_section_ids: Vec<PairElvesRangeSectionIds>, expected: Option<i32>) {
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

fn items_checker(items_checker: Vec<PairElvesRangeSectionIds>) -> i32 {
    let mut counter = 0;
    for pair_elves in items_checker {
        if pair_elves.elf_two_contains_elf_one() || pair_elves.elf_one_contains_elf_two() {
            counter += 1;
        }
    }

    counter
}

fn run_part_two(path: &String, expected: Option<i32>) {
    match read_file(path) {
        Ok(pair_elves_range_section_ids) => print_part_two_result(pair_elves_range_section_ids, expected),
        Err(error) => println!("{}", error)
    }
}

fn print_part_two_result(pair_elves_range_section_ids: Vec<PairElvesRangeSectionIds>, expected: Option<i32>) {
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

fn items_checker2(items_checker: Vec<PairElvesRangeSectionIds>) -> i32 {
    let mut counter = 0;
    for pair_elves in items_checker {
        if pair_elves.check_overlaps() {
            counter += 1;
        }
    }

    counter
}

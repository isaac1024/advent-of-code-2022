use std::fs;
use clap::{arg, command};

struct Tree {
    is_visible: bool,
    size: i32
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
    let x = read_file(path);
    print_part_one_result(x, expected);
}

fn read_file(path: &String) -> Vec<Vec<Tree>> {
    fs::read_to_string(path).and_then(|content| {
        Ok(content.lines().map(|line| {
            line.chars().map(|c| {
                Tree{is_visible: false, size: c.to_string().parse().unwrap()}
            }).collect()
        }).collect())
    }).unwrap()
}

fn print_part_one_result(x: Vec<Vec<Tree>>, expected: Option<i32>) {
    let items_checker = items_checker(x);
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

fn items_checker(mut ic: Vec<Vec<Tree>>) -> i32 {
    for i in 0..ic.len() {
        let mut size = -1;
        for j in 0..ic[i].len() {
            if ic[i][j].size > size {
                ic[i][j].is_visible = true;
                size = ic[i][j].size;
            }
        }

        let mut size = -1;
        for j in (0..ic[i].len()).rev() {
            if ic[i][j].size > size {
                ic[i][j].is_visible = true;
                size = ic[i][j].size;
            }
        }
    }

    for i in 0..ic[0].len() {
        let mut size = -1;
        for j in 0..ic.len() {
            if ic[j][i].size > size {
                ic[j][i].is_visible = true;
                size = ic[j][i].size;
            }
        }

        let mut size = -1;
        for j in (0..ic.len()).rev() {
            if ic[j][i].size > size {
                ic[j][i].is_visible = true;
                size = ic[j][i].size;
            }
        }
    }

    let mut trees = 0;
    for x in ic {
        for y in x {
            if y.is_visible {
                trees += 1;
            }
        }
    }

    trees
}

fn run_part_two(path: &String, expected: Option<i32>) {
    let x = read_file(path);
    print_part_two_result(x, expected);
}

fn print_part_two_result(x: Vec<Vec<Tree>>, expected: Option<i32>) {
    let items_checker = items_checker2(x);
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

fn items_checker2(ic: Vec<Vec<Tree>>) -> i32 {
    let mut highest = 0;
    for i in 0..ic.len() {
        for j in 0..ic[i].len() {
            if i == 0 || j == 0 || i == ic.len() - 1 || j == ic[i].len() - 1 {
                continue;
            }

            let mut up = 1;
            let mut down = 1;
            let mut left = 1;
            let mut right = 1;

            //up
            let mut  x = i.clone();
            x -= 1;
            while x > 0 && ic[x][j].size < ic[i][j].size {
                up += 1;
                x -= 1;
            }

            //down
            let mut  x = i.clone();
            x += 1;
            while x < ic.len() - 1 && ic[x][j].size < ic[i][j].size {
                down += 1;
                x += 1;
            }

            //left
            let mut  x = j.clone();
            x -= 1;
            while x > 0 && ic[i][x].size < ic[i][j].size {
                left += 1;
                x -= 1;
            }

            //right
            let mut  x = j.clone();
            x += 1;
            while x < ic[i].len() - 1 && ic[i][x].size < ic[i][j].size {
                right += 1;
                x += 1;
            }

            let total = up*down*left*right;
            if highest < total {
                highest = total;
            }
        }
    }

    highest
}

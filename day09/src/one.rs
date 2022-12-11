use std::fs;

#[derive(Debug)]
pub enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl Move {
    pub fn update_h_position(&self, h: (i32, i32)) -> (i32, i32) {
        match self {
            Move::Up(y) => (h.0, h.1+y),
            Move::Down(y) => (h.0, h.1-y),
            Move::Left(x) => (h.0-x, h.1),
            Move::Right(x) => (h.0+x, h.1),
        }
    }
}

pub fn run(path: &String, expected: Option<i32>) {
    let parsed_file = read_file(path);
    print_result(parsed_file, expected);
}

fn print_result(x: Vec<Move>, expected: Option<i32>) {
    let result = get_result(x);
    match expected {
        Some(expected_value) => {
            if expected_value == result {
                println!("Result is ok");
            } else {
                println!("Result is not ok");
            }
        },
        None => println!("{}", result)
    }
}

fn get_result(data: Vec<Move>) -> i32 {
    let mut h = (0, 0);
    let mut t = (0, 0);
    let mut t_positions = Vec::new();
    t_positions.push(t);

    data.iter().for_each(|movement| {
        h = movement.update_h_position(h);
        let update_t_position = update_t_position(t, h);
        t = update_t_position.last().unwrap().clone();
        update_t_position.iter().for_each(|movement| {
            if t_positions.contains(movement) {
                return;
            }
            t_positions.push(movement.clone());
        });
    });

    t_positions.iter().count() as i32
}

fn update_t_position(t: (i32, i32), h: (i32, i32)) -> Vec<(i32, i32)> {
    let mut positions = Vec::new();
    let x_diff = h.0 - t.0;
    let y_diff = h.1 - t.1;

    if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
        positions.push(t);
        return positions;
    }

    if x_diff.abs() <= 1 {
        let range = if h.1 > t.1 {
            (t.1..h.1).collect::<Vec<i32>>()
        } else {
            (h.1+1..t.1+1).rev().collect::<Vec<i32>>()
        };
        for y in range {
            positions.push((h.0, y))
        }
    } else {
        let range = if h.0 > t.0 {
            (t.0..h.0).collect::<Vec<i32>>()
        } else {
            (h.0+1..t.0+1).rev().collect::<Vec<i32>>()
        };
        for x in range {
            positions.push((x, h.1))
        }
    }

    positions.remove(0);
    return positions;
}

fn read_file(path: &String) -> Vec<Move> {
    fs::read_to_string(path).and_then(|content| {
        content.lines().map(|line| {
            let split_line: Vec<&str> = line.split_whitespace().collect();
            let value: i32 = split_line[1].parse().unwrap();
            let movement = match split_line[0] {
                "U" => Move::Up(value),
                "D" => Move::Down(value),
                "L" => Move::Left(value),
                "R" => Move::Right(value),
                _ => panic!("Invalid argument.")
            };

            Ok(movement)
        }).collect()
    }).unwrap()
}

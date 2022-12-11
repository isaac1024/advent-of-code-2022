use std::fs;

#[derive(Debug)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    pub fn update_h_position(&self, h: (i32, i32)) -> (i32, i32) {
        match self {
            Move::Up => (h.0, h.1+1),
            Move::Down => (h.0, h.1-1),
            Move::Left => (h.0-1, h.1),
            Move::Right => (h.0+1, h.1),
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
    let mut t1 = (0, 0);
    let mut t2 = (0, 0);
    let mut t3 = (0, 0);
    let mut t4 = (0, 0);
    let mut t5 = (0, 0);
    let mut t6 = (0, 0);
    let mut t7 = (0, 0);
    let mut t8 = (0, 0);
    let mut t9 = (0, 0);
    let mut positions = Vec::new();
    positions.push(t9);

    data.iter().for_each(|movement| {
        h = movement.update_h_position(h);
        t1 = update_t_position(t1, h);
        t2 = update_t_position(t2, t1);
        t3 = update_t_position(t3, t2);
        t4 = update_t_position(t4, t3);
        t5 = update_t_position(t5, t4);
        t6 = update_t_position(t6, t5);
        t7 = update_t_position(t7, t6);
        t8 = update_t_position(t8, t7);
        t9 = update_t_position(t9, t8);
        if positions.contains(&t9) {
            return;
        }
        positions.push(t9);
    });

    positions.iter().count() as i32
}

fn update_t_position(t: (i32, i32), h: (i32, i32)) -> (i32, i32) {
    let x_diff = h.0 - t.0;
    let y_diff = h.1 - t.1;

    if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
        return t;
    }

    let x = if x_diff == 0 { 0 } else { x_diff/x_diff.abs() };
    let y = if y_diff == 0 { 0 } else { y_diff/y_diff.abs() };

    (t.0+x, t.1+y)
}

fn read_file(path: &String) -> Vec<Move> {
    let content = fs::read_to_string(path).unwrap();
    let mut movements = Vec::new();
    content.lines().for_each(|line| {
        let split_line: Vec<&str> = line.split_whitespace().collect();
        let value: i32 = split_line[1].parse().unwrap();
        for _i in 0..value {
            let movement = match split_line[0] {
                "U" => Move::Up,
                "D" => Move::Down,
                "L" => Move::Left,
                "R" => Move::Right,
                _ => panic!("Invalid argument.")
            };

            movements.push(movement);
        }
    });

    movements
}

use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use clap::{arg, command};

#[derive(Clone, Debug)]
struct Filesystem {
    name: String,
    size: i32,
    childrens: Vec<Rc<RefCell<Filesystem>>>,
    parent: Option<Rc<RefCell<Filesystem>>>,
}

impl Filesystem {
    fn new(name: String) -> Filesystem {
        Filesystem { name, size: 0, childrens: Vec::new(), parent: None }
    }

    fn add_size(&mut self, size: i32) {
        self.size += size;
        if self.parent.is_some() {
            let x = self.parent.clone();
            x.unwrap().borrow_mut().add_size(size);
        }
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
    let x = read_file(path);
    print_part_one_result(x, expected);
}

fn read_file(path: &String) -> Rc<RefCell<Filesystem>> {
    let root = Rc::new(RefCell::new(Filesystem::new("/".to_string())));
    let mut filesystem = Rc::clone(&root);
    let content = fs::read_to_string(path).unwrap();

    'outer: for line in content.lines() {
        if line == "$ cd /" || line == "$ ls" || line.starts_with("dir") {
            continue;
        }

        if line == "$ cd .." {
            let filesystem_clone = Rc::clone(&filesystem);
            filesystem = Rc::clone(filesystem_clone.borrow().parent.as_ref().unwrap());
            continue;
        }

        if line.starts_with("$ cd") {
            let dir = &line[4..];
            let filesystem_clone = Rc::clone(&filesystem);

            for children in filesystem_clone.borrow().childrens.iter() {
                if children.borrow().name == dir {
                    filesystem = Rc::clone(children);
                    continue 'outer;
                }
            }

            let f = Rc::new(RefCell::new(Filesystem::new(dir.to_string())));
            filesystem.borrow_mut().childrens.push(Rc::clone(&f));
            f.borrow_mut().parent = Some(Rc::clone(&filesystem));

            filesystem = f;

            continue;
        }

        let file: Vec<&str> = line.split_whitespace().collect();
        let size: i32 = file[0].parse().unwrap();
        let f = Rc::new(RefCell::new(Filesystem::new(file[1].to_string())));
        filesystem.borrow_mut().childrens.push(Rc::clone(&f));
        let mut x = f.borrow_mut();
        x.parent = Some(Rc::clone(&filesystem));
        x.add_size(size);
    };

    root
}

fn print_part_one_result(x: Rc<RefCell<Filesystem>>, expected: Option<i32>) {
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

fn items_checker(ic: Rc<RefCell<Filesystem>>) -> i32 {
    let mut x = 0;
    let filesystem = ic.borrow();
    if filesystem.name.starts_with(" ") && filesystem.size < 100000 {
        x += filesystem.size;
    }
    for children in filesystem.childrens.iter() {
        let c = children.clone();
        x += items_checker(c);
    }

    x
}

fn run_part_two(path: &String, expected: Option<i32>) {
    let x = read_file(path);
    print_part_two_result(x, expected);
}

fn print_part_two_result(x: Rc<RefCell<Filesystem>>, expected: Option<i32>) {
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

fn items_checker2(ic: Rc<RefCell<Filesystem>>) -> i32 {
    let filesystem = ic.borrow();
    let x = filesystem.size - 40000000;

    get_value(ic.clone(), x, 70000000)
}

fn get_value(filesystem: Rc<RefCell<Filesystem>>, min: i32, max: i32) -> i32 {
    let mut m = max;
    for children in filesystem.borrow().childrens.iter() {
        let c = children.clone();
        let c_borrow = c.borrow();
        if c_borrow.name.starts_with(" ") {
            if c_borrow.size >= min && c_borrow.size < m {
                m = c_borrow.size;
            }

            let aux = get_value(c.clone(), min, m);
            if aux >= min && aux < m {
                m = aux
            }
        }
    }

    m
}

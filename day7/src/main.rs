use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::{Rc, Weak};
use std::time::Instant;

#[derive(Debug)]
struct Directory {
    parent: RefCell<Weak<Directory>>,
    subdirectories: RefCell<Vec<Rc<Directory>>>,
    name: String,
    files: RefCell<Vec<(usize, String)>>,
    size: RefCell<usize>,
    size_below_100k: RefCell<usize>
}

impl Directory {
    fn base(name: String) -> Directory {
        Directory {
            parent: RefCell::new(Weak::new()),
            subdirectories: RefCell::new(vec![]),
            files: RefCell::new(vec![]),
            size: RefCell::new(0),
            name,
            size_below_100k: RefCell::new(0)
        }
    }

    fn add_subdirectory(&self, d: Rc<Directory>) {
        self.subdirectories.borrow_mut().push(d);
    }

    fn add_file(&self, file: (usize, String)) {
        self.files.borrow_mut().push(file);
    }

    fn calculate_size(&self) -> usize {
        let mut size = 0;
        for f in self.files.borrow().iter() {
            size += f.0;
        }

        for d in self.subdirectories.borrow_mut().iter_mut() {
            size += d.calculate_size();
        }

        if size <= 100000 {
            self.size_below_100k.replace(size);
        }

        self.size.replace(size);
        size
    }

    fn get_subdirectory_by_name(&self, name: String) -> Rc<Directory> {
        let c = Rc::clone(self.subdirectories.borrow().iter().find(|v| v.name == name).unwrap());
        c
    }

    fn print_directory_tree(&self, depth: usize) {
        let mut padding = String::new();
        for _ in 0..depth {
            padding += " ";
        }
        let s = format!("{0}|-{1} ({2}) [{3}]", padding, self.name, self.size.borrow(), self.size_below_100k.borrow());
        println!("{s}");

        padding += " ";
        for f in self.files.borrow().iter() {
            let s = format!("{0}|-{1} ({2})", padding, f.1, f.0);
            println!("{s}");
        }

        for d in self.subdirectories.borrow().iter() {
            d.print_directory_tree(depth+1);
        }
    }

    fn sum_size_below_100k(&self) -> usize {
        let mut total = 0;
        total += *self.size_below_100k.borrow();

        for sub in self.subdirectories.borrow().iter() {
            total += sub.sum_size_below_100k();
        }

        total
    }

    fn smallest_directory_to_delete_size(&self, need_to_delete: usize) -> usize {
        let mut delete_size = *self.size.borrow();
        for sub in self.subdirectories.borrow().iter() {
            let sub_delete_size = sub.smallest_directory_to_delete_size(need_to_delete);
            if sub_delete_size >= need_to_delete && sub_delete_size < delete_size {
                delete_size = sub_delete_size;
            }
        }
        delete_size
    }
}

fn main() {
    let start_time = Instant::now();
    let part_one_oop_result = part_one_oop();
    println!("Part one OOP: {}", part_one_oop_result);
    let duration = start_time.elapsed();
    println!("{:?}", duration);

    let start_time = Instant::now();
    let part_two_oop_result = part_two_oop();
    println!("Part two OOP: {}", part_two_oop_result);
    let duration = start_time.elapsed();
    println!("{:?}", duration);

    let start_time = Instant::now();
    let part_one_result = part_one();
    println!("Part one: {}", part_one_result);
    let duration = start_time.elapsed();
    println!("{duration:?}");

    let start_time = Instant::now();
    let part_two_result = part_two();
    println!("Part two: {}", part_two_result);
    let duration = start_time.elapsed();
    println!("{duration:?}");
}


fn part_one() -> usize {
    let input = fs::read_to_string("input.txt").unwrap();
    let files = parse_input_optimised(input.as_str());

    files
        .iter()
        .filter(|v| v.1 <= 100000)
        .map(|v| v.1)
        .sum()
}

fn part_two() -> usize {
    let input = fs::read_to_string("input.txt").unwrap();
    let files = parse_input_optimised(input.as_str());
    let space_taken = files[0].1;
    let delete_min = space_taken - (70000000 - 30000000);
    files
        .iter()
        .filter(|v| v.1 >= delete_min)
        .map(|v| v.1)
        .min()
        .unwrap()
}

fn parse_input_optimised(input: &str) -> Vec<(Vec<&str>, usize)> {
    let mut lines = input.lines();
    lines.next();
    
    let mut cwd: Vec<&str> = vec!["/"];
    let mut filesys: Vec<(Vec<&str>, usize)> = vec![(vec!["/"], 0)];
    
    for line in lines {
        match line.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
            ["dir", subdir] => {
                let mut dir = cwd.clone();
                dir.push(subdir);
                filesys.push((dir, 0));
            },
            ["$", "cd", ".."] => {
                cwd.pop();
            },
            ["$", "cd", dir] => {
                cwd.push(dir);
            },
            ["$", "ls"] => (),
            [size, _] => {
                let filesize: usize = size.parse().unwrap();
                for i in 0..cwd.len() {
                    let path = cwd[0..=i].to_vec();
                    let index = filesys.iter().position(|v| v.0 == path).unwrap();
                    filesys[index].1 += filesize;
                }
            },
            _ => ()
        }
    }

    filesys
}

fn _parse_input() -> HashMap<String, usize> {
    let mut files = HashMap::new();
    files.insert(String::from("/"), 0);

    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    lines.next();

    let mut current_path = String::from("/");
    let mut parents: Vec<String> = vec![];

    for line in lines {
        match line.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
            ["dir", dir] => {
                files.insert(String::from(current_path.to_string() + dir + "/"), 0);
            },
            ["$", "cd", ".."] => {
                let l = current_path.len();
                let last_index = current_path[0..l-1].rfind("/").unwrap();
                if last_index == 0 {
                    current_path = String::from("/");
                    parents = vec![];
                } else {
                    current_path = current_path[0..=last_index].to_string();
                    parents.pop();
                }
            },
            ["$", "cd", dir] => {
                parents.push(current_path.to_string());
                current_path = current_path.to_string() + format!("{0}/", dir).as_str();
            },
            ["$", "ls"] => (),
            [size, _] => {
                let file_size = size.parse::<usize>().unwrap();
                let value = files.get_mut(&current_path).unwrap();
                *value += file_size;

                // need to add to all parent directories too
                for parent in parents.iter() {
                    let value = files.get_mut(parent).unwrap();
                    *value += file_size;
                }
            },
            _ => ()
        }
    }

    files
}

fn part_one_oop() -> usize {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    lines.next();
    
    let base_directory = Rc::new(Directory::base(String::from("/")));
    let mut current_directory = Rc::clone(&base_directory);

    for (index, line) in lines.clone().enumerate() {
        match line.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
            ["$", "ls"] => {
                let v = lines.clone().collect::<Vec<&str>>();
                for f in &v[index+1..] {
                    match f.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
                        ["dir", dir] => {
                            let new_directory = Rc::new(Directory {
                                name: String::from(dir),
                                subdirectories: RefCell::new(vec![]),
                                parent: RefCell::new(Rc::downgrade(&current_directory)),
                                files: RefCell::new(vec![]),
                                size: RefCell::new(0),
                                size_below_100k: RefCell::new(0)
                            });
                            current_directory.borrow_mut().add_subdirectory(new_directory);
                        },
                        ["$", _, _] => (),
                        ["$", _] => (),
                        [size, filename] => {
                            let file = (size.parse::<usize>().unwrap(), String::from(filename));
                            current_directory.add_file(file);
                        },
                        _ => ()
                    }
                    if f.chars().nth(0) == Some('$') {
                        break;
                    }
                }
            },
            ["$", "cd", ".."] => {
                let p = Rc::clone(&current_directory.parent.borrow().upgrade().unwrap());
                current_directory = p;
            },
            ["$", "cd", dir] => {
                current_directory = Rc::clone(&current_directory.get_subdirectory_by_name(String::from(dir)));
            },
            _ => ()
        }
    }

    base_directory.calculate_size();
    base_directory.print_directory_tree(0);

    let total_size_below_100k = base_directory.sum_size_below_100k();
    total_size_below_100k
}

fn part_two_oop() -> usize {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    lines.next();
    
    let base_directory = Rc::new(Directory::base(String::from("/")));
    let mut current_directory = Rc::clone(&base_directory);

    for (index, line) in lines.clone().enumerate() {
        match line.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
            ["$", "ls"] => {
                let v = lines.clone().collect::<Vec<&str>>();
                for f in &v[index+1..] {
                    match f.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
                        ["dir", dir] => {
                            let new_directory = Rc::new(Directory {
                                name: String::from(dir),
                                subdirectories: RefCell::new(vec![]),
                                parent: RefCell::new(Rc::downgrade(&current_directory)),
                                files: RefCell::new(vec![]),
                                size: RefCell::new(0),
                                size_below_100k: RefCell::new(0)
                            });
                            current_directory.borrow_mut().add_subdirectory(new_directory);
                        },
                        ["$", _, _] => (),
                        ["$", _] => (),
                        [size, filename] => {
                            let file = (size.parse::<usize>().unwrap(), String::from(filename));
                            current_directory.add_file(file);
                        },
                        _ => ()
                    }
                    if f.chars().nth(0) == Some('$') {
                        break;
                    }
                }
            },
            ["$", "cd", ".."] => {
                let p = Rc::clone(&current_directory.parent.borrow().upgrade().unwrap());
                current_directory = p;
            },
            ["$", "cd", dir] => {
                current_directory = Rc::clone(&current_directory.get_subdirectory_by_name(String::from(dir)));
            },
            _ => ()
        }
    }

    base_directory.calculate_size();
    // base_directory.print_directory_tree(0);

    const TOTAL_SPACE : usize = 70000000;
    const REQUIRED_SPACE : usize = 30000000;
    let space_taken = base_directory.size.borrow();

    let need_to_delete = *space_taken - (TOTAL_SPACE - REQUIRED_SPACE);

    let smallest_size = base_directory.smallest_directory_to_delete_size(need_to_delete);
    smallest_size
}
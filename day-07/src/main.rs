use std::borrow::BorrowMut;
use std::collections::{HashMap};
use std::error::Error;
use std::fs;
use std::vec::IntoIter;

trait Sizeable {
    fn get_total_size(&mut self) -> usize;
}

#[derive(Clone, Copy, Debug)]
struct File {
    size: usize,
}

impl File {
    fn new(size: usize) -> Self {
        Self {
            size,
        }
    }
}

impl Sizeable for File {
    fn get_total_size(&mut self) -> usize {
        self.size
    }
}

#[derive(Clone, Debug)]
struct Dir {
    childs: HashMap<String, Dir>,
    files: HashMap<String, File>,
}

impl Dir {
    fn new() -> Self {
        Self {
            childs: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn add_child(&mut self, name: String) -> &mut Dir {
        self.childs.insert(name.clone(), Dir::new());
        self.childs.get_mut(&name).unwrap()
    }

    fn add_file(&mut self, name: String, size: usize) {
        self.files.insert(name, File::new(size));
    }

    fn get_child(&mut self, name: String) -> Option<&mut Dir> {
        self.childs.get_mut(&name)
    }

    fn has_child(&self, name: String) -> bool {
        self.childs.contains_key(&name)
    }

    fn flat(&self) -> IntoIter<Dir> {
        self.childs
            .values()
            .map(|dir: &Dir| dir.flat())
            .flatten()
            .chain(
                vec![self.clone()].into_iter()
            )
            .collect::<Vec<Dir>>()
            .into_iter()
    }
}

impl Sizeable for Dir {
    fn get_total_size(&mut self) -> usize {
        self
            .files
            .values_mut()
            .map(|file: &mut File| file.get_total_size())
            .chain(
                self.childs.values_mut().map(|dir: &mut Dir| dir.get_total_size())
            )
            .sum::<usize>()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("day-07/input.txt")?;
    let mut root = Dir::new();
    let mut current: &mut Dir = root.borrow_mut();
    let mut stack: Box<Vec<String>> = Box::new(vec![]);
    for line in input.lines() {
        if line.starts_with('$') {
            let args: Vec<&str> = line.split(' ').collect();
            match args[1] {
                "cd" => {
                    match args[2] {
                        "/" => {
                            stack.clear();
                            current = root.borrow_mut();
                        },
                        ".." => {
                            current = root.borrow_mut();
                            stack.pop();
                            for segment in stack.iter() {
                                current = current.get_child(segment.clone()).unwrap();
                            }
                        },
                        _ => {
                            let path = args[2].to_string();
                            stack.push(path.clone());
                            if current.has_child(path.clone()) {
                                current = current.get_child(path).unwrap();
                            } else {
                                current = current.add_child(path);
                            }
                        }
                    }
                },
                "ls" => {},
                _ => {
                    println!("UNKNOWN COMMAND")
                }
            }
        } else if line.starts_with("dir") {
            current.add_child(line.split(' ').last().unwrap().to_string());
        } else {
            let size_and_name = line.split(' ').collect::<Vec<&str>>();
            current.add_file(size_and_name.last().unwrap().to_string(), size_and_name.first().unwrap().parse::<usize>().unwrap());
        }
    }

    let flat = root.flat().map(|mut dir| dir.borrow_mut().get_total_size());
    println!("Task 1: {}", flat.clone().filter(|value| *value <= 100000).sum::<usize>());

    let total_used = root.get_total_size();
    let free = 70000000 - total_used;
    let needed = 30000000 - free;

    println!("Task 2: {}", flat.filter(|value| *value >= needed).min().unwrap());

    Ok(())
}


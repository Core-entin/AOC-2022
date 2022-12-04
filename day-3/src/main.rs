use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let priority = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut score = 0;
    let mut bags: Vec<String> = vec![];
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                bags.push(ip);
            }
            if bags.len() == 3 {
                for s in bags[0].chars() {
                    if bags[1].contains(s) && bags[2].contains(s) {
                        score += priority.chars().position(|x| x == s).unwrap();
                        break;
                    }
                }
                bags.clear();
            }
        }
    }

    println!("Score is {}", score);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
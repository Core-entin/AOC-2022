use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let priority = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut score = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                // Logic
                let (first, last) = ip.split_at(ip.len() / 2);

                for s in first.chars() {
                    if last.contains(s) {
                        score += priority.chars().position(|x| x == s).unwrap();
                        break;
                    }
                }

                println!("score is {}", score);
            }
        }
    }

    // println!("Max is {}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
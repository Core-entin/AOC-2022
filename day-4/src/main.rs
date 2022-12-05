use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn between(a: i32, b: i32, c:i32) -> bool {
    return (a >= b && a <= c) || (a >= c && a <= b);
}

fn main() {
    let reg = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut overlap = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                // Logic
                for l in reg.captures_iter(&ip) {
                    // Ugly parsing :(
                    let a = l[1].parse::<i32>().unwrap();
                    let b = l[2].parse::<i32>().unwrap();
                    let c = l[3].parse::<i32>().unwrap();
                    let d = l[4].parse::<i32>().unwrap();

                    // Solution 1
                    // if (a <= c && b >= d) || (c <= a && d >= b) {
                    //     overlap += 1;
                    // }     
                    // Solution 2
                    if between(a, c, d) || between(b, c, d) || between(c, a, b) || between(d, a, b) {
                        overlap += 1;
                    }                 
                }
            }
        }
    }

    println!("Value is {}", overlap);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
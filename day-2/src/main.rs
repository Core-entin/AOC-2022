use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut score = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let split = ip.split(" ");
                let values: Vec<&str> = split.collect();
                // println!("{}", ip);

                // Logic
                if values[0] == "A" { // Rock
                    score += match values[1] {
                        "X" => 3,
                        "Y" => 4,
                        "Z" => 8,
                        &_ => -10000000,
                    }
                }
                else if values[0] == "B" { // Paper
                    score += match values[1] {
                        "X" => 1,
                        "Y" => 5,
                        "Z" => 9,
                        &_ => -10000000,
                    }
                }
                else if values[0] == "C" { // Scissors
                    score += match values[1] {
                        "X" => 2,
                        "Y" => 6,
                        "Z" => 7,
                        &_ => -10000000,
                    }
                }
            }
        }
    }
    println!("Result is {}", score);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
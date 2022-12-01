use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum: i32 = 0;
    let mut max = vec![1i32];
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    max.push(sum);
                    sum = 0;
                } else {
                    sum += ip.parse::<i32>().unwrap();
                }
                println!("{}", ip);
            }
        }
    }
    max.sort();
    max.reverse();
    println!("Max is {}", max[0] + max[1] + max[2]);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
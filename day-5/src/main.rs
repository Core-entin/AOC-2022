use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn solve(opt: Option<&str>) -> &str {
    return match opt {
        Some(p) => p,
        None => ""
    }
}

fn main() {
    let step_reg = Regex::new(r"move\s([\d]+)\sfrom\s([\d]+)\sto\s([\d]+)$").unwrap();

    // Not a cheater
    let mut crates: [Vec<&str>; 9] = [
        vec!["Z", "P", "M", "H", "R"],
        vec!["P", "C", "J", "B"],
        vec!["S", "N", "H", "G", "L", "C", "D"],
        vec!["F", "T", "M", "D", "Q", "S", "R", "L"],
        vec!["F", "S", "P", "Q", "B", "T", "Z", "M"],
        vec!["T", "F", "S", "Z", "B", "G"],
        vec!["N", "R", "V"],
        vec!["P", "G", "L", "T", "D", "V", "C", "M"],
        vec!["W", "Q", "N", "J", "F", "M", "L"]
    ];
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if step_reg.is_match(&ip) {
                    let caps   = step_reg.captures(&ip).unwrap();
                    let amount = caps[1].parse::<usize>().unwrap();
                    let from   = caps[2].parse::<usize>().unwrap() - 1;
                    let to     = caps[3].parse::<usize>().unwrap() - 1;


                    // Solution 1
                    // for n in 0..amount {
                    //     let crt = solve(crates[from].pop());
                    //     crates[to].push(crt);
                    // }

                    // Solution 2
                    let mut tmp: Vec<&str> = vec![];
                    for _n in 0..amount {
                        tmp.push(solve(crates[from].pop()));
                    }
                    for _n in 0..amount {
                        crates[to].push(solve(tmp.pop()));
                    }
                }
            }
        }

        let res = crates
            .map(|mut x| x.pop())
            .map(|s| solve(s))
            .join("");
        println!("Res is {}", res);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
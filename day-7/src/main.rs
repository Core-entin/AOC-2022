use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let cmd_reg = Regex::new(r"\$\scd\s([\w/]+|\.\.)").unwrap();
    let file_reg = Regex::new(r"(\d+)\s").unwrap();
    let up_reg = Regex::new(r"\.\.").unwrap();
    let mut current_folder: String = String::new();
    let mut files: HashMap<String, u64> = HashMap::new();
 
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                // Logic
                // println!("{}", ip);
                if ip.chars().next().unwrap() == '$' {

                    if cmd_reg.is_match(&ip) { // CD Folder
                        let caps = cmd_reg.captures(&ip).unwrap();
                        let mut folder = caps[1].parse::<String>().unwrap();
                        if !folder.contains('/') {
                            folder.insert(0, '/');
                        }
                        if up_reg.is_match(&folder) { // move up
                            let mut tmp: Vec<&str> = current_folder.split('/').collect();
                            tmp.pop();
                            current_folder = tmp.join("/");
                        } else { // move back
                            current_folder.insert_str(current_folder.clone().len(), &folder);
                        }

                        // println!("{}", folder);
                        files.entry(current_folder.to_string()).or_insert(0);
                    }

                } else {

                    if file_reg.is_match(&ip) { // Listed File
                        let caps = file_reg.captures(&ip).unwrap();
                        let size = caps[1].parse::<u64>().unwrap();
                        let cp = files.clone();
                        for (key, _val) in cp.iter() {
                            if key.contains(&current_folder.to_string()) {
                                files.entry(key.to_string()).and_modify(|f| *f += size).or_insert(size);
                            }
                        }
                        // println!("{}", size);
                    }
                }
            }
            // break;
        }
    }

    let mut ta: u64 = 0;
    for (key, value) in files.iter() {
        if value > &100000 {
            ta += value;
            println!("{}, {}", key, value);
        }
    }
    println!("{}", ta);

    // println!("Value is {}", value);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
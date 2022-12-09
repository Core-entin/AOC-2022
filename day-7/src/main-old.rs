use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

enum FileType {
    File,
    Dir
}

struct Node {
    filetype: FileType,
    parent: Box<Option<Node>>,
    files: Vec<Node>,
    size: u64,
}

fn main() {
    let cmd_reg = Regex::new(r"\$\scd\s([\w/]+|\.\.)").unwrap();
    let file_reg = Regex::new(r"(\d+)\s").unwrap();
    let mut nodes = new_node(FileType::Dir, Box::new(None), 0);
    let mut current_node: Node = new_node(FileType::Dir, Box::new(None), 0);
 
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                // Logic
                println!("{}", ip);
                if ip.chars().next().unwrap() == '$' {

                    if cmd_reg.is_match(&ip) { // CD Folder
                        let caps = cmd_reg.captures(&ip).unwrap();
                        let folder = caps[1].parse::<String>().unwrap();
                        // println!("{}", folder);
                        let node: Node = new_node(FileType::Dir, Box::new(Some(current_node)), 0);
                        current_node.files.push(node);
                        current_node = node;
                    }

                } else {

                    if file_reg.is_match(&ip) { // Listed File
                        let caps = file_reg.captures(&ip).unwrap();
                        let size = caps[1].parse::<u64>().unwrap();
                        // println!("{}", size);
                        let node: Node = new_node(FileType::File, Box::new(Some(current_node)), size);
                        nodes.files.push(node);
                        nodes.size += size;
                    }
                }
            }
        }
    }

    // println!("Value is {}", value);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn new_node (filetype: FileType, parent: Box<Option<Node>>, size: u64) -> Node {
    return Node {
        filetype: filetype,
        files: vec![],
        parent: parent,
        size: size,
    }
}

fn get_size(node: Node) -> u64 {
    let mut size = node.size;
    for i in node.files.into_iter() {
        size += get_size(i);
    }
    return size;
}
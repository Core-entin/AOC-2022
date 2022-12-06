use std::fs;

fn main() {
    if let Ok(content) = fs::read_to_string("./input.txt") {
        let mut chars: Vec<char> = vec![];
        let mut index = 0;
        for c in content.chars() {
            chars.push(c);
            index += 1;
            if chars.len() == 14 { // 4 instead of 14 for part 1
                let mut is_index = true;

                for i in &chars { // &char for filter below
                    let copy = chars.to_vec(); // Copy because into_iter() borrow the vector
                    let dupes = copy.into_iter().filter(|x| x == i).collect::<Vec<char>>();

                    // If at least one dupe
                    if dupes.len() > 1 {
                        is_index = false;
                        break;
                    }
                }
                if is_index {
                    println!("index is {}", index);
                    dbg!(chars);
                    break;
                } 
                chars.remove(0);
            }
        }
    }
}

use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    part1(contents);
}

fn part1(contents: String) {
    // make empty set
    let mut set: HashSet<char> = HashSet::new();
    for (i, c) in contents.chars().enumerate() {
        if set.len() < 14 {
            if set.contains(&c) {
                set.clear();
            }
            set.insert(c);
        } else {
            println!("{}", i);
            break;
        }
    }
}

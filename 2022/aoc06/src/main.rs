use std::fs;
use std::collections::VecDeque;

fn main() {
    // let prank = "Get pranked bro!";
    // println!("{}", prank);
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    part1(contents);
}

fn part1(contents: String) {
    let mut marker: VecDeque<char>  = VecDeque::new();
    for (i, c) in contents.chars().enumerate() {
        if marker.len() < 14 {
            // pranked();
            if marker.contains(&c) {
                let mut index = 0;
                for (x, ch) in marker.iter().enumerate() {
                    if *ch == c {
                        index = x;
                        break;
                    }
                }
                marker.drain(0..index+1);
            }
            marker.push_back(c);
        } else {
            println!("First marker: {}", i);
            break;
        }
    }
}

// fn pranked(){
//     println!("Get pranked vesty")
// }
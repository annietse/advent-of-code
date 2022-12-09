use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read from file");
    // println!("Hello, world!");
    part1(contents);
}

fn part1(contents: String) {
    let mut head: [i32;2] = [0;2];
    let mut tail: [i32;2] = [0;2];
    let mut coords: HashSet<[i32;2]> = HashSet::new();
    coords.insert(tail);
    for line in contents.lines() {
        let v: Vec<&str> = line.split(" ").collect();
        let moves = v[1].parse::<i32>().unwrap();
        for _ in 0..moves {
            match v[0] {
                "U" => head[1] += 1,
                "R" => head[0] += 1,
                "D" => head[1] -= 1,
                _ => head[0] -= 1
            }
            // check if head is in one of the surrounding spaces of tail
            let x_diff = head[0] - tail[0];
            let y_diff = head[1] - tail[1];
            
            let x_magnitude;
            match x_diff {
                0.. => x_magnitude = x_diff,
                _ => x_magnitude = -x_diff,
            }
            let y_magnitude;
            match y_diff {
                0.. => y_magnitude = y_diff,
                _ => y_magnitude = -y_diff,
            }
            // y and x out of range so diagonal
            // y out of range only so move y
            // x out of range only so move x
            // not out of range so do not move
            // println!("{} {}", x_diff, x_magnitude);
            if x_magnitude > 1 && y_magnitude == 1 || y_magnitude > 1 && x_magnitude == 1 {
                // move diagonally
                // find direction of y and x
                // move once in 2 directions
                if x_diff > 0 {
                    tail[0] += 1;
                    // println!("right");
                } else {
                    tail[0] += -1;
                    // println!("left");
                }
                if y_diff > 0 {
                    tail[1] += 1;
                    // println!("up");
                } else {
                    tail[1] += -1;
                    // println!("down");
                }
                
            } else  {
                if head[0] == tail [0] &&  y_magnitude > 1 {
                    if y_diff > 0 {
                        tail[1] += 1;
                        // println!("up");
                    } else {
                        tail[1] -= 1;
                        // println!("down");
                    }
                } else if head[1] == tail[1] && x_magnitude > 1 {
                    if x_diff > 0 {
                        tail[0] += 1;
                        // println!("right");
                    } else {
                        tail[0] -= 1;
                        // println!("left");
                    }

                }
                // if to the right, x + 1
                // if to the left, x - 1
                // up, y + 1
                // down, y - 1
                // if neither x or y are the same as the head,
                // move diagonally once
            }
            // println!("h {:?}", head);
            // println!("t {:?}", tail);
            coords.insert(tail);
            
        }
    }
    println!("{}", coords.len());
}
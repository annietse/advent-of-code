use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read from file");
    part2(contents);
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

fn part2(contents: String) {
    // array of 10 knots
    // compare against knot in front of it at all times
    // only add the 9nth knots position to the hashset
    let mut coords: HashSet<[i32;2]> = HashSet::new();
    coords.insert([0,0]);
    let mut knot: [[i32;2];10] = [[0,0];10];
    // knot positions
        
    for line in contents.lines() {
        // get intruction
        let v: Vec<&str> = line.split(" ").collect();
        let moves = v[1].parse::<i32>().unwrap();
        for _ in 0..moves {
            for i in 0..9 {
            // head and tail knots retrieved
            // repeats through all the moves
            
                let mut head: [i32;2] = knot[i];
                let mut tail: [i32;2] = knot[i+1];
                // changes only the first one
                if i == 0 {
                    match v[0] {
                        "U" => head[1] += 1,
                        "R" => head[0] += 1,
                        "D" => head[1] -= 1,
                        _ => head[0] -= 1
                    }
                }
                // compares the head and tail
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
                // moves tail accordingly
                if x_magnitude > 1 && y_magnitude == 1 || y_magnitude > 1 && x_magnitude == 1 || x_magnitude > 1 && y_magnitude > 1 {
                    // move diagonally
                    // find direction of y and x
                    // move once in 2 directions
                    // println!("Diagonal");
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
                    // up down right left
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
                }
                // add head and tail to knots array
                //last knot, add 
                knot[i] = head;
                knot[i+1] = tail;
                coords.insert(knot[9]);
            }
        }
        
        // println!("{:?}", knot);
    }
    println!("{}", coords.len());
    // println!("{:?}", knot); 
}

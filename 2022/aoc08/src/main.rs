use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read from file");
    // part1(contents);
    part2(contents);
}

fn part1(contents: String) {
    let mut trees: Vec<Vec<u32>> = Vec::new();
    for line in contents.lines() {
        let v: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        trees.push(v);
    }
    
    let height = trees.len();
    let width = trees[0].len();
    let mut count = height*2 + width*2 - 4;
    for i in 1..height-1 {
        for j in 1..width-1 {
            let mut vis = false;
            let t = trees[i][j];
            let mut row = trees[i].clone();
            let right: Vec<u32> = row.drain(j..).skip(1).collect();
            // println!("{:?}", row);
            // println!("{:?}", right);
            //compare sideways
            if t > *right.iter().max().unwrap() {
                vis = true;
            } else if t > *row.iter().max().unwrap() {
                vis = true;
            } else {
                //columns compare
                let mut col: Vec<u32> = Vec::new();
                for x in trees.iter() {
                    col.push(x[j]);
                }
                let down: Vec<u32> = col.drain(i..).skip(1).collect();
                // println!("{:?}", col);
                // println!("{:?}", down);
                if t > *col.iter().max().unwrap() {
                    vis = true;
                } else if t > *down.iter().max().unwrap() {
                    vis = true;
                }
            }
            if vis == true {
                // println!("{}",t);
                count += 1;
            }
        }
    }
    println!("{}", count);
}


fn part2(contents: String) {
    let mut trees: Vec<Vec<u32>> = Vec::new();
    for line in contents.lines() {
        let v: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        trees.push(v);
    }
    let mut greatest_scenery_score = 0;
    let height = trees.len();
    let width = trees[0].len();
    for i in 1..height-1 {
        for j in 1..width-1 {
            let t = trees[i][j];
            // println!("tree {}", t);
            let mut row = trees[i].clone();
            let right: Vec<u32> = row.drain(j..).skip(1).collect();
            let left: Vec<u32> = row.into_iter().rev().collect();
            let mut col: Vec<u32> = Vec::new();
            for x in trees.iter() {
                col.push(x[j]);
            }
            let down: Vec<u32> = col.drain(i..).skip(1).collect();
            let up: Vec<u32> = col.into_iter().rev().collect();
            let directions: [Vec<u32>; 4] = [left, right, up, down];
            
            let mut distances : [usize; 4] = [0;4];
            for (p, d) in directions.iter().enumerate() {
                let mut distance;
                // sets it to length of the direction automatically
                distance = d.len();
                // iterates left, right, up, down
                for (i, x) in d.iter().enumerate() {
                    if t <= *x {
                        // cannot see further
                        distance = i+1;
                        break;
                    }
                }
                distances[p] = distance;
            }
            let mut total = 1;
            // println!("{:?}", distances);
            for x in distances {
                total *= x;
            }
            // println!("{}", total);
            if total > greatest_scenery_score {
                greatest_scenery_score = total;
            }
        }
    }
    println!("{}", greatest_scenery_score);
}

use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read from file");
    part1(contents)
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

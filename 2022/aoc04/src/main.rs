use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

fn main() {
    let mut count: u32 = 0;
    if let Ok(lines) = read_lines("./hosts") {
        for line in lines {
            if let Ok(ip) = line {
                // println!("{}", ip);
                let pair = ip.split(",");
                let mut limits: Vec<u32> = Vec::new();
                for item in pair {
                    // println!("{:?}", item);
                    let lims = item.split("-");
                    for l in lims {
                        let limit = l.parse::<u32>().unwrap();
                        limits.push(limit);
                    }
                }
                // println!("{:?}", limits);
                // count = part1(limits, count);
                count = part2(limits, count);
            }
        }
    }
    println!("Count: {}", count);
}

fn part1(limits: Vec<u32>, mut count: u32) -> u32 {
    if limits[0] <= limits[2] && limits[1] >= limits[3]
        || limits[0] >= limits[2] && limits[1] <= limits[3]
    {
        count += 1;
    }
    return count;   
}

fn part2(limits: Vec<u32>, mut count: u32) -> u32 {
    if !(limits[0] > limits[3] || limits[1] < limits[2]) {
        count += 1;
    }
    return count;
}

fn read_lines<P>(input: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open("input.txt")?;
    Ok(io::BufReader::new(file).lines())
}


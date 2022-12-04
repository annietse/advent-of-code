use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

fn main() {
    part1()
}

fn part1() {
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
                println!("{:?}", limits);
                if limits[0] <= limits[2] && limits[1] >= limits[3]
                    || limits[0] >= limits[2] && limits[1] <= limits[3]
                {
                    count += 1;
                }
            }
        }
    }
    println!("Count: {}", count);
}



fn read_lines<P>(input: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open("input.txt")?;
    Ok(io::BufReader::new(file).lines())
}


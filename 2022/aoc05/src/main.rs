use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;

fn main() {
    part1()
}

fn part1() {
    let mut array: [Vec<char>; 9] = Default::default();
    let mut array_complete = false;
    if let Ok(lines) = read_lines("./hosts") {
        for line in lines {
            if let Ok(ip) = line {
                // create vec
                let v: Vec<char> = ip.chars().collect();
                if v.len() != 0 && v[1] == '1'{
                    array_complete = true;
                }
                if array_complete == false {
                    for n in 1..ip.chars().count() {
                        if v[n] != ' '&& v[n] != '['&& v[n] != ']' {
                            // println!("{} {}",v[n], (n+2)/4);
                            let index = (n+2)/4;
                            array[index].insert(0, v[n]);
                            // println!("{:?}", array[index]);
                        }
                        // println!("{:?}", array[index]);
                    }
                    
                }
                if v.len() != 0 && v[0] == 'm' {
                    let instruction: Vec<&str> = ip.split(" ").collect();
                    let count = instruction[1].parse::<usize>().unwrap();
                    let location = instruction[3].parse::<usize>().unwrap()-1;
                    let destination = instruction[5].parse::<usize>().unwrap()-1;
                    for _i in 0..count {
                        let temp = array[location].last().unwrap();
                        array[destination].push(*temp);
                        array[location].pop();
                    }
                }
                // while no numbers found
            }
        }
    }
    let mut result: Vec<char> = Vec::new();
    for item in array {
        result.push(*item.last().unwrap());
    }
    println!("{:?}",result);
    
}


fn read_lines<P>(_input: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open("input.txt")?;
    Ok(io::BufReader::new(file).lines())
}
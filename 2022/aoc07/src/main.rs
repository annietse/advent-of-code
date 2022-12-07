use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read from file");
    // let mut p = PathBuf::new();
    // p.push(r"C:\");
    // p.push("windows");
    // p.push("system32");
    // println!("{:?}", p);
    part1(contents);
}

fn part1(contents: String) {
    let mut path = Vec::new();
    let mut map = HashMap::new();
    for line in contents.lines() {
        let command: Vec<&str> = line.split(" ").collect();
            if command.contains(&"cd") {
                let dir = command[2];
                match dir {
                    ".." => {
                        path.pop();
                    }
                    "/" => {
                        path.clear();
                    }
                    _ => {
                        path.push(dir);
                        map.insert(path.join(""), 0);
                        // println!("{:?}", path);
                    }
                }
            } else if !command.contains(&"$") && !command.contains(&"dir") {
                let n = command[0].parse::<i64>().unwrap();
                // get all path dir
                // add to each value on map
                for x in 0..path.len() {
                    let mut temp_path = path.clone();
                    temp_path.drain(path.len()-x..);
                    let p = temp_path.join("");
                    let value = map.get(&p).unwrap() + n;
                    map.insert(p, value);
                }
            }
    }
    // add up total of map values, exclude over 100,000
    let mut total = 0;
    for val in map.values() {
        if val <= &100000{
            total += val;
        }
    }
    println!("sum: {}", total);
}
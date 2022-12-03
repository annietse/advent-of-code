use std::fs;

fn main() {
    part1();
}

fn part1() {
    let a = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let alphabet: Vec<char> = a.chars().collect();
    let mut sum: u64 = 0;
    let mut match_count = 0;
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let mut count = 0;
    let lines = contents.lines();
    for l in lines {
        count += 1;
        let rucksack: Vec<char> = l.chars().collect();
        let mut count = 0;
        let mut compart1: Vec<char> = Vec::new();
        let mut compart2: Vec<char> = Vec::new();
        // println!("{:?}, {}", rucksack, rucksack.len());
        for item in &rucksack {
            if count < rucksack.len()/2 {
                compart1.push(*item);
                // println!("{:?}", compart1);
            } else {
                compart2.push(*item);
            }
            count +=1;
        }
        // println!("{:?}, {:?}, {:?}", compart1, compart2, rucksack.len());
        
        'outer: for &item1 in compart1.iter() {
            // println!("yessir");
            for &item2 in compart2.iter() {
                // println!("{}, {}", item1, item2);
                if item1 == item2 {
                    for n in 0..52 {
                        // println!("{:?}, {}",alphabet[n], n);
                        if alphabet[n] == item1 {
                            // println!("Match found");
                            match_count += 1;
                            let x: u64 = n.try_into().unwrap();
                            sum += x+1;
                            // println!("{}, {}", alphabet[n], x+1);
                            // println!("count: {}", match_count);
                            break 'outer;
                        }
                        
                    }
                }
                
            }
        }
    }
    println!("Sum: {}", sum);
    println!("Count: {}", count);
}


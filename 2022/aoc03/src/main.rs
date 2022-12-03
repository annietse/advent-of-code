use std::fs;

fn main() {
    part2();
}

fn part1() {
    let a = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let alphabet: Vec<char> = a.chars().collect();
    let mut sum: u64 = 0;
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
        for item in &rucksack {
            if count < rucksack.len()/2 {
                compart1.push(*item);
            } else {
                compart2.push(*item);
            }
            count +=1;
        }
        
        'outer: for &item1 in compart1.iter() {
            for &item2 in compart2.iter() {
                if item1 == item2 {
                    for n in 0..52 {
                        if alphabet[n] == item1 {
                            let x: u64 = n.try_into().unwrap();
                            sum += x+1;
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

fn part2() {
    let a = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let alphabet: Vec<char> = a.chars().collect();
    let mut sum: u64 = 0;
    // let mut match_count = 0;
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let mut count = 0;
    let lines = contents.lines();
    let mut elf1: Vec<char> = Vec::new();
    let mut elf2: Vec<char> = Vec::new();
    for l in lines {
        count += 1;
        let rucksack: Vec<char> = l.chars().collect();
        if count%3 ==1 {
            elf1 = rucksack;
        } else if count%3 ==2 {
            elf2 = rucksack;
        } else {
            'outer: for &item1 in elf1.iter() {
                for &item2 in elf2.iter() {
                    for &item3 in rucksack.iter() {
                        if item1 == item2 && item1 == item3 {
                            for n in 0..52 {
                                if alphabet[n] == item1 {
                                    let x: u64 = n.try_into().unwrap();
                                    sum += x+1;
                                    break 'outer;
                                }
                                
                            }
                        }
                        
                    }
                    
                }
            }
        }
        
    }
    println!("Sum: {}", sum);
    println!("Count: {}", count);
}
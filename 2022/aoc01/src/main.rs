use std::fs;

fn main() {
    //part1();
    part2();
}

fn part1() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    
    let lines = contents.lines();
    let mut greatest_calories: u64 = 0;
    let mut total: u64 = 0;
    for l in lines {
        if l != "" {
            let num: u64 = l.parse::<u64>().unwrap();
            total += num;
        } else {
            if greatest_calories < total {
                greatest_calories = total;
            }
            total = 0;
        }
    }
    println!("Greatest calories: {}", greatest_calories);
}

fn part2() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    
    let lines = contents.lines();
    let mut top_3: Vec<u64> = vec![0, 0, 0];
    let mut total: u64 = 0;
    for l in lines {
        if l != "" {
            let num: u64 = l.parse::<u64>().unwrap();
            total += num;
        } else {
            for i in 0..3 {
                if top_3[i] < total{
                    insert_in_place(&mut top_3, total, i);
                    break;
                }
            }
            total = 0;
        }
    }
    let greatest_calories: u64 = top_3[0] + top_3[1] + top_3[2];
    println!("Calories of top 3 elves: {}", greatest_calories);
}

fn insert_in_place<T>(array: &mut [T], value: T, index: usize) {
    *array.last_mut().unwrap() = value;
    array[index..].rotate_right(1);
}
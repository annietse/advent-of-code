use std::fs;

fn main() {
    println!("Hello, world!");

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


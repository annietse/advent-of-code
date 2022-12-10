use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read from file");
    part1(contents);
}

fn part1(contents: String) {
    let mut x = 1;
    let mut cycle = 0;
    let mut count = 0;
    for line in contents.lines() {
        let v : Vec<&str> = line.split(" ").collect();
        let l;
        let mut increase = 0;
        match v[0] {
            "noop" => l = 1,
            _ => {
                l = 2;
                increase = v[1].parse::<i32>().unwrap();
                println!("{}", increase);
            }
        }
        for i in 0..l {
            cycle += 1;
            if cycle <= 220 && (cycle+20)%40 == 0 {
                let sig_strength = x*cycle;
                count += sig_strength;
            }
            if i == 1 {
                x += increase;
            }
        }
        
    }
    println!("Sum: {}", count);
}
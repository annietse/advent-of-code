use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read from file");
    // part1(contents);
    part2(contents);
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

fn part2(contents: String) {
    let mut arr: [[char;40];6] = [['.';40]; 6];
    let mut x: i32 = 1;
    let mut cycle: i32 = 0;
    for line in contents.lines() {
        let v : Vec<&str> = line.split(" ").collect();
        let l;
        let mut increase = 0;
        match v[0] {
            "noop" => l = 1,
            _ => {
                l = 2;
                increase = v[1].parse::<i32>().unwrap();
                // println!("{}", increase);
            }
        }
        for i in 0..l {
            cycle += 1;
            // find positions
            // see if sprite position matches
            // input
            let sprite_pos = (cycle-1)%40;
            // println!("{}", cycle/40);
            let y_pos: usize = (cycle/40).try_into().unwrap();
            if sprite_pos <= x+1 && sprite_pos >= x-1 {
                let x_pos: usize = sprite_pos.try_into().unwrap();
                let c = '#';
                // println!("{},{}", x_pos, y_pos);
                arr[y_pos][x_pos] = c;
            } 
            // println!("{} {}", x_pos, y_pos);
            if i == 1 {
                x += increase;
            }
        }   
    }
    for item in arr {
        for i in item {
            print!("{}", i);
        }
        println!();
    }
}
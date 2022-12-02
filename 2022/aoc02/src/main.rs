use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut score = 0;
    let mut count = 0;
    let lines = contents.lines();
    //iterate through each line
    for l in lines {
        count += 1;
        let chars_vec: Vec<char> = l.chars().collect();
        // Rock - x, a
        if chars_vec[2] == 'X' {
            // increase score for rock by 1
            score += 1;
            if chars_vec[0] == 'C' {
                score += 6;
            } else if chars_vec[0] == 'A' {
                score += 3;
            }
        // Paper - y, b
        }else if chars_vec[2] == 'Y' {
            // increase score for paper by 2
            score += 2;
            if chars_vec[0] == 'A' {
                // rock x paper = win
                score += 6;
            } else if chars_vec[0] == 'B' {
                // paper x paper = draw
                score += 3;
            }
        // Scissors - z, c
        } else if chars_vec[2] == 'Z' {
            // score for scizzors is +3
            score += 3;
            if chars_vec[0] == 'B' {
                // paper x scissors = win
                score +=6;
            } else if chars_vec[0] == 'C' {
                // scissors x scissors = draw
                score += 3;
            }
        }
    }
    println!("Count: {}", count);
    println!("Total score: {}", score);
}

fn part2() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut score = 0;
    let mut count = 0;
    let lines = contents.lines();
    //iterate through each line
    for l in lines {
        count += 1;
        let chars_vec: Vec<char> = l.chars().collect();
        // Rock - x, a
        if chars_vec[2] == 'X' {
            // x lose
            // increase score for rock by 1
            //score += 1;
            if chars_vec[0] == 'A' {
                score += 3;
            } else if chars_vec[0] == 'B' {
                score += 1;
            } else if chars_vec[0] == 'C' {
                score += 2;
            }
        // Paper - y, b
        }else if chars_vec[2] == 'Y' {
            // y draw
            // increase score for paper by 2
            score += 3;
            if chars_vec[0] == 'A' {
                // rock x paper = win
                score += 1;
            } else if chars_vec[0] == 'B' {
                // paper x paper = draw
                score += 2;
            } else if chars_vec[0] == 'C' {
                score += 3;
            }
        // Scissors - z, c
        } else if chars_vec[2] == 'Z' {
            // z win
            // score for scizzors is +3
            score += 6;
            if chars_vec[0] == 'A' {
                // paper x scissors = win
                score += 2;
            } else if chars_vec[0] == 'B' {
                // scissors x scissors = draw
                score += 3;
            } else if chars_vec[0] == 'C' {
                score += 1;
            }
        }
    }
    println!("Count: {}", count);
    println!("Total score: {}", score);
}
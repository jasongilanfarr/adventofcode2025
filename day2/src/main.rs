
use std::io::stdin;
#[derive(PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
    Init
}
fn main() {
    let mut safe = 0;
    for line in stdin().lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let mut current_level: i32 = iter.next().unwrap().parse().unwrap();
        let mut direction = Direction::Init;
        let mut is_safe = true;

        for next_level in iter {
            let next_level: i32 = next_level.parse().unwrap();
            if direction == Direction::Init {
                if current_level < next_level {
                    direction = Direction::Increasing;
                } else {
                    direction = Direction::Decreasing;
                }
            }
            match direction {
                Direction::Increasing => {
                    if current_level >= next_level {
                        is_safe = false;
                        break;
                    }
                    if next_level - current_level > 3  {
                        is_safe = false;
                        break;
                    }
                }
                Direction::Decreasing => {
                    if current_level <= next_level {
                        is_safe = false;
                        break;
                    }
                    if current_level - next_level > 3 {
                        is_safe = false;
                        break;
                    } 
                }
                _ => {}
            }
            current_level = next_level;
        }
        if is_safe {
            safe += 1;
        }
    }
    println!("Safe Levels: {}", safe);
}

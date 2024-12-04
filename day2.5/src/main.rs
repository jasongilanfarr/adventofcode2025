
use std::io::stdin;
#[derive(PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
    Init
}

fn is_safe(list: &Vec<i32>) -> bool {
    let mut current_level: i32 = list[0];
    let mut direction = Direction::Init;
    let mut is_safe = true;

    for next_level in list.iter().skip(1) {
        if direction == Direction::Init {
            if current_level < *next_level {
                direction = Direction::Increasing;
            } else {
                direction = Direction::Decreasing;
            }
        }
        match direction {
            Direction::Increasing => {
                if current_level >= *next_level {
                    is_safe = false;
                    break;
                }
                if *next_level - current_level > 3  {
                    is_safe = false;
                    break;
                }
            }
            Direction::Decreasing => {
                if current_level <= *next_level {
                    is_safe = false;
                    break;
                }
                if current_level - *next_level > 3 {
                    is_safe = false;
                    break;
                } 
            }
            _ => {}
        }
        current_level = *next_level;
    }
    is_safe
}

fn main() {
    let mut safe = 0;
    for line in stdin().lines() {
        let line = line.unwrap();
        let list = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if is_safe(&list) {
            safe += 1;
        } else {
            for i in 0..list.len() {
                let mut refined = list.clone();
                refined.remove(i);
                if is_safe(&refined) {
                    safe += 1;
                    break;
                }
            }
        }
    }
    println!("Safe Levels: {}", safe);
}

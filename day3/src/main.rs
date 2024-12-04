use regex::Regex;
use std::io;

fn main() {
    let mut sum = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    
    for line in io::stdin().lines() {
        let line = line.unwrap();
        sum += re.captures_iter(&line).map(|cap| cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap()).sum::<i32>();
    }
    println!("{}", sum);
}

use itertools::{repeat_n, Itertools};

fn can_reach(target: &usize, options: &[usize]) -> bool {
    let reachable= repeat_n(['+', '*'].iter(), options.len()).multi_cartesian_product().any(|ops| {
        let mut result = options[0];
        for (op, num) in ops.iter().zip(options.iter().skip(1)) {
            match op {
                '+' => result += num,
                '*' => result *= num,
                _ => unreachable!(),
            }
        }
        result == *target
    });
    reachable
}

fn main() {
    let result: usize = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap().replace(':', " ");
            let split = line.split_whitespace();
            let numbers: Vec<usize> = split.map(|x| x.parse().unwrap()).collect();
            let target = numbers[0];
            let options = numbers[1..].to_vec();
            (target, options)
        })
        .filter(|(target, options)| can_reach(target, options))
        .map(|(target, _)| target)
        .sum();
           
    println!("\n{}", result);
}

use std::collections::HashMap;

fn num_digits(num: usize) -> usize {
    num.ilog10() as usize + 1
}

fn split(num: &usize, cache: &mut HashMap<usize, (usize, usize)>) -> (usize, usize) {
    if cache.contains_key(num) {
        return cache[num];
    }

    let split_point = num_digits(*num);
    let right = num / 10usize.pow(split_point as u32 / 2);
    let left = num % 10usize.pow(split_point as u32 / 2);
    cache.insert(*num, (left, right));
    (left, right)
}


fn main() {
    let mut num_counts = HashMap::<usize, usize>::new();

    std::io::stdin().lines().next().unwrap().unwrap().split_whitespace().for_each(|x| { 
        *num_counts.entry(x.parse::<usize>().unwrap()).or_insert(0) += 1;
    });

    let mut splits = HashMap::<usize, (usize, usize)>::new();

    for _ in 0..75 {
        let old = num_counts.clone();
        num_counts = HashMap::<usize, usize>::new();

        let keys = old.keys();
        for key in keys {
            if old[key] == 0 {
                continue;
            }
            if *key == 0 {
                *num_counts.entry(1).or_insert(0) += old[&0];
            } else if num_digits(*key) % 2 == 0 {
                let (left, right) = split(key, &mut splits);

                *num_counts.entry(left).or_insert(0) += old[key];
                *num_counts.entry(right).or_insert(0) += old[key];
            } else {
                let new = *key * 2024;
                *num_counts.entry(new).or_insert(0) += old[key];
            }
        }
    }
    println!("{}", num_counts.values().sum::<usize>());
}

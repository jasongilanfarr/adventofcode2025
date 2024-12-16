fn num_digits(num: usize) -> usize {
    num.ilog10() as usize + 1
}

fn main() {
    let mut input = std::io::stdin().lines().next().unwrap().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    for _ in 0..25 {
        let mut i = 0;
        while i < input.len() {
            let value = input[i];
            if value == 0 {
                input[i] = 1;
                i += 1;
            } else if num_digits(value) % 2 == 0 {
                let split_point = num_digits(value);
                let right = value / 10usize.pow(split_point as u32 / 2);
                let left = value % 10usize.pow(split_point as u32 / 2);
                input[i] = left;
                input.insert(i, right);
                i += 2;
            } else {
                input[i] = value * 2024;
                i += 1;
            }
            
        }
    }
    println!("{}", input.len());
}

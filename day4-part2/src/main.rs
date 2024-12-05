fn is_valid_move(x: i32, y: i32, arr: &[Vec<char>]) -> bool {
    x >= 0 && y >= 0 && x < arr.len() as i32 && y < arr[0].len() as i32
}

fn main() {

    const SLANTS: [[(i32, i32); 2]; 2] = 
        [[(-1, -1), (1, 1)], [(1, -1), (-1, 1)]];


    let arr: Vec<Vec<char>> = 
        std::io::stdin()
        .lines()
        .map(|line|
            line.unwrap().chars().collect()
        )
        .collect();
    
    let mut count = 0;
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            if arr[i][j] != 'A' {
                continue;
            }

            let mut slant_count = 0;
            for slant in SLANTS {
                if !is_valid_move(i as i32 + slant[0].0, j as i32+ slant[0].1, &arr) ||
                    !is_valid_move(i as i32 + slant[1].0, j as i32 + slant[1].1, &arr) {
                    continue;
                }
                let char1 = arr[(i as i32 + slant[0].0)as usize][(j as i32 + slant[0].1) as usize];
                let char2 = arr[(i as i32 + slant[1].0) as usize][(j as i32 + slant[1].1) as usize];
                if (char1 == 'M' && char2 == 'S') || (char1 == 'S' && char2 == 'M') {
                    slant_count += 1;
                }
            }
            if slant_count == 2 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}

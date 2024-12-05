fn is_valid_move(x: i32, y: i32, arr: &[Vec<char>]) -> bool {
    x >= 0 && y >= 0 && x < arr.len() as i32 && y < arr[0].len() as i32
}

fn main() {
    const EAST:[(i32, i32); 3] = [(0, 1), (0, 2), (0, 3)];
    const WEST: [(i32, i32); 3] = [(0, -1), (0, -2), (0, -3)];
    const NORTH: [(i32, i32); 3] = [(-1, 0), (-2, 0), (-3, 0)];
    const SOUTH: [(i32, i32); 3] = [(1, 0), (2, 0), (3, 0)];
    const NORTHEAST: [(i32, i32); 3] = [(-1, 1), (-2, 2), (-3, 3)];
    const NORTHWEST: [(i32, i32); 3] = [(-1, -1), (-2, -2), (-3, -3)];
    const SOUTHWEST: [(i32, i32); 3] = [(1, -1), (2, -2), (3, -3)];
    const SOUTHEAST: [(i32, i32); 3] = [(1, 1), (2, 2), (3, 3)];
    const MAPPING: [char; 3] = ['M', 'A', 'S'];


    let mut count = 0;
    let arr: Vec<Vec<char>> = 
        std::io::stdin()
        .lines()
        .map(|line|
            line.unwrap().chars().collect()
        )
        .collect();

    for (x_index, x) in arr.iter().enumerate() {
        for (y_index, y) in x.iter().enumerate() {
            if *y != 'X' {
                continue;
            }

            for direction in [EAST, WEST, NORTH, SOUTH, NORTHEAST, NORTHWEST, SOUTHWEST, SOUTHEAST].iter() {
            for (char_index, ch) in MAPPING.iter().enumerate() {
                let (offset_x, offset_y) = direction.get(char_index).unwrap();

                if !is_valid_move(x_index as i32 + offset_x, y_index as i32 + offset_y, &arr) {
                    break;
                }
                let char_at_index = arr[(x_index as i32 + offset_x) as usize][(y_index as i32 + offset_y) as usize];


                if char_at_index == *ch {
                    if *ch == 'S' {
                        count += 1;
                    }
                } else {
                    break;
                }
            }
        }
        }
    }

    println!("{}", count);
}

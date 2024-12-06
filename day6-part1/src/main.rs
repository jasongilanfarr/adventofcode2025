#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Guarded,
    Obstacle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn value(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn move_from(&self, x: usize, y: usize) -> (usize, usize) {
        let (dx, dy) = self.value();
        ((x as i32 + dx) as usize, (y as i32 + dy) as usize)
    }
}

fn move_guard_until_looped(grid: &mut [Vec<Cell>], x: usize, y: usize) {
    let mut direction = Direction::Up;
    let mut x = x;
    let mut y = y;

    loop {
        let (new_x, new_y) = direction.move_from(x, y);
        // guard walked away
        if new_x >= grid.len() || new_y >= grid[0].len() {
            break;
        }
        match grid[new_x][new_y] {
            Cell::Empty | Cell::Guarded => {
                x = new_x;
                y = new_y;
                grid[new_x][new_y] = Cell::Guarded
            }
            Cell::Obstacle => {
                // note, don't change the current location of the guard,
                // just change the direction they are travelling.
                direction = match direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                };
            }
        }
    }
}

fn main() {
    let mut initial_guard_row = 0;
    let mut initial_guard_column = 0;

    let mut arr: Vec<Vec<Cell>> = std::io::stdin()
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.unwrap()
                .chars()
                .enumerate()
                .map(|(y, c)| match c {
                    '^' => {
                        initial_guard_row = x;
                        initial_guard_column = y;
                        Cell::Guarded
                    }
                    '#' => Cell::Obstacle,
                    _ => Cell::Empty,
                })
                .collect()
        })
        .collect();

    move_guard_until_looped(&mut arr, initial_guard_row, initial_guard_column);
    let visited_count: usize = arr
        .iter()
        .map(|column| column.iter().filter(|cell| **cell == Cell::Guarded).count())
        .sum();

    println!("{}", visited_count);
}

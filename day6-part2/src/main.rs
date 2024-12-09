use std::{collections::HashSet, io::empty};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    GuardedUp,
    GuardedLeft,
    GuardedRight,
    GuardedDown,
    Obstacle,
    NewObstacle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    fn turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn guard_cell(&self) -> Cell {
        match self {
            Direction::Up => Cell::GuardedUp,
            Direction::Down => Cell::GuardedDown,
            Direction::Left => Cell::GuardedLeft,
            Direction::Right => Cell::GuardedRight,
        }
    }
}

#[allow(dead_code)]
fn show_grid(grid: &[Vec<Cell>], x: usize, y: usize) {
    println!("\n");
    print!(" ");
    for i in 0..grid[0].len() {
        print!("{}",i);
    }
    println!();

    for (i, row) in grid.iter().enumerate() {
        print!("{}", i);
        for (j, cell) in row.iter().enumerate() {
            if i == x && j == y {
                print!("G");
            } else {
                match cell {
                    Cell::Empty => print!("."),
                    Cell::GuardedUp => print!("↑"),
                    Cell::GuardedLeft => print!("←"),
                    Cell::GuardedRight => print!("→"),
                    Cell::GuardedDown => print!("↓"),
                    Cell::Obstacle => print!("#"),
                    Cell::NewObstacle => print!("O"),
                }
            }
        }
        println!();
    }
}

fn move_guard_until_looped_or_exited(grid: &mut [Vec<Cell>], x: usize, y: usize) -> bool {
    let mut visited: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
    let mut direction = Direction::Up;
    let mut x = x;
    let mut y = y;

    loop {
        let (new_x, new_y) = direction.move_from(x, y);
        // guard walked away
        if new_x >= grid.len() || new_y >= grid[0].len() {
            return false;
        }
        if !visited.insert(((x, y), (new_x, new_y))) {
            return true;
        }

        match grid[new_x][new_y] {
            Cell::Empty => {
                if grid[new_x][new_y] == Cell::Empty {
                    grid[new_x][new_y] = direction.guard_cell();
                }
                x = new_x;
                y = new_y;
            }
            Cell::GuardedDown | Cell::GuardedLeft | Cell::GuardedRight | Cell::GuardedUp => {
                x = new_x;
                y = new_y;
            }
            Cell::Obstacle | Cell::NewObstacle => {
                direction = direction.turn();
            }
        }
    }
}

fn main() {
    let mut initial_guard_row = 0;
    let mut initial_guard_column = 0;

    let arr: Vec<Vec<Cell>> = std::io::stdin()
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
                        Cell::GuardedUp
                    }
                    '#' => Cell::Obstacle,
                    _ => Cell::Empty,
                })
                .collect()
        })
        .collect();

    let mut initial = arr.to_vec();
    move_guard_until_looped_or_exited(&mut initial, initial_guard_row, initial_guard_column);

    let mut empty_cells: Vec<(usize, usize)> = Vec::new();

    for (x, _) in arr.iter().enumerate() {
        for (y, _) in arr[x].iter().enumerate() {
            if (arr[x][y] == Cell::Empty) {
                empty_cells.push((x, y));
            }
        }
    }

    let mut looping_obstacles = 0;

    for new_obstacle in empty_cells {
        let mut grid = arr.to_vec();
        grid[new_obstacle.0][new_obstacle.1] = Cell::NewObstacle;
        if move_guard_until_looped_or_exited(&mut grid, initial_guard_row, initial_guard_column) {
            looping_obstacles += 1;
        }
    }
    println!("Looping obstacles: {}", looping_obstacles);
}

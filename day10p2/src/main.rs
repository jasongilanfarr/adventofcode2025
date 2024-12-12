#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize   
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    fn inbounds(&self, grid: &[Vec<usize>]) -> bool {
        self.x < grid[0].len() && self.y < grid.len()
    }
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

    fn move_from(&self, from: &Point) -> Point {
        let (dx, dy) = self.value();
        Point::new((from.x as i32 + dx) as usize, (from.y as i32 + dy) as usize)
    }
}

fn find_trails(grid: &[Vec<usize>], trailhead: Point) -> usize {
    let mut queue = vec![trailhead];
    let mut num_nines_hit = 0;
    while let Some(current) = queue.pop() {

        let current_value = grid[current.x][current.y];
        [Direction::Up, Direction::Down, Direction::Left, Direction::Right].iter().for_each(|dir| {
            let next = dir.move_from(&current);
        
            if next.inbounds(grid) && grid[next.x][next.y] == current_value + 1 {
                let next_value = grid[next.x][next.y];

                if next_value == 9 {
                    num_nines_hit += 1;
                } else {
                    queue.push(next);
                }
            }
        })
    }
    
    num_nines_hit
}

fn main() {
    let mut trailheads = Vec::<Point>::new();
    let grid: Vec<Vec<usize>> = std::io::stdin().lines().enumerate().map(|(row,line)| {
        line.unwrap().chars().enumerate().map(|(col,cell)| {
            let digit = cell.to_digit(10).unwrap_or(u32::MAX) as usize;
            if digit == 0 {
                trailheads.push(Point::new(row, col));
            }
            digit
        }).collect()
    }).collect();   

    let trail_scores: Vec<usize> = trailheads.iter().map(|trailhead| find_trails(&grid, *trailhead)).collect();

    println!("{:?}", trail_scores.iter().sum::<usize>());
}

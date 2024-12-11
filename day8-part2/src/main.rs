use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn inbounds(&self, max_x: i32, max_y: i32) -> bool {
        self.x >= 0 && self.x < max_x && self.y >= 0 && self.y < max_y
    }

    fn offset(&self, delta_x: i32, delta_y: i32) -> Point {
        Point::new(self.x + delta_x, self.y + delta_y)
    }
}

#[allow(dead_code)]
fn show_grid(grid: &[Vec<char>]) {
    print!(" ");
    for i in 0..grid[0].len() {
        print!("{}", i % 10);
    }
    println!();

    for (x, row) in grid.iter().enumerate() {
        print!("{}", x % 10);
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn main() {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut ants: HashMap<char, Vec<Point>> = HashMap::new();

    std::io::stdin().lines().enumerate().for_each(|(i, line)| {
        let line = line.unwrap();
        let chars = line.chars();
        grid.push(chars.clone().collect());

        chars.enumerate().for_each(|(j, c)| {
            if !(c == '.' || c == '#') {
                ants.entry(c)
                    .or_insert(Vec::new())
                    .push(Point::new(i as i32, j as i32));
            }
        });
    });

    let mut antis: HashSet<Point> = HashSet::new();

    let max_y = grid.len() as i32;
    let max_x = grid[0].len() as i32;

    ants.iter().for_each(|(_, points)| {
        for point1 in points.iter() {
            for point2 in points.iter() {
                if point1 == point2 {
                    continue;
                }
                let delta_x = point1.x - point2.x;
                let delta_y = point1.y - point2.y;

                let mut p = point1.offset(delta_x, delta_y);
                while p.inbounds(max_x, max_y) {
                    grid[p.x as usize][p.y as usize] = 'X';
                    antis.insert(p.clone());
                    p = p.offset(delta_x, delta_y);
                }
                let mut p = point1.offset(-delta_x, -delta_y);
                while p.inbounds(max_x, max_y) {
                    grid[p.x as usize][p.y as usize] = 'X';
                    antis.insert(p.clone());
                    p = p.offset(-delta_x, -delta_y);
                }
            }
        }
    });

    println!("{}", antis.len());
}

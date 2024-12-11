use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

#[allow(dead_code)]
fn show_grid(grid: &[Vec<char>]) {
    print!(" ");
    for i in 0..grid[0].len() {
        print!("{}", i % 10);
    }
    println!();

    for (x,row) in grid.iter().enumerate() {
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
            if c != '.'{
                ants.entry(c)
                    .or_insert(Vec::new())
                    .push(Point::new(i as i32, j as i32));
            }
        });
    });

    let mut antis: HashSet<Point> = HashSet::new();

    let max_y = grid.len() as i32;
    let max_x = grid[0].len() as i32;

    ants
        .iter()
        .for_each(|(_, points)| {
            
            for (i, point1) in points.iter().enumerate() {
                for point2 in points[i+1..].iter() {
                    let anti1 = Point::new(point1.x + point1.x - point2.x, point1.y + point1.y - point2.y);     
                    let anti2 = Point::new(point2.x + point2.x - point1.x, point2.y + point2.y - point1.y);
    
                    if anti1.x >= 0 && anti1.x < max_x && anti1.y >= 0 && anti1.y < max_y {
                        antis.insert(anti1);
                    }
                    if anti2.x >= 0 && anti2.x < max_x && anti2.y >= 0 && anti2.y < max_y {
                        antis.insert(anti2);
                    }
                }
            }
        });

    println!("{}", antis.len());
}

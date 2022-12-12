use std::collections::{HashMap, HashSet};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Cell {
    point: Point,
    char: char,
    parent: Option<Box<Cell>>,
    cost: i32,
}

impl Cell {
    fn new(point: Point, char: char, parent: Option<Box<Cell>>) -> Cell {
        let cost = match &parent {
            Some(parent) => parent.cost + 1,
            None => 0,
        };

        Cell {
            point,
            char,
            parent,
            cost,
        }
    }
}

fn index_to_position(index: usize, width: usize) -> Point {
    Point {
        x: index % width,
        y: index / width,
    }
}

fn position_to_index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

fn get_neighbor(
    map: &Vec<char>,
    width: usize,
    height: usize,
    cell: &Cell,
    direction: Direction,
    can_traverse: fn(char, char) -> bool,
) -> Option<Cell> {
    let index = match direction {
        Direction::Up => {
            if cell.point.y > 0 {
                Some(position_to_index(cell.point.x, cell.point.y - 1, width))
            } else {
                None
            }
        }
        Direction::Down => {
            if cell.point.y < height - 1 {
                Some(position_to_index(cell.point.x, cell.point.y + 1, width))
            } else {
                None
            }
        }
        Direction::Left => {
            if cell.point.x > 0 {
                Some(position_to_index(cell.point.x - 1, cell.point.y, width))
            } else {
                None
            }
        }
        Direction::Right => {
            if cell.point.x < width - 1 {
                Some(position_to_index(cell.point.x + 1, cell.point.y, width))
            } else {
                None
            }
        }
    };

    match index {
        Some(index) => {
            let char = map[index];
            if can_traverse(cell.char, char) {
                Some(Cell::new(
                    index_to_position(index, width),
                    char,
                    Some(Box::new(cell.clone())),
                ))
            } else {
                None
            }
        }
        None => None,
    }
}

#[derive(Debug)]
struct Neighbors {
    up: Option<Cell>,
    down: Option<Cell>,
    left: Option<Cell>,
    right: Option<Cell>,
}

impl Neighbors {
    fn into_vec(self) -> Vec<Cell> {
        let mut vec: Vec<Cell> = Vec::new();

        if let Some(up) = self.up {
            vec.push(up);
        }

        if let Some(down) = self.down {
            vec.push(down);
        }

        if let Some(left) = self.left {
            vec.push(left);
        }

        if let Some(right) = self.right {
            vec.push(right);
        }

        vec
    }
}

fn get_neighbors(
    map: &Vec<char>,
    width: usize,
    height: usize,
    cell: &Cell,
    can_traverse: fn(char, char) -> bool,
) -> Neighbors {
    Neighbors {
        up: get_neighbor(map, width, height, cell, Direction::Up, can_traverse),
        down: get_neighbor(map, width, height, cell, Direction::Down, can_traverse),
        left: get_neighbor(map, width, height, cell, Direction::Left, can_traverse),
        right: get_neighbor(map, width, height, cell, Direction::Right, can_traverse),
    }
}

fn manhattan_distance(a: &Point, b: &Point) -> i32 {
    (a.x as i32 - b.x as i32).abs() + (a.y as i32 - b.y as i32).abs()
}

fn euclidean_distance(a: &Point, b: &Point) -> f32 {
    let x = (a.x as f32 - b.x as f32).powf(2.0);
    let y = (a.y as f32 - b.y as f32).powf(2.0);
    (x + y).sqrt()
}

fn part1(file_contents: &str) {
    const WIDTH: usize = 95;
    const HEIGHT: usize = 41;

    let map: Vec<_> = file_contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .flatten()
        .collect();

    let start_pos = index_to_position(map.iter().position(|c| *c == 'S').unwrap(), WIDTH);
    let end_pos = index_to_position(map.iter().position(|c| *c == 'E').unwrap(), WIDTH);

    let start_cell = Cell::new(start_pos, 'a', None);

    let mut cache = HashMap::new();
    let mut cached_calculate_distance = |a: &Point, b: &Point| {
        if let Some(distance) = cache.get(&(a.x, a.y, b.x, b.y)) {
            *distance
        } else {
            // let distance = manhattan_distance(a, b);
            let distance = euclidean_distance(a, b);
            cache.insert((a.x, a.y, b.x, b.y), distance);
            distance
        }
    };

    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: Vec<Cell> = Vec::new();
    let mut iters = 0;

    queue.push(start_cell);

    // A-star hopefully =)

    while !queue.is_empty() {
        iters += 1;
        let cell = queue.pop().unwrap();

        if cell.point == end_pos {
            let mut path: Vec<Point> = Vec::new();
            let mut current_cell = Some(cell);
            while let Some(cell) = current_cell {
                path.push(cell.point);
                current_cell = cell.parent.map(|c| *c)
            }

            path.reverse();

            let mut map = map.clone();

            for index in 0..path.len() - 1 {
                let current = path[index];
                let next = path[index + 1];
                let map_index = position_to_index(current.x, current.y, WIDTH);

                match (
                    next.x as i32 - current.x as i32,
                    next.y as i32 - current.y as i32,
                ) {
                    (1, 0) => map[map_index] = '>',
                    (-1, 0) => map[map_index] = '<',
                    (0, 1) => map[map_index] = 'v',
                    (0, -1) => map[map_index] = '^',
                    _ => (),
                };
            }

            // print map line by line
            for y in 0..HEIGHT {
                let line = map
                    .iter()
                    .skip(y * WIDTH)
                    .take(WIDTH)
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join("");

                println!("{}", line);
            }

            println!("Path length: {}", path.len() - 1);
            println!("Iterations: {}", iters);

            break;
        }

        match visited.get(&cell.point) {
            Some(_) => {
                continue;
            }
            None => visited.insert(cell.point),
        };

        let neighbors = get_neighbors(&map, WIDTH, HEIGHT, &cell, |a, b| {
            (b as i32 - a as i32) <= 1
        });

        for neighbor in neighbors.into_vec() {
            let insertion_index = queue.partition_point(|c| {
                cached_calculate_distance(&c.point, &end_pos) + c.cost as f32
                    > cached_calculate_distance(&neighbor.point, &end_pos) + neighbor.cost as f32
            });
            queue.insert(insertion_index, neighbor);
        }
    }
}

fn part2(file_contents: &str) {
    const WIDTH: usize = 95;
    const HEIGHT: usize = 41;

    let map: Vec<_> = file_contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .flatten()
        .collect();

    let start_pos = index_to_position(map.iter().position(|c| *c == 'E').unwrap(), WIDTH);
    let start_cell = Cell::new(start_pos, 'z', None);

    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: Vec<Cell> = Vec::new();

    queue.push(start_cell);

    // breadth-first search

    while !queue.is_empty() {
        let cell = queue.pop().unwrap();

        let cell_char = map[position_to_index(cell.point.x, cell.point.y, WIDTH)];

        if cell_char == 'a' {
            println!("Found a: {:?}", cell.point);
            println!("Cost: {}", cell.cost);

            break;
        }

        match visited.get(&cell.point) {
            Some(_) => {
                continue;
            }
            None => visited.insert(cell.point),
        };

        let neighbors = get_neighbors(&map, WIDTH, HEIGHT, &cell, |a, b| {
            (a as i32 - b as i32) <= 1
        });

        for neighbor in neighbors.into_vec() {
            let insertion_index = queue.partition_point(|c| c.cost > neighbor.cost);
            queue.insert(insertion_index, neighbor);
        }
    }
}

fn main() {
    let file_contents =
        std::fs::read_to_string("./input.txt").expect("Something went wrong reading the file");

    part1(&file_contents);
    part2(&file_contents);
}

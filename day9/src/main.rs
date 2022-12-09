use std::collections::HashSet;

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn follow_point(&mut self, other: &Point) {
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();

        if x_diff + y_diff > 2 {
            self.x += -(self.x - other.x).signum();
            self.y += -(self.y - other.y).signum();
            return;
        }

        if x_diff > 1 {
            self.x += -(self.x - other.x).signum();
        } else if y_diff > 1 {
            self.y += -(self.y - other.y).signum();
        }
    }
}

fn main() {
    let file_contents =
        std::fs::read_to_string("./input.txt").expect("Something went wrong reading the file");

    // part 1 & 2

    let mut knots: Vec<Point> = vec![Point { x: 0, y: 0 }; 10]; // change to 2 for part 1
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    for line in file_contents.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let dir = parts[0];
        let steps = parts[1].parse::<i32>().unwrap();

        for _ in 0..steps {
            for index in 0..knots.len() {
                if index == 0 {
                    match dir {
                        "U" => knots[index].y += 1,
                        "D" => knots[index].y -= 1,
                        "L" => knots[index].x -= 1,
                        "R" => knots[index].x += 1,
                        _ => panic!("Unknown direction"),
                    }
                    continue;
                }

                let prev = knots[index - 1].clone();
                knots[index].follow_point(&prev);
            }

            visited.insert((knots.last().unwrap().x, knots.last().unwrap().y));
        }
    }

    println!("Visited {} points", visited.len());
}

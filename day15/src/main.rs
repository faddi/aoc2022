use std::{collections::HashSet, hash::Hash, ops::RangeInclusive};

type Num = i128;

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Point {
    x: Num,
    y: Num,
}

fn manhattan_distance(p1: &Point, p2: &Point) -> Num {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn merge_ranges(ranges: Vec<RangeInclusive<Num>>) -> Vec<RangeInclusive<Num>> {
    let mut r = ranges.clone();

    let mut out = Vec::new();

    while let Some(current) = r.pop() {
        let mut merged = false;
        for i in 0..r.len() {
            let other = &r[i];

            if current.start() <= other.end() && current.end() >= other.start() {
                let new_range: RangeInclusive<Num> =
                    *other.start().min(current.start())..=*other.end().max(current.end());
                r[i] = new_range;
                merged = true;
                break;
            }
        }

        if !merged {
            out.push(current);
        }
    }

    out
}

fn calculate_ranges_for_y(
    sensor: &Vec<Point>,
    beacons: &Vec<Point>,
    y: Num,
) -> Vec<RangeInclusive<Num>> {
    let mut ranges: Vec<RangeInclusive<Num>> = Vec::new();

    for (sensor, beacon) in sensor.iter().zip(beacons.iter()) {
        let r = manhattan_distance(sensor, beacon);

        let bottom = Point {
            x: sensor.x,
            y: sensor.y - r,
        };

        let top = Point {
            x: sensor.x,
            y: sensor.y + r,
        };

        // check how many point of and calculate width at offset from top or bottom
        if y >= bottom.y && y <= top.y {
            if y == bottom.y {
                ranges.push(bottom.x..=bottom.x);
            } else if y == top.y {
                ranges.push(top.x..=top.x)
            } else {
                let left = Point {
                    x: sensor.x - (r - (y - sensor.y).abs()),
                    y,
                };

                let right = Point {
                    x: sensor.x + (r - (y - sensor.y).abs()),
                    y,
                };

                ranges.push(left.x..=right.x);
            }
        }
    }

    ranges
}

fn main() {
    let file_contents =
        std::fs::read_to_string("./input.txt").expect("Something went wrong reading the file");

    let (sensor, beacons): (Vec<Point>, Vec<Point>) = file_contents
        .lines()
        .map(|line| {
            let mut parts = line.split("=");

            // println!("{:?}", parts.clone().collect::<Vec<&str>>());

            let sensor_x = parts
                .nth(1)
                .unwrap()
                .split(",")
                .nth(0)
                .unwrap()
                .parse::<Num>()
                .unwrap();

            let sensor_y = parts
                .next()
                .unwrap()
                .split(":")
                .nth(0)
                .unwrap()
                .parse::<Num>()
                .unwrap();

            let beacon_x = parts
                .next()
                .unwrap()
                .split(",")
                .nth(0)
                .unwrap()
                .parse::<Num>()
                .unwrap();

            let beacon_y = parts.next().unwrap().parse::<Num>().unwrap();

            (
                Point {
                    x: sensor_x,
                    y: sensor_y,
                },
                Point {
                    x: beacon_x,
                    y: beacon_y,
                },
            )
        })
        .fold(
            (Vec::new(), Vec::new()),
            |(mut sensors, mut beacons), (sensor, beacon)| {
                sensors.push(sensor);
                beacons.push(beacon);
                (sensors, beacons)
            },
        );

    let test_line_y = 2000000;

    let blocked_positions: HashSet<Point> =
        HashSet::from_iter(beacons.iter().cloned().filter(|p| p.y == test_line_y));

    let mut ranges: Vec<RangeInclusive<Num>> =
        calculate_ranges_for_y(&sensor, &beacons, test_line_y);

    // merge ranges

    ranges.sort_by(|a, b| a.start().cmp(&b.start()));

    let merged_ranges = merge_ranges(ranges);

    let visited_count = merged_ranges.iter().fold(0, |count, range| {
        let blocked_in_range = blocked_positions
            .iter()
            .filter(|p| range.contains(&p.x))
            .count();

        count + range.end() - range.start() + 1 - blocked_in_range as Num
    });

    println!("part 1: {:?}", visited_count);

    // part 2

    for y in 0..=4000000 {
        let ranges = calculate_ranges_for_y(&sensor, &beacons, y);

        let mut merged_ranges = merge_ranges(ranges);

        if merged_ranges.len() == 2 {
            println!("y: {:?}, ranges: {:?}", y, merged_ranges);

            merged_ranges.sort_by(|a, b| a.start().cmp(&b.start()));

            let position = merged_ranges[0].end() + 1;

            println!("position : ({:?}, {:?})", position, y);

            println!("part 2: {:?}", position * 4000000 + y);
            break;
        }
    }
}

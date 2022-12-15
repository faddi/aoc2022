#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn print_matrix(matrix_in: &Vec<Vec<char>>, path: Option<&Vec<Point>>) {
    let mut matrix = matrix_in.clone();

    if path.is_some() {
        let path = path.unwrap();
        for point in path {
            matrix[point.y as usize][point.x as usize] = 'x';
        }
    }

    let (min_x, min_y, max_x, max_y) = matrix
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == '#' || **c == 'o' || **c == 'x')
                .map(move |(x, _)| (x, y))
        })
        .fold((600, 600, 0, 0), |(min_x, min_y, max_x, max_y), (x, y)| {
            (
                std::cmp::min(min_x, x),
                std::cmp::min(min_y, y),
                std::cmp::max(max_x, x),
                std::cmp::max(max_y, y),
            )
        });

    // print matrix with min_x, min_y, max_x, max_y
    let output = matrix
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                // .filter(|(x, _)| x >= &min_x && x <= &max_x && y >= min_y && y <= max_y)
                .filter(|(x, _)| x >= &min_x && x <= &max_x)
                .map(|(_, c)| *c)
                .collect::<String>()
        })
        .filter(|l| !l.is_empty())
        .collect::<Vec<String>>()
        .join("\n");

    std::fs::write("out.txt", output).expect("Unable to write file");

    // println!("{} {} {} {}", min_x, min_y, max_x, max_y);
    // println!("{}", output);
}

fn main() {
    let file_contents =
        std::fs::read_to_string("./input.txt").expect("Something went wrong reading the file");

    let mut matrix_base = vec![vec!['.'; 1200]; 1200];

    file_contents.lines().for_each(|line| {
        let points: Vec<Point> = line
            .split(" -> ")
            .map(|s| {
                let coords: Vec<i32> = s.split(",").map(|s| s.parse().unwrap()).collect();
                Point {
                    x: coords[0],
                    y: coords[1],
                }
            })
            .collect();

        for point_index in 0..points.len() - 1 {
            let current_point = &points[point_index];
            let next_point = &points[point_index + 1];

            let x_diff = next_point.x - current_point.x;
            let y_diff = next_point.y - current_point.y;

            if x_diff == 0 {
                let iter = if current_point.y < next_point.y {
                    current_point.y..=next_point.y
                } else {
                    next_point.y..=current_point.y
                };

                for y in iter {
                    matrix_base[y as usize][current_point.x as usize] = '#';
                }
            } else if y_diff == 0 {
                let iter = if current_point.x < next_point.x {
                    current_point.x..=next_point.x
                } else {
                    next_point.x..=current_point.x
                };

                for x in iter {
                    matrix_base[current_point.y as usize][x as usize] = '#';
                }
            }
        }
    });

    let mut matrix_p1 = matrix_base.clone();

    let starting_pos = Point { x: 500, y: 0 };

    let mut fallthrough = false;
    while !fallthrough {
        let mut path: Vec<Point> = Vec::new();
        let mut current_pos = starting_pos;
        loop {
            path.push(current_pos);

            // if at last line, break
            if current_pos.y == matrix_p1[0].len() as i32 - 1 {
                fallthrough = true;
                break;
            }

            // try to move down
            if matrix_p1[current_pos.y as usize + 1][current_pos.x as usize] == '.' {
                current_pos.y += 1;
                continue;
            }

            // try to move left diagonally
            if matrix_p1[current_pos.y as usize + 1][current_pos.x as usize - 1] == '.' {
                current_pos.x -= 1;
                current_pos.y += 1;
                continue;
            }

            // try to move right diagonally
            if matrix_p1[current_pos.y as usize + 1][current_pos.x as usize + 1] == '.' {
                current_pos.x += 1;
                current_pos.y += 1;
                continue;
            }

            // set current pos to o
            matrix_p1[current_pos.y as usize][current_pos.x as usize] = 'o';
            break;
        }

        // print_matrix(&matrix, Some(&path));
    }
    print_matrix(&matrix_p1, None);

    // count o's
    let count = matrix_p1
        .iter()
        .flat_map(|row| row.iter())
        .filter(|c| **c == 'o')
        .count();

    println!("part1: {}", count);

    // part 2

    let mut matrix_p2 = matrix_base.clone();

    // highest y
    let lowest = matrix_p2
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().any(|c| *c == '#'))
        .map(|(y, _)| y)
        .max()
        .unwrap();

    // set all y's above lowest to #

    matrix_p2[lowest + 2].fill('#');

    fallthrough = false;

    while !fallthrough {
        let mut path: Vec<Point> = Vec::new();
        let mut current_pos = starting_pos;
        loop {
            path.push(current_pos);

            // try to move down
            if matrix_p2[current_pos.y as usize + 1][current_pos.x as usize] == '.' {
                current_pos.y += 1;
                continue;
            }

            // try to move left diagonally
            if matrix_p2[current_pos.y as usize + 1][current_pos.x as usize - 1] == '.' {
                current_pos.x -= 1;
                current_pos.y += 1;
                continue;
            }

            // try to move right diagonally
            if matrix_p2[current_pos.y as usize + 1][current_pos.x as usize + 1] == '.' {
                current_pos.x += 1;
                current_pos.y += 1;
                continue;
            }

            if matrix_p2[current_pos.y as usize][current_pos.x as usize] == 'o' {
                fallthrough = true;
                print_matrix(&matrix_p2, Some(&path));
                break;
            }

            // set current pos to o
            matrix_p2[current_pos.y as usize][current_pos.x as usize] = 'o';
            break;
        }
    }

    // count o's
    let count = matrix_p2
        .iter()
        .flat_map(|row| row.iter())
        .filter(|c| **c == 'o')
        .count();

    println!("part2: {}", count);
}

use std::collections::HashSet;

fn main() {
    let file_contents = std::fs::read_to_string("./input.txt").expect("Could not read file");

    let points: Vec<Vec<i32>> = file_contents
        .lines()
        .map(|l| {
            return l
                .split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
        })
        .collect();

    let mut exposed_sides = 0;

    for i in 0..points.len() {
        exposed_sides += 6;
        for j in 0..points.len() {
            if i == j {
                continue;
            }

            let point = &points[i];
            let other_point = &points[j];

            let x_diff = point[0] - other_point[0];
            let y_diff = point[1] - other_point[1];
            let z_diff = point[2] - other_point[2];

            if x_diff.abs() == 1 && y_diff == 0 && z_diff == 0 {
                exposed_sides -= 1;
            }

            if y_diff.abs() == 1 && x_diff == 0 && z_diff == 0 {
                exposed_sides -= 1;
            }

            if z_diff.abs() == 1 && x_diff == 0 && y_diff == 0 {
                exposed_sides -= 1;
            }
        }
    }

    println!("Part 1: {}", exposed_sides);

    // part 2
    let (min_x, max_x, min_y, max_y, min_z, max_z) = points.iter().fold(
        (
            points[0][0],
            points[0][0],
            points[0][1],
            points[0][1],
            points[0][2],
            points[0][2],
        ),
        |(min_x, max_x, min_y, max_y, min_z, max_z), point| {
            (
                std::cmp::min(min_x, point[0]),
                std::cmp::max(max_x, point[0]),
                std::cmp::min(min_y, point[1]),
                std::cmp::max(max_y, point[1]),
                std::cmp::min(min_z, point[2]),
                std::cmp::max(max_z, point[2]),
            )
        },
    );

    let is_point_at_or_outside_boundary = |point: &Vec<i32>| -> bool {
        point[0] <= min_x
            || point[0] >= max_x
            || point[1] <= min_y
            || point[1] >= max_y
            || point[2] <= min_z
            || point[2] >= max_z
    };

    fn is_point_contained(
        point: &Vec<i32>,
        all_points: &HashSet<Vec<i32>>,
        is_point_at_or_outside_boundary: &dyn Fn(&Vec<i32>) -> bool,
        visited_in_path: &mut HashSet<Vec<i32>>,
    ) -> bool {
        if is_point_at_or_outside_boundary(point) {
            return false;
        }

        visited_in_path.insert(point.to_vec());

        let directions = vec![
            vec![1, 0, 0],
            vec![-1, 0, 0],
            vec![0, 1, 0],
            vec![0, -1, 0],
            vec![0, 0, 1],
            vec![0, 0, -1],
        ];

        let mut contained = true;

        for direction in directions {
            let mut new_point = point.to_vec();
            new_point[0] += direction[0];
            new_point[1] += direction[1];
            new_point[2] += direction[2];

            if visited_in_path.contains(&new_point) {
                continue;
            }

            if all_points.contains(&new_point) {
                continue;
            }

            let c = is_point_contained(
                &new_point,
                all_points,
                is_point_at_or_outside_boundary,
                visited_in_path,
            );

            if !c {
                contained = false;
                break;
            }
        }

        if contained {
            return true;
        } else {
            return false;
        }
    }

    // let mut contained_set = HashSet::new();
    // let mut uncontained_set = HashSet::new();
    let all_points_set: &HashSet<Vec<i32>> = &points.iter().cloned().collect();

    exposed_sides = 0;
    let directions = vec![
        vec![1, 0, 0],
        vec![-1, 0, 0],
        vec![0, 1, 0],
        vec![0, -1, 0],
        vec![0, 0, 1],
        vec![0, 0, -1],
    ];

    for i in 0..points.len() {
        for direction in &directions {
            let mut new_point = points[i].to_vec();
            new_point[0] += direction[0];
            new_point[1] += direction[1];
            new_point[2] += direction[2];

            if all_points_set.contains(&new_point) {
                continue;
            }

            if !is_point_contained(
                &new_point,
                all_points_set,
                &is_point_at_or_outside_boundary,
                &mut HashSet::new(),
            ) {
                exposed_sides += 1;
            }
        }
    }

    println!("Part 2: {}", exposed_sides);
}

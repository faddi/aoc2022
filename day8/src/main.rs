use std::collections::HashSet;

fn main() {
    let file_contents = std::fs::read_to_string("./input.txt").expect("Could not read file");

    const WIDTH: usize = 99;
    const HEIGHT: usize = 99;

    let data: Vec<u32> = file_contents
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
        .flatten()
        .collect();

    let get_data_at = |x: usize, y: usize| -> u32 { data[y * WIDTH + x] };

    let for_each_cell_in_direction =
        |x: usize,
         y: usize,
         dx: i32,
         dy: i32,
         callback: &mut dyn FnMut(u32, (usize, usize)) -> bool| {
            let mut x = x as i32;
            let mut y = y as i32;
            while x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
                if !callback(
                    get_data_at(x as usize, y as usize),
                    (x as usize, y as usize),
                ) {
                    return;
                }
                x += dx;
                y += dy;
            }
        };

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut check_dir = |x: usize, y: usize, dx: i32, dy: i32| {
        let mut h = get_data_at(x, y);
        visited.insert((x, y));

        for_each_cell_in_direction(x, y, dx, dy, &mut |cell, position| {
            if cell > h {
                h = cell;
                visited.insert(position);
            }

            h < 9
        });
    };

    for x in 0..WIDTH {
        // col
        check_dir(x, 0, 0, 1);

        // col reverse
        check_dir(x, HEIGHT - 1, 0, -1);

        // row
        check_dir(0, x, 1, 0);

        // row reverse
        check_dir(WIDTH - 1, x, -1, 0);
    }

    println!("Visited: {}", visited.len());

    // part 2

    let check_dir2 = |x: usize, y: usize, dx: i32, dy: i32| {
        let h = get_data_at(x, y);
        let mut counter = 0;

        for_each_cell_in_direction(
            (x as i32 + dx) as usize,
            (y as i32 + dy) as usize,
            dx,
            dy,
            &mut |cell, _position| {
                counter += 1;

                if cell >= h {
                    return false;
                }

                true
            },
        );
        counter
    };

    let mut score = 1;

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let col = check_dir2(x, y, 0, 1);
            let row = check_dir2(x, y, 1, 0);
            let col_rev = check_dir2(x, y, 0, -1);
            let row_rev = check_dir2(x, y, -1, 0);

            let tree_score = col * row * col_rev * row_rev;

            if tree_score > score {
                score = tree_score;
            }
        }
    }

    println!("Score: {}", score);

    // println!("{:?}", data);
}

use std::{collections::VecDeque, fs};

fn main() {
    let file_contents = fs::read_to_string("./input.txt").unwrap();

    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); 9];
    let mut counter = 0;

    for line in file_contents.lines() {
        counter += 1;
        if line.len() > 0 {
            line.char_indices().skip(1).step_by(4).for_each(|c| {
                if c.1 == ' ' || c.1 == '1' {
                    return;
                }

                let index = (c.0 - 1) / 4;
                stacks[index].push_front(c.1);
            });
        } else {
            break;
        }
    }

    let mut part2 = stacks.clone();

    for line in file_contents.lines().skip(counter) {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let count = parts[1].parse::<u32>().unwrap();
        let from = parts[3].parse::<u32>().unwrap() - 1;
        let to = parts[5].parse::<u32>().unwrap() - 1;

        let len = stacks[from as usize].len();
        let mut moved = stacks[from as usize]
            .split_off(len - count as usize)
            .into_iter()
            .rev()
            .collect();

        stacks[to as usize].append(&mut moved);
    }

    println!("Part 1: ");

    stacks.iter().for_each(|f| {
        print!("{}", f.back().unwrap());
    });

    // part2

    for line in file_contents.lines().skip(counter) {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let count = parts[1].parse::<u32>().unwrap();
        let from = parts[3].parse::<u32>().unwrap() - 1;
        let to = parts[5].parse::<u32>().unwrap() - 1;

        let len = part2[from as usize].len();
        let mut moved = part2[from as usize].split_off(len - count as usize);

        part2[to as usize].append(&mut moved);
    }

    println!("\nPart 2: ");
    part2.iter().for_each(|f| {
        print!("{}", f.back().unwrap());
    });
}

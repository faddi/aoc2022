use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

const LOWER_A_CODE: i32 = 'a' as i32;
const LOWER_Z_CODE: i32 = 'z' as i32;
const UPPER_A_CODE: i32 = 'A' as i32;
const ALPHABET_LENGTH: i32 = LOWER_Z_CODE - LOWER_A_CODE + 1;

fn part1(input: &str) {
    let costs = input.lines().map(|line| {
        let parts = line.split_at(line.len() / 2);

        let first: HashSet<_> = parts.0.chars().collect();

        let dupes: HashSet<_> = parts.1.chars().filter(|c| first.contains(c)).collect();

        let values = dupes.into_iter().map(|d| match d {
            'a'..='z' => d as i32 - LOWER_A_CODE + 1,
            'A'..='Z' => d as i32 - UPPER_A_CODE + 1 + ALPHABET_LENGTH,
            _ => 0,
        });

        return values.sum::<i32>();
    });

    println!("Sum: {}", costs.sum::<i32>());
}

fn part2(input: &str) {
    let costs: Vec<i32> = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|lines_chunk| {
            let lines = lines_chunk.collect::<Vec<_>>();

            let chars_common_to_all: HashSet<_> = lines[0]
                .chars()
                .filter(|c| lines[1].contains(*c) && lines[2].contains(*c))
                .collect();

            let values = chars_common_to_all.into_iter().map(|d| match d {
                'a'..='z' => d as i32 - LOWER_A_CODE + 1,
                'A'..='Z' => d as i32 - UPPER_A_CODE + 1 + ALPHABET_LENGTH,
                _ => 0,
            });

            return values.sum::<i32>();
        })
        .collect();

    println!("Sum: {}", costs.into_iter().sum::<i32>());
}

fn main() {
    let file_contents =
        fs::read_to_string("./input.txt").expect("Something went wrong reading the file");

    part1(&file_contents);
    part2(&file_contents);
}

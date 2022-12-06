use std::{collections::HashSet, fs};

fn part1(file_contents: &str) {
    let mut list: Vec<_> = file_contents[0..4].chars().collect();

    let chars: Vec<char> = file_contents.chars().collect();

    for (index, char) in file_contents.chars().skip(4).enumerate() {
        list.remove(0);
        list.push(char);

        let s: HashSet<_> = list.iter().collect();

        if s.len() == 4 {
            println!("Found at index: {}", index);
            println!("Chars processed: {}", index + 4 + 1);
            println!(
                "Last 4 chars: {:?}",
                chars[index..index + 4].iter().collect::<String>()
            );
            break;
        }
    }
}

fn part2(file_contents: &str) {
    let mut list: Vec<_> = file_contents[0..14].chars().collect();

    let chars: Vec<char> = file_contents.chars().collect();

    for (index, char) in file_contents.chars().skip(14).enumerate() {
        list.remove(0);
        list.push(char);

        let s: HashSet<_> = list.iter().collect();

        if s.len() == 14 {
            println!("Found at index: {}", index);
            println!("Chars processed: {}", index + 14 + 1);
            println!(
                "Last 14 chars: {:?}",
                chars[index..index + 14].iter().collect::<String>()
            );
            break;
        }
    }
}

fn main() {
    let file_contents = fs::read_to_string("./input.txt").unwrap();

    part1(&file_contents);
    part2(&file_contents);
}

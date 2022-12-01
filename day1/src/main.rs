use std::fs;

fn main() {
    let file_contents =
        fs::read_to_string("./input.txt").expect("Something went wrong reading the file");

    let mut total: u32 = 0;
    let mut list = Vec::new();

    for line in file_contents.lines() {
        if line == "" {
            list.push(total);
            total = 0;
        } else {
            total += line.parse::<u32>().unwrap();
        }
    }

    list.sort_by(|a, b| b.cmp(a));

    println!("max: {:?}", list[0]);
    println!("sum of top 3: {:?}", list[0] + list[1] + list[2]);
}

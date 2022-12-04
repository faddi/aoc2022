use std::fs;
use std::ops::RangeInclusive;

trait ContainsRangeInclusive<T> {
    fn contains_range(&self, range: &RangeInclusive<T>) -> bool;
}

// contains range for RangeInclusive
impl<T> ContainsRangeInclusive<T> for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, range: &RangeInclusive<T>) -> bool {
        self.contains(range.start()) && self.contains(range.end())
    }
}

trait OverlapCountRangeInclusive {
    fn overlap_count(&self, range: &RangeInclusive<u32>) -> u32;
}

// overlap count for RangeInclusive
impl OverlapCountRangeInclusive for RangeInclusive<u32> {
    fn overlap_count(&self, range: &RangeInclusive<u32>) -> u32 {
        let lower = self.start().max(range.start());
        let upper = self.end().min(range.end());

        return if lower <= upper { upper - lower + 1 } else { 0 };
    }
}

fn part1(file_contents: &str) {
    let containing_count = file_contents
        .lines()
        .map(|line| {
            let ranges: Vec<std::ops::RangeInclusive<u32>> = line
                .split(",")
                .map(|segment| {
                    let endpoints: Vec<u32> = segment
                        .split('-')
                        .map(|num| {
                            return num.parse::<u32>().unwrap();
                        })
                        .collect();

                    return std::ops::RangeInclusive::new(endpoints[0], endpoints[1]);
                })
                .collect();

            return ranges[0].contains_range(&ranges[1]) || ranges[1].contains_range(&ranges[0]);
        })
        .filter(|contains| *contains)
        .count();

    println!("containing count: {}", containing_count);
}

fn part2(file_contents: &str) {
    let sum: u32 = file_contents
        .lines()
        .map(|line| {
            let ranges: Vec<std::ops::RangeInclusive<u32>> = line
                .split(",")
                .map(|segment| {
                    let endpoints: Vec<u32> = segment
                        .split('-')
                        .map(|num| {
                            return num.parse::<u32>().unwrap();
                        })
                        .collect();

                    return std::ops::RangeInclusive::new(endpoints[0], endpoints[1]);
                })
                .collect();

            return ranges[0].overlap_count(&ranges[1]);
        })
        .filter(|a| *a > 0)
        .count() as u32;

    println!("sum: {}", sum);
}

fn main() {
    let file_contents = fs::read_to_string("./input.txt").unwrap();
    part1(&file_contents);
    part2(&file_contents);
}

use std::collections::VecDeque;

#[derive(Copy, Clone, Debug)]
enum Operation {
    Add(i64),
    Multiply(i64),
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<i64>,
    operation: Operation,
    test: i64,
    true_branch: i64,
    false_branch: i64,
}

fn main() {
    let file_contents =
        std::fs::read_to_string("./input.txt").expect("Something went wrong reading the file");

    let mut line_iter = file_contents.lines();

    let mut monkeys: Vec<Monkey> = Vec::new();

    // parse

    loop {
        let id_line = line_iter.next();

        if id_line.is_none() {
            break;
        }

        let _id = id_line
            .unwrap()
            .replace("Monkey ", "")
            .replace(":", "")
            .trim()
            .parse::<i64>()
            .unwrap();

        let items: VecDeque<_> = line_iter
            .next()
            .unwrap()
            .replace("  Starting items:", "")
            .split(",")
            .map(|r| r.trim().parse::<i64>().unwrap())
            .collect();

        let op_parts: Vec<String> = line_iter
            .next()
            .unwrap()
            .replace("  Operation: new = old ", "")
            .split(" ")
            .map(|s| s.to_string())
            .collect();

        let op = match op_parts[0].as_str() {
            "+" => Operation::Add(op_parts[1].trim().parse::<i64>().unwrap()),
            "*" => {
                if op_parts[1] == "old" {
                    Operation::Square
                } else {
                    Operation::Multiply(op_parts[1].trim().parse::<i64>().unwrap())
                }
            }
            _ => panic!("Unknown operation"),
        };

        let test = line_iter
            .next()
            .unwrap()
            .replace("  Test: divisible by ", "")
            .parse::<i64>()
            .unwrap();

        let true_branch = line_iter
            .next()
            .unwrap()
            .replace("    If true: throw to monkey ", "")
            .parse::<i64>()
            .unwrap();

        let false_branch = line_iter
            .next()
            .unwrap()
            .replace("    If false: throw to monkey ", "")
            .parse::<i64>()
            .unwrap();

        let _ = line_iter.next(); // consume the empty line

        monkeys.push(Monkey {
            items,
            operation: op,
            test,
            true_branch,
            false_branch,
        });
    }

    let mut monkeys2 = monkeys.clone();

    let mut inspects = vec![0; monkeys.len()];

    fn round_1(monkeys: &mut Vec<Monkey>, inspects: &mut Vec<i32>) {
        // run round
        for monkey_index in 0..monkeys.len() {
            let monkey = monkeys[monkey_index].clone();

            for item in monkey.items.iter() {
                inspects[monkey_index] += 1;
                let new_item = match monkey.operation {
                    Operation::Add(n) => item + n,
                    Operation::Multiply(n) => item * n,
                    Operation::Square => item * item,
                };

                let n = new_item / 3;

                if n % monkey.test == 0 {
                    // true branch

                    monkeys[monkey.true_branch as usize].items.push_back(n);
                } else {
                    // false branch
                    monkeys[monkey.false_branch as usize].items.push_back(n);
                }
            }

            monkeys[monkey_index].items.clear();
        }
    }

    for _ in 0..20 {
        round_1(&mut monkeys, &mut inspects);
    }

    println!("{:?}", inspects);

    inspects.sort();
    inspects.reverse();

    println!("{:?}", inspects[0] * inspects[1]);

    fn round_2(monkeys: &mut Vec<Monkey>, inspects: &mut Vec<i32>) {
        let mod_space: i64 = monkeys.iter().map(|m| m.test).product();

        // run round
        for monkey_index in 0..monkeys.len() {
            let monkey = monkeys[monkey_index].clone();

            for item in monkey.items.iter() {
                inspects[monkey_index] += 1;
                let new_item = match monkey.operation {
                    Operation::Add(n) => item + n,
                    Operation::Multiply(n) => item * n,
                    Operation::Square => item * item,
                };

                let n = new_item % mod_space;

                if n % monkey.test == 0 {
                    // true branch

                    monkeys[monkey.true_branch as usize].items.push_back(n);
                } else {
                    // false branch
                    monkeys[monkey.false_branch as usize].items.push_back(n);
                }
            }

            monkeys[monkey_index].items.clear();
        }
    }

    let mut inspects_2 = vec![0; monkeys.len()];
    for _ in 0..10000 {
        round_2(&mut monkeys2, &mut inspects_2);
    }

    inspects_2.sort();
    inspects_2.reverse();

    println!("{:?}", i64::from(inspects_2[0]) * i64::from(inspects_2[1]));
}

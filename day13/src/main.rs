use serde_json::Value;

#[derive(PartialEq)]
enum CorrectOrder {
    True,
    False,
    Undecided,
}

fn has_correct_order(l1: &Value, l2: &Value) -> CorrectOrder {
    if l1.is_number() && l2.is_number() {
        let diff = l1.as_i64().unwrap() - l2.as_i64().unwrap();
        if diff < 0 {
            return CorrectOrder::True;
        } else if diff > 0 {
            return CorrectOrder::False;
        } else {
            return CorrectOrder::Undecided;
        }
    }

    if l1.is_number() && l2.is_array() {
        return has_correct_order(&Value::Array(vec![l1.clone()]), l2);
    }

    if l1.is_array() && l2.is_number() {
        return has_correct_order(l1, &Value::Array(vec![l2.clone()]));
    }

    if l1.is_array() && l2.is_array() {
        let l1 = l1.as_array().unwrap();
        let l2 = l2.as_array().unwrap();

        let longest = if l1.len() > l2.len() {
            l1.len()
        } else {
            l2.len()
        };

        for i in 0..longest {
            if l1.len() <= i {
                return CorrectOrder::True;
            }

            if l2.len() <= i {
                return CorrectOrder::False;
            }

            match has_correct_order(&l1[i].clone(), &l2[i].clone()) {
                CorrectOrder::True => return CorrectOrder::True,
                CorrectOrder::False => return CorrectOrder::False,
                CorrectOrder::Undecided => (),
            }
        }

        return CorrectOrder::Undecided;
    }

    return CorrectOrder::False;
}

fn main() {
    let file_contents = std::fs::read_to_string("./input.txt").expect("Could not read file");
    // let file_contents = std::fs::read_to_string("./test.txt").expect("Could not read file");

    let mut iter = file_contents.lines();

    let mut index = 0;

    let mut part1 = 0;

    let mut part2 = Vec::new();

    loop {
        let l = iter.next();
        index += 1;

        if l.is_none() {
            break;
        }

        let l1: Value = serde_json::from_str(l.unwrap()).unwrap();
        let l2: Value = serde_json::from_str(iter.next().unwrap()).unwrap();

        part2.push(l1.clone());
        part2.push(l2.clone());

        let _ = iter.next();

        println!("{}\n{}\n", l1, l2);

        match has_correct_order(&l1, &l2) {
            CorrectOrder::True => {
                println!("Has right order: {}", index);
                part1 += index
            }

            _ => (),
        }

        // println!("{}", has_right_order(&l1, &l2));
        println!("-------------------------------------------------");
    }

    println!("Part 1: {}", part1);

    /// part 2
    part2.push(serde_json::from_str("[[2]]").unwrap());
    part2.push(serde_json::from_str("[[6]]").unwrap());

    // bubble sort

    let mut swapped = true;

    while swapped {
        swapped = false;
        for i in 0..part2.len() - 1 {
            if has_correct_order(&part2[i], &part2[i + 1]) == CorrectOrder::False {
                part2.swap(i, i + 1);
                swapped = true;
            }
        }
    }

    let p1 = part2
        .iter()
        .position(|v| serde_json::to_string(v).unwrap() == "[[6]]")
        .unwrap();
    let p2 = part2
        .iter()
        .position(|v| serde_json::to_string(v).unwrap() == "[[2]]")
        .unwrap();

    println!("Part 2: {}", (p1 + 1) * (p2 + 1));

    println!("done");
}

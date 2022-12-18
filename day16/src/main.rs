use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct Node {
    name: String,
    flow: i32,
    neighbors: Vec<String>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Path {
    path: Vec<String>,
    open_valves: Vec<String>,
}

const OPEN_FLOW: &str = "OPEN_FLOW";

fn calculate_path_flow(
    path: &Vec<String>,
    node_lookup: &HashMap<String, &Node>,
    max_steps: usize,
) -> i32 {
    let mut open_valves = Vec::new();

    let mut total_flow = 0;

    for index in 1..max_steps {
        total_flow += open_valves
            .iter()
            .map(|valve| node_lookup.get(*valve).unwrap().flow)
            .sum::<i32>();

        if path.len() <= index {
            continue;
        }

        let n = &path[index];

        if n == OPEN_FLOW && !open_valves.contains(&&path[index - 1]) {
            open_valves.push(&path[index - 1]);
            continue;
        }
    }

    return total_flow;
}

fn calculate_shortest_path(
    from: &str,
    to: &str,
    nodes: &Vec<Node>,
    cache: &mut HashMap<String, Vec<String>>,
) -> Option<Vec<String>> {
    if let Some(path) = cache.get(&(from.to_owned() + to)) {
        return Some(path.clone());
    }

    let mut queue = VecDeque::new();
    let mut visited: HashSet<String> = HashSet::new();

    queue.push_back(vec![from.to_owned()]);

    while let Some(path) = queue.pop_front() {
        let last = path.last().unwrap();

        if last == to {
            cache.insert(from.to_owned() + to, path.clone());
            cache.insert(to.to_owned() + from, path.iter().rev().cloned().collect());
            return Some(path);
        }

        if visited.contains(last) {
            continue;
        }

        visited.insert(last.to_owned());

        let node = nodes
            .iter()
            .filter(|q| q.name != OPEN_FLOW)
            .find(|n| n.name == *last)
            .unwrap();

        for neighbor in &node.neighbors {
            let mut new_path = path.clone();
            new_path.push(neighbor.to_owned());
            queue.push_back(new_path);
        }
    }

    return None;
}

fn main() {
    let file_contents =
        std::fs::read_to_string("./input.txt").expect("Something went wrong reading the file");

    let nodes: Vec<Node> = file_contents
        .lines()
        .map(|line| {
            let name = line[6..=7].to_string();

            let flow = line[23..]
                .split(";")
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            let v_index = line.find("valve");
            let v1_index = line.find("valves");

            let slice = if v1_index.is_some() {
                &line[v1_index.unwrap() + 7..]
            } else {
                &line[v_index.unwrap() + 6..]
            };

            let neighbors = slice
                .split(" ")
                .map(|s| s.replace(",", "").trim().to_string())
                .collect();

            println!("{} -> {:?}", name, neighbors);

            return Node {
                name: name,
                flow: flow,
                neighbors: neighbors,
            };
        })
        .collect();

    let max_steps = 31;

    let node_lookup: std::collections::HashMap<String, &Node> =
        nodes.iter().map(|node| (node.name.clone(), node)).collect();

    let mut tested_paths = HashMap::new();

    let mut paths_to_test: Vec<Vec<String>> = vec![vec!["AA".to_owned()]];

    let mut max_flow = 0;

    let nodes_with_flow: Vec<&Node> = nodes.iter().filter(|n| n.flow != 0).collect();
    let mut cache = HashMap::new();

    // part 1

    while let Some(path) = paths_to_test.pop() {
        if tested_paths.get(&path).is_some() {
            continue;
        }
        let flow = calculate_path_flow(&path, &node_lookup, 31);

        tested_paths.insert(path.clone(), flow);

        if max_flow < flow {
            max_flow = flow;
            // println!("New max flow: {}, len: {}", max_flow, path.len());
        }

        if path.len() > max_steps {
            continue;
        }

        let name = path.iter().filter(|q| *q != OPEN_FLOW).last().unwrap();

        for p in &nodes_with_flow {
            if p.name == *name {
                continue;
            }

            if let Some(index) = path.iter().position(|q| q == &p.name) {
                if path.iter().nth(index + 1) == Some(&OPEN_FLOW.to_owned()) {
                    continue;
                }
            }

            let path_to_node = calculate_shortest_path(name, &p.name, &nodes, &mut cache);

            if path_to_node.is_none() {
                continue;
            }

            if (path.len() - 1) + (path_to_node.as_ref().unwrap().len() - 1) > max_steps {
                continue;
            }

            let mut new_path = path.clone();
            new_path.extend(path_to_node.unwrap()[1..].iter().cloned());
            new_path.push(OPEN_FLOW.to_owned());
            paths_to_test.push(new_path.clone());
        }
    }

    println!("Part 1: {}", max_flow);

    // part 2

    let paths_w_elephant: Vec<(_, _)> = tested_paths
        .iter()
        .filter_map(|(k, _)| (k.len() < 27).then(|| (k, calculate_path_flow(k, &node_lookup, 27))))
        .filter(|(_, flow)| *flow > 1000)
        .collect();

    let mut max_flow = 0;

    let opened_map = paths_w_elephant
        .iter()
        .map(|(path, _)| {
            let opened_set: HashSet<&String> = path
                .iter()
                .enumerate()
                .filter_map(|(i, s)| {
                    if *s == OPEN_FLOW {
                        return path.get(i - 1);
                    } else {
                        return None;
                    }
                })
                .collect();

            return opened_set;
        })
        .collect::<Vec<_>>();

    // find pair with no overlap and max flow
    for index in 0..paths_w_elephant.len() {
        let (_, flow1) = paths_w_elephant[index];

        println!("index: {}", index);

        let path1_opened_set = &opened_map[index];

        for index2 in index + 1..paths_w_elephant.len() {
            let (_, flow2) = paths_w_elephant[index2];

            let path2_opened_set = &opened_map[index2];

            let overlap = path1_opened_set.intersection(&path2_opened_set).count() > 0;

            if overlap {
                continue;
            }

            if max_flow < flow1 + flow2 {
                max_flow = flow1 + flow2;
                // println!("New max flow: {}", max_flow);
            }
        }
    }

    println!("Part 2: {}", max_flow);
}

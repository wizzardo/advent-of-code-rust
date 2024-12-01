use std::collections::{HashMap, HashSet};
use common::{char_at, find_substring};

pub fn calculate1(input: &str) -> String {
    let mut path: &str = "";
    let mut nodes: HashMap<&str, [&str; 2]> = HashMap::new();

    input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            if path.is_empty() {
                path = line;
                return;
            }

            let node = &line[..3];

            let left_start = find_substring(line,"(").unwrap() + 1;
            let left = &line[left_start..left_start + 3];

            let right_start = find_substring(line,")").unwrap() - 3;
            let right = &line[right_start..right_start + 3];
            nodes.insert(node, [left, right]);
        });

    let mut steps = 0;
    let mut position = "AAA";

    while !position.eq("ZZZ") {
        if char_at(path, steps % path.len()).unwrap() == 'L' {
            position = nodes.get(position).unwrap()[0];
        } else {
            position = nodes.get(position).unwrap()[1];
        }
        steps += 1;
    }

    return steps.to_string();
}


pub fn calculate2(input: &str) -> String {
    let mut path: &str = "";
    let mut nodes: HashMap<&str, [&str; 2]> = HashMap::new();
    let mut positions: Vec<&str> = Vec::new();

    input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            if path.is_empty() {
                path = line;
                return;
            }

            let node = &line[..3];

            let left_start = find_substring(line, "(").unwrap() + 1;
            let left = &line[left_start..left_start + 3];

            let right_start = find_substring(line,")").unwrap() - 3;
            let right = &line[right_start..right_start + 3];
            nodes.insert(node, [left, right]);
            if node.ends_with("A") {
                positions.push(node);
            }
        });

    let mut steps_for_path: Vec<u32> = Vec::new();
    for i in 0..positions.len() {
        let mut k = 0;
        let mut l: Vec<u32> = Vec::new();

        let mut position = *positions.get(i).unwrap();
        let mut ends: HashSet<&str> = HashSet::new();
        loop {
            while !position.ends_with("Z") {
                if char_at(path, k % path.len()).unwrap() == 'L' {
                    position = nodes.get(position).unwrap()[0];
                } else {
                    position = nodes.get(position).unwrap()[1];
                }
                k += 1;
            }

            l.push(k as u32);

            if char_at(path, k % path.len()).unwrap() == 'L' {
                position = nodes.get(position).unwrap()[0];
            } else {
                position = nodes.get(position).unwrap()[1];
            }

            if !ends.insert(position) {
                break;
            }

            k += 1
        }
        steps_for_path.push(*l.get(0).unwrap())
    }


    let mut multipliers: Vec<Vec<u32>> = Vec::new();
    for i in 0..steps_for_path.len() {
        let mut m: u32 = 2;
        let n = *steps_for_path.get(i).unwrap();
        let mut l: Vec<u32> = Vec::new();
        while m < n {
            if (n % m) == 0 {
                l.push(m);
            }
            m += 1;
        }
        if l.is_empty() {
            l.push(n);
        }
        multipliers.push(l);
    }

    let mut result: u64 = 1;
    let mut unique_numbers: HashSet<u32> = HashSet::new();
    multipliers.iter()
        .flat_map(|x| x)
        .filter(|x| unique_numbers.insert(**x))
        .for_each(|x| {
            result *= *x as u64;
        });


    return result.to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1("
                RL

                AAA = (BBB, CCC)
                BBB = (DDD, EEE)
                CCC = (ZZZ, GGG)
                DDD = (DDD, DDD)
                EEE = (EEE, EEE)
                GGG = (GGG, GGG)
                ZZZ = (ZZZ, ZZZ)
            "
        );
        assert_eq!(result, "2")
    }

    #[test]
    fn test_1_2() {
        let result = calculate1("
                LLR

                AAA = (BBB, BBB)
                BBB = (AAA, ZZZ)
                ZZZ = (ZZZ, ZZZ)
            "
        );
        assert_eq!(result, "6")
    }

    #[test]
    fn test_2() {
        let result = calculate2("
                LR

                11A = (11B, XXX)
                11B = (XXX, 11Z)
                11Z = (11B, XXX)
                22A = (22B, XXX)
                22B = (22C, 22C)
                22C = (22Z, 22Z)
                22Z = (22B, 22B)
                XXX = (XXX, XXX)
            "
        );
        assert_eq!(result, "6")
    }
}
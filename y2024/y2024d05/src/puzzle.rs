use std::collections::HashMap;

pub fn calculate1(input: &str) -> String {
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            if line.len() == 5 {
                let (l, r) = line.split_once('|').unwrap();
                let l = l.parse::<u32>().unwrap();
                let r = r.parse::<u32>().unwrap();
                rules.entry(l).or_default().push(r);
            } else {
                updates.push(
                    line.split(',')
                        .map(|it| it.parse::<u32>().unwrap())
                        .collect(),
                );
            }
        });
    let mut sum = 0;
    let empty_list = Vec::with_capacity(0);

    let is_correct = |update: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>| {
        for (i, x) in update.iter().enumerate() {
            for before in rules.get(x).or(Some(&empty_list)).unwrap() {
                if update[0..i].contains(before) {
                    return false;
                }
            }
        }
        true
    };

    for update in updates {
        if !is_correct(&update, &rules) {
            continue;
        }

        sum += update[update.len() / 2];
    }
    format!("{sum}")
}

pub fn calculate2(input: &str) -> String {
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            if line.len() == 5 {
                let (l, r) = line.split_once('|').unwrap();
                let l = l.parse::<u32>().unwrap();
                let r = r.parse::<u32>().unwrap();
                rules.entry(l).or_default().push(r);
            } else {
                updates.push(
                    line.split(',')
                        .map(|it| it.parse::<u32>().unwrap())
                        .collect(),
                );
            }
        });
    let mut sum = 0;
    let empty_list = Vec::with_capacity(0);

    let is_correct = |update: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>| {
        for (i, x) in update.iter().enumerate() {
            for before in rules.get(x).or(Some(&empty_list)).unwrap() {
                if update[0..i].contains(before) {
                    return false;
                }
            }
        }
        true
    };

    for mut update in updates {
        if is_correct(&update, &rules) {
            continue;
        }

        loop {
            let mut index = 0;
            'outer: for (i, x) in update.iter().enumerate() {
                for before in rules.get(x).or(Some(&empty_list)).unwrap() {
                    if update[0..i].contains(before) {
                        index = i;
                        break 'outer;
                    }
                }
            }

            update.swap(index, index - 1);

            if is_correct(&update, &rules) {
                break;
            }
        }

        sum += update[update.len() / 2];
    }

    format!("{sum}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                47|53
                97|13
                97|61
                97|47
                75|29
                61|13
                75|53
                29|13
                97|29
                53|29
                61|53
                97|53
                61|29
                47|13
                75|47
                97|75
                47|61
                75|61
                47|29
                75|13
                53|13

                75,47,61,53,29
                97,61,53,29,13
                75,29,13
                75,97,47,61,53
                61,13,29
                97,13,75,29,47
            ",
        );
        assert_eq!(result, "143")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                47|53
                97|13
                97|61
                97|47
                75|29
                61|13
                75|53
                29|13
                97|29
                53|29
                61|53
                97|53
                61|29
                47|13
                75|47
                97|75
                47|61
                75|61
                47|29
                75|13
                53|13

                75,47,61,53,29
                97,61,53,29,13
                75,29,13
                75,97,47,61,53
                61,13,29
                97,13,75,29,47
            ",
        );
        assert_eq!(result, "123")
    }
}

use std::collections::{HashMap, HashSet};

pub fn calculate1(input: &str) -> String {
    let mut beams: HashSet<usize> = HashSet::new();
    let lines: Vec<&str> = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .collect();

    let mut split_count = 0;
    beams.insert(lines[0].find('S').unwrap());
    for i in 1..lines.len() {
        let line = lines[i];
        let mut next_beams = HashSet::new();
        for beam in &beams {
            match line[*beam..].chars().next().unwrap() {
                '.' => { next_beams.insert(*beam); }
                '^' => {
                    next_beams.insert(*beam - 1);
                    next_beams.insert(*beam + 1);
                    split_count += 1
                }
                _ => { panic!() }
            };
        }
        beams = next_beams;
    }

    let result = split_count;
    format!("{result}")
}

pub fn calculate2(input: &str) -> String {
    let mut beams: HashMap<usize, usize> = HashMap::new();
    let lines: Vec<&str> = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .collect();

    beams.insert(lines[0].find('S').unwrap(), 1);
    for i in 1..lines.len() {
        let line = lines[i];
        let mut next_beams = HashMap::new();
        for (beam, paths) in &beams {
            match line[*beam..].chars().next().unwrap() {
                '.' => {
                    let entry = next_beams.entry(*beam).or_insert(0);
                    *entry += paths;
                }
                '^' => {
                    let entry = next_beams.entry(*beam - 1).or_insert(0);
                    *entry += paths;
                    let entry = next_beams.entry(*beam + 1).or_insert(0);
                    *entry += paths;
                }
                _ => { panic!() }
            };
        }
        // println!("{:?}", next_beams);
        beams = next_beams;
    }

    let result = beams.iter().fold(0, |acc, (_, paths)| acc + paths);
    format!("{result}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                .......S.......
                ...............
                .......^.......
                ...............
                ......^.^......
                ...............
                .....^.^.^.....
                ...............
                ....^.^...^....
                ...............
                ...^.^...^.^...
                ...............
                ..^...^.....^..
                ...............
                .^.^.^.^.^...^.
                ...............
            ",
        );
        assert_eq!(result, "21")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                .......S.......
                ...............
                .......^.......
                ...............
                ......^.^......
                ...............
                .....^.^.^.....
                ...............
                ....^.^...^....
                ...............
                ...^.^...^.^...
                ...............
                ..^...^.....^..
                ...............
                .^.^.^.^.^...^.
                ...............
            ",
        );
        assert_eq!(result, "40")
    }
}

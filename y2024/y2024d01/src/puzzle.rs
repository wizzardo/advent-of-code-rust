use common::read_numbers;
use std::collections::HashMap;

pub fn calculate1(input: &str) -> String {
    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();
    input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            let mut numbers: [i32; 2] = [0; 2];
            read_numbers(line, 0, line.len(), &mut numbers);
            left_numbers.push(numbers[0]);
            right_numbers.push(numbers[1]);
        });

    left_numbers.sort_unstable();
    right_numbers.sort_unstable();

    assert_eq!(left_numbers.len(), right_numbers.len());

    let mut total_distance = 0;

    for i in 0..left_numbers.len() {
        let l = left_numbers[i];
        let r = right_numbers[i];

        total_distance += (l - r).abs();
    }

    format!("{total_distance}")
}

pub fn calculate2(input: &str) -> String {
    let mut right_map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut left_numbers: Vec<i32> = Vec::new();
    input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .enumerate()
        .for_each(|(i, line)| {
            let mut numbers: [i32; 2] = [0; 2];
            read_numbers(line, 0, line.len(), &mut numbers);

            right_map.entry(numbers[1]).or_default().push(i as i32);
            left_numbers.push(numbers[0]);
        });

    let score: i32 = left_numbers
        .iter()
        .map(|it| (*it, right_map.get(&it).map(|it| it.len()).unwrap_or(0)))
        // .inspect(|it| println!("{:?}", it))
        .map(|(it, n)| it * (n as i32))
        .sum();

    format!("{score}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                3   4
                4   3
                2   5
                1   3
                3   9
                3   3
            ",
        );
        assert_eq!(result, "11")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                3   4
                4   3
                2   5
                1   3
                3   9
                3   3
            ",
        );
        assert_eq!(result, "31")
    }
}

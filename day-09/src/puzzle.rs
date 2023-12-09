use common::read_numbers;
use itertools::Itertools;

const MAX_SIZE: usize = 22;

pub fn calculate1(input: &str) -> String {
    let mut slices: [i32; MAX_SIZE * MAX_SIZE] = [0; MAX_SIZE * MAX_SIZE];

    let sum: i32 = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let count = read_numbers(line, 0, line.len(), &mut slices);
            let (left, right) = slices.split_at_mut(count);
            return get_next_sequence_value_slices(&left, right);
        })
        // .inspect(|line| { dbg!(line); })
        .sum();

    return sum.to_string();
}
fn get_next_sequence_value_slices(sequence: &[i32], slices: &mut [i32]) -> i32 {
    if sequence.iter().all_equal() {
        return sequence[0];
    }

    let (next, rest) = slices.split_at_mut(sequence.len() - 1);
    for i in 0..sequence.len() - 1 {
        next[i] = sequence[i + 1] - sequence[i];
    }

    let value = get_next_sequence_value_slices(&next[0..sequence.len() - 1], rest);
    return sequence.last().or(Some(&0)).unwrap() + value;
}

fn get_next_sequence_value(sequence: &[i32], slices: &mut [Vec<i32>]) -> i32 {
    if sequence.iter().all_equal() {
        return sequence[0];
    }

    let (first, rest) = slices.split_at_mut(1);
    let next = first[0].as_mut_slice();
    for i in 0..sequence.len() - 1 {
        next[i] = sequence[i + 1] - sequence[i];
    }

    let value = get_next_sequence_value(&next[0..sequence.len() - 1], rest);
    return sequence.last().or(Some(&0)).unwrap() + value;
}

fn get_prev_sequence_value(sequence: &[i32], slices: &mut [Vec<i32>]) -> i32 {
    if sequence.iter().all_equal() {
        return sequence[0];
    }

    let (first, rest) = slices.split_at_mut(1);
    let next = first[0].as_mut_slice();
    for i in 0..sequence.len() - 1 {
        next[i] = sequence[i + 1] - sequence[i];
    }

    let value = get_prev_sequence_value(&next[0..sequence.len() - 1], rest);
    return sequence.first().or(Some(&0)).unwrap() - value;
}

pub fn calculate2(input: &str) -> String {
    // let mut tmp: Vec<&mut [i32]> = vec![&mut [0; MAX_SIZE]; MAX_SIZE];
    let mut tmp: Vec<Vec<i32>> = Vec::with_capacity(MAX_SIZE);
    // let mut array: [i32; MAX_SIZE] = [0; MAX_SIZE];
    for _ in 0..MAX_SIZE {
        let mut vec = Vec::with_capacity(MAX_SIZE);
        for _ in 0..MAX_SIZE {
            vec.push(0);
        }
        tmp.push(vec);
    }

    // let mut slices = tmp.as_mut_slice();

    let sum: i32 = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let slices = tmp.as_mut_slice();

            // let mut numbers = &mut slices[0];
            // let mut numbers = tmp.get_mut(0).unwrap();
            // let count = read_numbers(line, 0, line.len(), slices[0]);
            let count = read_numbers(line, 0, line.len(), slices[0].as_mut_slice());
            let (left, right) = slices.split_at_mut(1);
            return get_prev_sequence_value(&left[0][0..count], right);
        })
        // .inspect(|line| { dbg!(line); })
        .sum();

    return sum.to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1("
                0 3 6 9 12 15
                1 3 6 10 15 21
                10 13 16 21 30 45
            "
        );
        assert_eq!(result, "114")
    }


    #[test]
    fn test_2() {
        let result = calculate2("
                0 3 6 9 12 15
                1 3 6 10 15 21
                10 13 16 21 30 45
            "
        );
        assert_eq!(result, "2")
    }
}
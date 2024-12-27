use std::collections::{HashMap, HashSet};

pub fn calculate1(input: &str) -> String {
    let result: u64 = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let n = line.parse::<u64>().unwrap();
            process(n, 2000)
        })
        .sum();
    format!("{result}")
}

fn process(mut v: u64, n: usize) -> u64 {
    for _ in 0..n {
        v = prune(mix(v, v << 6));
        v = prune(mix(v, v >> 5));
        v = prune(mix(v, v << 11));
    }
    v
}

fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

fn prune(a: u64) -> u64 {
    a & 16777215
}

fn collect(mut v: u64, n: usize) -> Vec<i8> {
    let mut result = Vec::with_capacity(n + 1);
    result.push((v % 10) as i8);
    for _ in 0..n {
        v = prune(mix(v, v << 6));
        v = prune(mix(v, v >> 5));
        v = prune(mix(v, v << 11));
        result.push((v % 10) as i8);
    }
    result
}

pub fn calculate2(input: &str) -> String {
    let mut sequences: HashSet<(i8, i8, i8, i8)> = HashSet::new();
    let buyers: Vec<HashMap<(i8, i8, i8, i8), i8>> = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let n = line.parse::<u64>().unwrap();

            let mut first_sequence_value: HashMap<(i8, i8, i8, i8), i8> = HashMap::new();

            collect(n, 2000)
                .windows(5)
                .for_each(|w| {
                    let sequence = (
                        w[1] - w[0],
                        w[2] - w[1],
                        w[3] - w[2],
                        w[4] - w[3]
                    );

                    sequences.insert(sequence);

                    let value = w[4];
                    first_sequence_value.entry(sequence).or_insert(value);
                });

            first_sequence_value
        })
        .collect();

    let (result, _) = find_max_sequence(&sequences, &buyers);
    format!("{result}")
}

#[inline(never)]
fn find_max_sequence(
    sequences: &HashSet<(i8, i8, i8, i8)>,
    buyers: &Vec<HashMap<(i8, i8, i8, i8), i8>>
) -> (u64, (i8, i8, i8, i8)) {
    let mut max = 0;
    let mut max_seq = (0, 0, 0, 0);
    for x in sequences {
        let mut sum = 0;
        for buyer in buyers {
            sum += *buyer.get(x).unwrap_or(&0) as u64;
        }
        if sum > max {
            max = sum;
            max_seq = x.clone();
        }
    }
    // println!("{:?}", max_seq);
    (max, max_seq)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                1
                10
                100
                2024
            ",
        );
        assert_eq!(result, "37327623")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                1
                2
                3
                2024
            ",
        );
        assert_eq!(result, "23")
    }
}

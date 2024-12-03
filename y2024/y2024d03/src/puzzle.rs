use regex::Regex;
use std::cell::Cell;

pub fn calculate1(input: &str) -> String {
    let mul_pattern = Regex::new(r"mul\((?<l>\d{1,3}),(?<r>\d{1,3})\)").unwrap();
    let result = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            mul_pattern
                .captures_iter(line)
                .map(|x| {
                    (
                        x.name("l").unwrap().as_str().parse::<u32>().unwrap(),
                        x.name("r").unwrap().as_str().parse::<u32>().unwrap(),
                    )
                })
                .map(|(l, r)| l * r)
                .sum::<u32>()
        })
        .sum::<u32>();
    format!("{result}")
}

pub fn calculate2(input: &str) -> String {
    let pattern = Regex::new(r"do\(\)|don't\(\)|mul\((?<l>\d{1,3}),(?<r>\d{1,3})\)").unwrap();
    let do_mul: Cell<bool> = Cell::new(true);
    let result = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            pattern
                .captures_iter(line)
                .inspect(|it| {
                    let c = it.get(0).unwrap().as_str();
                    if c.starts_with("don") {
                        do_mul.set(false);
                    } else if c.starts_with("do") {
                        do_mul.set(true);
                    }
                    // println!("{} - {}", c, do_mul.get());
                })
                .filter(|cap| do_mul.get() && cap.get(0).unwrap().as_str().starts_with("mul"))
                .map(|x| {
                    (
                        x.name("l").unwrap().as_str().parse::<u32>().unwrap(),
                        x.name("r").unwrap().as_str().parse::<u32>().unwrap(),
                    )
                })
                .map(|(l, r)| l * r)
                .sum::<u32>()
        })
        .sum::<u32>();
    format!("{result}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
            ",
        );
        assert_eq!(result, "161")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
            ",
        );
        assert_eq!(result, "48")
    }
}

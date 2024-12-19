pub fn calculate1(input: &str) -> String {
    let result: i64 = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let (l, r) = line.split_once(": ").unwrap();
            let l: i64 = l.parse().unwrap();
            let r: Vec<i64> = r.split(" ").map(|it| it.parse::<i64>().unwrap()).collect();
            (l, r)
        })
        .filter(|(l, r)| calculate_is_valid_1(*l, 0, &r))
        .map(|(l, _)| l)
        .sum();
    format!("{result}")
}

fn calculate_is_valid_1(result: i64, current: i64, operands: &[i64]) -> bool {
    if current == result && operands.len() == 0 {
        return true;
    }

    if current > result {
        return false;
    }

    if operands.len() == 0 {
        return false;
    }

    if calculate_is_valid_1(result, current + operands[0], &operands[1..]) {
        true
    } else {
        calculate_is_valid_1(result, current * operands[0], &operands[1..])
    }
}

pub fn calculate2(input: &str) -> String {
    let result: i64 = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let (l, r) = line.split_once(": ").unwrap();
            let l: i64 = l.parse().unwrap();
            let r: Vec<i64> = r.split(" ").map(|it| it.parse::<i64>().unwrap()).collect();
            (l, r)
        })
        .filter(|(l, r)| calculate_is_valid_2(*l, 0, &r))
        .map(|(l, _)| l)
        .sum();
    format!("{result}")
}

fn calculate_is_valid_2(result: i64, current: i64, operands: &[i64]) -> bool {
    if current == result && operands.len() == 0 {
        return true;
    }

    if current > result {
        return false;
    }

    if operands.len() == 0 {
        return false;
    }

    if calculate_is_valid_2(result, current + operands[0], &operands[1..]) {
        true
    } else if calculate_is_valid_2(result, current * operands[0], &operands[1..]) {
        true
    } else {
        let next = current * get_multiplier(operands[0]) + operands[0];
        calculate_is_valid_2(result, next, &operands[1..])
    }
}

fn get_multiplier(mut v: i64) -> i64 {
    let mut m = 10;
    while v >= 10 {
        m *= 10;
        v /= 10;
    }
    return m;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                190: 10 19
                3267: 81 40 27
                83: 17 5
                156: 15 6
                7290: 6 8 6 15
                161011: 16 10 13
                192: 17 8 14
                21037: 9 7 18 13
                292: 11 6 16 20
            ",
        );
        assert_eq!(result, "3749")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                190: 10 19
                3267: 81 40 27
                83: 17 5
                156: 15 6
                7290: 6 8 6 15
                161011: 16 10 13
                192: 17 8 14
                21037: 9 7 18 13
                292: 11 6 16 20
            ",
        );
        assert_eq!(result, "11387")
    }
}

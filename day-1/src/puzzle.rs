pub fn calculate1(input: &str) -> String {
    let sum: i32 = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let first = find_first_int(line);
            let last = find_last_int(line);
            first * 10 + last
        })
        // .inspect(|line| { dbg!(line); })
        .sum();

    return sum.to_string();
}

pub fn calculate2(input: &str) -> String {
    let sum: i32 = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let first = find_first_int_named(line);
            let last = find_last_int_named(line);
            first * 10 + last
        })
        // .inspect(|line| { dbg!(line); })
        .sum();

    return sum.to_string();
}

fn find_first_int(line: &str) -> i32 {
    for c in line.chars() {
        let char = c as u32;
        if char >= 48 && char <= 57 {
            return (char - 48) as i32;
        }
        // if c.is_digit(10) {
        //     return c.to_digit(10).unwrap() as i32;
        // }
    }
    return -1;
}

fn find_last_int(line: &str) -> i32 {
    for i in (0..line.len()).rev() {
        let c = line.chars().nth(i).unwrap();
        // if c.is_digit(10) {
        //     return c.to_digit(10).unwrap() as i32;
        // }
        let char = c as u32;
        if char >= 48 && char <= 57 {
            return (char - 48) as i32;
        }
    }
    return -1;
}

static DIGITS: [&str; 10] = ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];


fn find_first_int_named(line: &str) -> i32 {
    for i in 0..line.len() {
        let c = line.chars().nth(i).unwrap();
        if c.is_digit(10) {
            return c.to_digit(10).unwrap() as i32;
        }
        for j in 1..DIGITS.len() {
            if (&line[i..]).starts_with(DIGITS[j]) {
                return j as i32;
            }
        }
    }
    return -1;
}

fn find_last_int_named(line: &str) -> i32 {
    for i in (0..line.len()).rev() {
        let c = line.chars().nth(i).unwrap();
        if c.is_digit(10) {
            return c.to_digit(10).unwrap() as i32;
        }
        for j in 1..DIGITS.len() {
            if i + 1 < DIGITS[j].len() {
                continue;
            }
            if (&line[i + 1 - DIGITS[j].len()..]).starts_with(DIGITS[j]) {
                return j as i32;
            }
        }
    }
    return -1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1("
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
            "
        );
        assert_eq!(result, "142")
    }

    #[test]
    fn test_find_last_int() {
        assert_eq!(find_last_int("1"), 1);
        assert_eq!(find_last_int("12"), 2);
    }

    #[test]
    fn test_find_first_named() {
        assert_eq!(find_first_int_named("one"), 1);
        assert_eq!(find_first_int_named("atwo3"), 2);
    }

    #[test]
    fn test_find_last_named() {
        assert_eq!(find_last_int_named("one"), 1);
        assert_eq!(find_last_int_named("1two"), 2);
    }
}
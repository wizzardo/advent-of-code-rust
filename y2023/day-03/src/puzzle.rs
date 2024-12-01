pub fn calculate1(input: &str) -> String {
    let mut top_line: &str = "";
    let mut mid_line: &str = "";
    let sum: u32 = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            if top_line.len() == 0 {
                top_line = line;
                return 0;
            }
            if mid_line.len() == 0 {
                mid_line = line;
                return 0;
            }

            const MAX_NUMBERS: usize = 6;
            let mut numbers: [u32; MAX_NUMBERS] = [0; MAX_NUMBERS];
            let mut sum: u32 = 0;

            for i in 0..mid_line.len() {
                let char = mid_line.chars().nth(i).unwrap();
                if char == '.' {
                    continue;
                }
                if char.is_digit(10) {
                    continue;
                }
                let count = get_numbers_adjacent_to(top_line, mid_line, line, i, &mut numbers);
                let numbers = &numbers[0..count];
                sum += numbers.iter().sum::<u32>();
            }


            top_line = mid_line;
            mid_line = line;

            return sum;
        })
        // .inspect(|line| { dbg!(line); })
        .sum();

    return sum.to_string();
}

fn get_numbers_adjacent_to(top: &str, mid: &str, bot: &str, index: usize, into: &mut [u32; 6]) -> usize {
    let mut count = 0;
    count = get_numbers_adjacent_to_into(top, index, into, count);
    count = get_numbers_adjacent_to_into(mid, index, into, count);
    count = get_numbers_adjacent_to_into(bot, index, into, count);
    return count;
}

fn get_numbers_adjacent_to_into(line: &str, index: usize, into: &mut [u32; 6], mut at: usize) -> usize {
    if line.chars().nth(index).unwrap().is_digit(10) {
        let start = get_first_digit_char_index(line, index);
        let end = get_last_digit_char_index(line, index);
        into[at] = (&line[start..end]).parse::<u32>().unwrap();
        at += 1;
    } else {
        if index >= 1 && line.chars().nth(index - 1).unwrap().is_digit(10) {
            let start = get_first_digit_char_index(line, index - 1);
            let end = index;
            into[at] = (&line[start..end]).parse::<u32>().unwrap();
            at += 1;
        }
        if index + 1 < line.len() && line.chars().nth(index + 1).unwrap().is_digit(10) {
            let start = index + 1;
            let end = get_last_digit_char_index(line, index + 1);
            into[at] = (&line[start..end]).parse::<u32>().unwrap();
            at += 1;
        }
    }
    return at;
}

fn count_numbers_adjacent_to(top: &str, mid: &str, bot: &str, index: usize) -> usize {
    let mut count = 0;
    count += count_numbers_adjacent_to_index(top, index);
    count += count_numbers_adjacent_to_index(mid, index);
    count += count_numbers_adjacent_to_index(bot, index);
    return count;
}

fn count_numbers_adjacent_to_index(line: &str, index: usize) -> usize {
    let mut count = 0;
    if line.chars().nth(index).unwrap().is_digit(10) {
        count += 1;
    } else {
        if index >= 1 && line.chars().nth(index - 1).unwrap().is_digit(10) {
            count += 1;
        }
        if index + 1 < line.len() && line.chars().nth(index + 1).unwrap().is_digit(10) {
            count += 1;
        }
    }
    return count;
}

pub fn calculate2(input: &str) -> String {
    let mut top_line: &str = "";
    let mut mid_line: &str = "";
    let sum: u32 = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            if top_line.len() == 0 {
                top_line = line;
                return 0;
            }
            if mid_line.len() == 0 {
                mid_line = line;
                return 0;
            }

            const MAX_NUMBERS: usize = 6;
            let mut numbers: [u32; MAX_NUMBERS] = [0; MAX_NUMBERS];
            let mut sum: u32 = 0;

            for i in 0..mid_line.len() {
                let char = mid_line.chars().nth(i).unwrap();
                if char != '*' {
                    continue;
                }

                let count = count_numbers_adjacent_to(top_line, mid_line, line, i);
                if count == 2 {
                    get_numbers_adjacent_to(top_line, mid_line, line, i, &mut numbers);
                    sum += numbers.iter()
                        .take(2)
                        .fold(1, |acc, n| { n * acc });
                }
            }


            top_line = mid_line;
            mid_line = line;

            return sum;
        })
        // .inspect(|line| { dbg!(line); })
        .sum();

    return sum.to_string();
}


fn get_first_digit_char_index(s: &str, mid: usize) -> usize {
    let mut i = mid;
    while i > 0 && s.chars().nth(i - 1).unwrap().is_digit(10) {
        i = i - 1;
    }
    return i;
}

fn get_last_digit_char_index(s: &str, mid: usize) -> usize {
    let mut i = mid;
    while i < s.len() && s.chars().nth(i).unwrap().is_digit(10) {
        i = i + 1;
    }
    return i;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1("
                467..114..
                ...*......
                ..35..633.
                ......#...
                617*......
                .....+.58.
                ..592.....
                ......755.
                ...$.*....
                .664.598..
            "
        );
        assert_eq!(result, "4361")
    }

    #[test]
    fn test_2() {
        let result = calculate2("
                467..114..
                ...*......
                ..35..633.
                ......#...
                617*......
                .....+.58.
                ..592.....
                ......755.
                ...$.*....
                .664.598..
            "
        );
        assert_eq!(result, "467835")
    }

    #[test]
    fn test_get_numbers_adjacent_to_into() {
        const MAX_NUMBERS: usize = 6;
        let mut numbers: [u32; MAX_NUMBERS] = [0; MAX_NUMBERS];
        let count = get_numbers_adjacent_to_into("467..114..", 3, &mut numbers, 0);
        assert_eq!(1, count);
        assert_eq!(467, numbers[0]);
    }
}
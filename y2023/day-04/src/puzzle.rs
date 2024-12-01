use nom::FindSubstring;

pub fn calculate1(input: &str) -> String {
    let sum: u32 = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let position = line.find_substring(":").unwrap() + 1;
            let start_of_my_numbers = line.find_substring("|").unwrap();

            const MAX_NUMBERS: usize = 10;
            let mut numbers: [u32; MAX_NUMBERS] = [0; MAX_NUMBERS];

            let count = read_numbers(line, position, start_of_my_numbers, &mut numbers);
            return count_points(line, start_of_my_numbers + 1, line.len(), &numbers[0..count]);
        })
        // .inspect(|line| { dbg!(line); })
        .sum();

    return sum.to_string();
}

fn count_points(line: &str, mut from: usize, to: usize, winning_numbers: &[u32]) -> u32 {
    let mut count = 0;
    while from < to {
        if line.chars().nth(from).unwrap() <= ' ' {
            from += 1;
            continue;
        }
        let end = get_last_digit_char_index(line, from);
        let number: u32 = (&line[from..end]).parse().unwrap();
        if winning_numbers.contains(&number) {
            if count == 0 {
                count = 1;
            } else {
                count *= 2;
            }
        }
        from = end;
    }
    return count;
}

fn read_numbers(line: &str, mut from: usize, to: usize, into: &mut [u32; 10]) -> usize {
    let mut count = 0;
    while from < to {
        if line.chars().nth(from).unwrap() <= ' ' {
            from += 1;
            continue;
        }

        let end = get_last_digit_char_index(line, from);
        into[count] = (&line[from..end]).parse().unwrap();
        count += 1;
        from = end
    }
    return count;
}


pub fn calculate2(input: &str) -> String {
    let mut cards: [u32; 200] = [0; 200];
    input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            let position = line.find_substring(":").unwrap() + 1;
            let start_of_my_numbers = line.find_substring("|").unwrap();
            let card_number = get_card_number(line, position - 1);
            cards[card_number] += 1;

            const MAX_NUMBERS: usize = 10;
            let mut numbers: [u32; MAX_NUMBERS] = [0; MAX_NUMBERS];
            let count = read_numbers(line, position, start_of_my_numbers, &mut numbers);

            let matches = count_matches(line, start_of_my_numbers + 1, line.len(), &numbers[0..count]);
            for _i in 0..cards[card_number] {
                for j in 0..matches {
                    cards[card_number + 1 + j] += 1;
                }
            }
        });
    // .inspect(|line| { dbg!(line); })
    // .sum();

    let x: u32 = cards.iter().sum();
    return x.to_string();
}

fn count_matches(line: &str, mut from: usize, to: usize, winning_numbers: &[u32]) -> usize {
    let mut count = 0;
    while from < to {
        if line.chars().nth(from).unwrap() <= ' ' {
            from += 1;
            continue;
        }
        let end = get_last_digit_char_index(line, from);
        let number: u32 = (&line[from..end]).parse().unwrap();
        if winning_numbers.contains(&number) {
            count += 1;
        }
        from = end;
    }
    return count;
}

fn get_card_number(s: &str, end: usize) -> usize {
    let start = get_first_digit_char_index(s, end - 1);
    return (&s[start..end]).parse().unwrap();
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
                Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
            "
        );
        assert_eq!(result, "13")
    }

    #[test]
    fn test_2() {
        let result = calculate2("
                Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
            "
        );
        assert_eq!(result, "30")
    }
}
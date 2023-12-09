use std::str::FromStr;

pub fn get_first_digit_char_index(s: &str, mid: usize) -> usize {
    let mut i = mid;
    while i > 0 && s.chars().nth(i - 1).unwrap().is_digit(10) {
        i = i - 1;
    }
    return i;
}

pub fn get_last_digit_char_index(s: &str, mid: usize) -> usize {
    let mut i = mid;
    while i < s.len() && s.chars().nth(i).unwrap().is_digit(10) {
        i = i + 1;
    }
    return i;
}

pub fn read_numbers<T>(line: &str, mut from: usize, to: usize, into: &mut [T]) -> usize
    where T: FromStr, <T as FromStr>::Err: std::fmt::Debug {
    let mut count = 0;
    while from < to {
        if line.chars().nth(from).unwrap() <= ' ' {
            from += 1;
            continue;
        }

        let end: usize;
        if line.chars().nth(from).unwrap() == '-' {
            end = get_last_digit_char_index(line, from + 1);
        } else {
            end = get_last_digit_char_index(line, from);
        }

        into[count] = (&line[from..end]).parse().unwrap();
        count += 1;
        from = end
    }
    return count;
}
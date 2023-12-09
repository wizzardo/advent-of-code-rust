use std::str::FromStr;

pub fn get_first_digit_char_index(s: &str, mid: usize) -> usize {
    let mut i = mid;
    while i > 0 && char_at_unsafe(s, i - 1).is_digit(10) {
        i = i - 1;
    }
    return i;
}

pub fn get_last_digit_char_index(s: &str, mid: usize) -> usize {
    let mut i = mid;
    while i < s.len() && char_at_unsafe(s, i).is_digit(10) {
        i = i + 1;
    }
    return i;
}

fn char_at_unsafe(s: &str, i: usize) -> char {
    s[i..].chars().nth(0).unwrap()
}

pub fn char_at(s: &str, i: usize) -> Option<char> {
    if i >= s.len() {
        return None;
    }

    return s[i..].chars().nth(0);
}

pub fn read_numbers<T>(line: &str, mut from: usize, to: usize, into: &mut [T]) -> usize
    where T: FromStr, <T as FromStr>::Err: std::fmt::Debug {
    let mut count = 0;
    while from < to {
        if char_at_unsafe(line, from) <= ' ' {
            from += 1;
            continue;
        }

        let end: usize;
        if char_at_unsafe(line, from) == '-' {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slices() {
        let x = "a";
        assert_eq!(None, x[1..].chars().nth(0));
        assert_eq!(None, (&x[1..]).chars().nth(0));
    }
}
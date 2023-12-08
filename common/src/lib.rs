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
pub fn calculate1(_input: &str) -> String {
    let mut count = 0;

    let from = 134792;
    let to = 675810;

    for i in from..to {
        if is_valid_password_1(i) {
            count += 1;
        }
    }

    return count.to_string();
}

pub fn calculate2(_input: &str) -> String {
    let mut count = 0;

    let from = 134792;
    let to = 675810;

    for i in from..to {
        if is_valid_password_2(i) {
            count += 1;
        }
    }

    return count.to_string();
}

fn is_valid_password_1(password: u32) -> bool {
    let mut prev_digit: u32 = 99;
    let mut test = password;

    let mut has_doubles = false;
    while test > 0 {
        let digit = test % 10;
        has_doubles |= digit == prev_digit;
        if digit > prev_digit {
            return false;
        }

        prev_digit = digit;
        test /= 10;
    }

    has_doubles
}


fn is_valid_password_2(password: u32) -> bool {
    let mut prev_digit: u32 = 99;
    let mut test = password;

    let mut has_doubles = false;
    let mut doubles_counter = 0;
    while test > 0 {
        let digit = test % 10;
        if digit == prev_digit {
            doubles_counter += 1;
        } else {
            has_doubles |= doubles_counter == 1;
            doubles_counter = 0;
        }

        if digit > prev_digit {
            return false;
        }

        prev_digit = digit;
        test /= 10;
    }

    has_doubles |= doubles_counter == 1;

    has_doubles
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_password_1() {
        assert_eq!(is_valid_password_1(111111), true);
        assert_eq!(is_valid_password_1(223450), false);
        assert_eq!(is_valid_password_1(123789), false);
    }

    #[test]
    fn test_password_2() {
        assert_eq!(is_valid_password_2(112233), true);
        assert_eq!(is_valid_password_2(123444), false);
        assert_eq!(is_valid_password_2(111122), true);
    }
}
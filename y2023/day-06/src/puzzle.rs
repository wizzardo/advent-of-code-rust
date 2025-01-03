use common::read_numbers;

pub fn calculate1(input: &str) -> String {
    let input = input.trim_start();

    let mut result: u64 = 1;
    let mut time: [u64; 4] = [0; 4];
    let mut distance: [u64; 4] = [0; 4];
    let line_end = input.chars().position(|x| x == '\n').unwrap();
    let count = read_numbers(input, input.chars().position(|x| x == ':').unwrap() + 1, line_end, &mut time);
    {
        let line = &input[line_end + 1..];
        read_numbers(line, line.chars().position(|x| x == ':').unwrap() + 1, line.len(), &mut distance);
    }

    for i in 0..count {
        let t = time[i];
        let d = distance[i];

        let mut min: u64 = f64::ceil((d as f64) / (t as f64)) as u64;
        while min < t && min * (t - min) <= d {
            min += 1;
        }
        if min == t {
            panic!()
        }

        result *= t - min - min + 1;
    }

    return result.to_string();
}

pub fn calculate2(input: &str) -> String {
    let string = input.trim_start().replace(" ", "");
    let input = string.as_str();

    let mut result: u64 = 1;
    let mut time: [u64; 4] = [0; 4];
    let mut distance: [u64; 4] = [0; 4];
    let line_end = input.chars().position(|x| x == '\n').unwrap();
    let count = read_numbers(input, input.chars().position(|x| x == ':').unwrap() + 1, line_end, &mut time);
    {
        let line = &input[line_end + 1..];
        read_numbers(line, line.chars().position(|x| x == ':').unwrap() + 1, line.len(), &mut distance);
    }

    for i in 0..count {
        let t = time[i];
        let d = distance[i];

        let mut min: u64 = f64::ceil((d as f64) / (t as f64)) as u64;
        while min < t && min * (t - min) <= d {
            min += 1;
        }
        if min == t {
            panic!()
        }

        result *= t - min - min + 1;
    }

    return result.to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1("
                Time:      7  15   30
                Distance:  9  40  200
            "
        );
        assert_eq!(result, "288")
    }

    #[test]
    fn test_2() {
        let result = calculate2("
                Time:      7  15   30
                Distance:  9  40  200
            "
        );
        assert_eq!(result, "71503")
    }
}
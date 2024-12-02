use common::read_numbers;

pub fn calculate1(input: &str) -> String {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let mut numbers: [i32; 8] = [0; 8];
            let count = read_numbers(line, 0, line.len(), &mut numbers);
            Vec::from(&numbers[..count])
        })
        .collect();

    let result = reports
        .iter()
        .filter(|it| it.windows(2).all(|w| (w[0] - w[1]).abs() <= 3))
        .filter(|it| it.windows(2).all(|w| w[0] < w[1]) || it.windows(2).all(|w| w[0] > w[1]))
        .count();

    format!("{result}")
}

pub fn calculate2(input: &str) -> String {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let mut numbers: [i32; 8] = [0; 8];
            let count = read_numbers(line, 0, line.len(), &mut numbers);
            Vec::from(&numbers[..count])
        })
        .collect();

    let mut result = 0;
    for report in reports {
        if report.windows(2).all(|w| (w[0] - w[1]).abs() <= 3)
            && (report.windows(2).all(|w| w[0] < w[1]) || report.windows(2).all(|w| w[0] > w[1]))
        {
            result += 1;
            continue;
        }

        for i in 0..report.len() {
            let mut vec = report.clone();
            vec.remove(i);

            if vec.windows(2).all(|w| (w[0] - w[1]).abs() <= 3)
                && (vec.windows(2).all(|w| w[0] < w[1]) || vec.windows(2).all(|w| w[0] > w[1]))
            {
                result += 1;
                break;
            }
        }
    }

    format!("{result}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                7 6 4 2 1
                1 2 7 8 9
                9 7 6 2 1
                1 3 2 4 5
                8 6 4 4 1
                1 3 6 7 9
            ",
        );
        assert_eq!(result, "2")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                7 6 4 2 1
                1 2 7 8 9
                9 7 6 2 1
                1 3 2 4 5
                8 6 4 4 1
                1 3 6 7 9
            ",
        );
        assert_eq!(result, "4")
    }
}

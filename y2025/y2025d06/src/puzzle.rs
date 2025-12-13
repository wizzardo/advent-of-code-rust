pub fn calculate1(input: &str) -> String {
    let mut numbers: Vec<Vec<u64>> = vec![];
    let mut actions: Vec<char> = vec![];
    input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            let parts = line.split_whitespace();
            if line.starts_with('+') || line.starts_with('*') {
                for x in parts {
                    actions.push(x.chars().next().unwrap())
                }
            } else {
                numbers.push(parts.map(|x| x.parse::<u64>().unwrap()).collect())
            }
        });

    let mut result = 0;

    for (i, action) in actions.iter().enumerate() {
        result += match action {
            '*' => { numbers.iter().map(|v| v[i]).fold(1, |acc, v| acc * v) }
            '+' => { numbers.iter().map(|v| v[i]).fold(0, |acc, v| acc + v) }
            c => { panic!("Unexpected action {}", c) }
        }
    }

    format!("{result}")
}

pub fn calculate2(input: &str) -> String {
    let mut numbers: Vec<&str> = vec![];
    let mut actions: &str = "";
    let mut padding = usize::MAX;
    input
        .lines()
        .filter(|it| it.trim().len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            if padding == usize::MAX {
                padding = line.len() - line.trim_start_matches(|c: char| c.is_whitespace()).len()
            }
            let line = &line[padding..];

            if line.starts_with('+') || line.starts_with('*') {
                actions = line;
            } else {
                numbers.push(line);
            }
        });

    let mut result = 0;

    for (i, action) in actions.chars().enumerate() {
        if action.is_whitespace() {
            continue;
        }
        let start = i;
        let mut sub_result = match action {
            '*' => 1,
            '+' => 0,
            c => panic!("Unexpected action {}", c),
        };
        for column in start..usize::MAX {
            let mut number = 0u64;
            for line in &numbers {
                if column >= line.len() { continue; }

                let x = line[column..].chars().next().unwrap();
                if x.is_digit(10) {
                    number = number * 10 + x.to_digit(10).unwrap() as u64;
                }
            }
            if number == 0 {
                break;
            }


            match action {
                '*' => sub_result *= number,
                '+' => sub_result += number,
                c => panic!("Unexpected action {}", c),
            };
        }

        result += sub_result;
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
                123 328  51 64
                 45 64  387 23
                  6 98  215 314
                *   +   *   +
            ",
        );
        assert_eq!(result, "4277556")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                123 328  51 64
                 45 64  387 23
                  6 98  215 314
                *   +   *   +
            ",
        );
        assert_eq!(result, "3263827")
    }
}

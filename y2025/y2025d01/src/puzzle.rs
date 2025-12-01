pub fn calculate1(input: &str) -> String {
    let mut position = 50;
    let mut count = 0;
    input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            match line.chars().next().unwrap_or(' ') {
                'R' => {
                    position += line[1..].parse::<i32>().unwrap();
                    while position >= 100 {
                        position -= 100;
                    }
                }
                'L' => {
                    position -= line[1..].parse::<i32>().unwrap();
                    while position < 0 {
                        position += 100;
                    }
                }
                _ => { panic!("unknown start char at line: {}", line) }
            }
            // println!("position: {} - {}", position, line);
            if position == 0 {
                count += 1;
            }
        });
    let result = count;
    format!("{result}")
}

pub fn calculate2(input: &str) -> String {
    let mut position = 50;
    let mut count = 0;
    input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            match line.chars().next().unwrap_or(' ') {
                'R' => {
                    position += line[1..].parse::<i32>().unwrap();
                    while position >= 100 {
                        position -= 100;
                        count += 1;
                    }
                    if position == 0 {
                        count -= 1;
                    }
                }
                'L' => {
                    if position == 0 {
                        count -= 1;
                    }
                    position -= line[1..].parse::<i32>().unwrap();
                    while position < 0 {
                        position += 100;
                        count += 1;
                    }
                }
                _ => { panic!("unknown start char at line: {}", line) }
            }
            if position % 100 == 0 {
                count += 1;
            }
            // println!("position: {} - {} - {}", position, count, line);
        });
    let result = count;
    format!("{result}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                L68
                L30
                R48
                L5
                R60
                L55
                L1
                L99
                R14
                L82
            ",
        );
        assert_eq!(result, "3")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                L68
                L30
                R48
                L5
                R60
                L55
                L1
                L99
                R14
                L82
            ",
        );
        assert_eq!(result, "6")
    }
}

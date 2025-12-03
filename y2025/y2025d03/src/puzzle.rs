pub fn calculate1(input: &str) -> String {
    let sum = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let mut h = line.chars().nth(0).unwrap() as u32 - '0' as u32;
            let mut l = 0;
            for i in 1..line.len() - 1 {
                let d = line[i..].chars().next().unwrap() as u32 - '0' as u32;
                if d > h {
                    h = d;
                    l = 0;
                } else if d > l {
                    l = d;
                }
            }
            {
                let d = line[line.len() - 1..].chars().next().unwrap() as u32 - '0' as u32;
                if d > l {
                    l = d;
                }
            }
            // println!("{line}: {h}{l}");
            h * 10 + l
        })
        .sum::<u32>();
    let result = sum;
    format!("{result}")
}

pub fn calculate2(input: &str) -> String {
    let sum = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let mut number = [0u8; 12];
            for i in 0..line.len() {
                let d = line[i..].chars().next().unwrap() as u8 - '0' as u8;
                let offset = (i as i32 - (line.len() - number.len()) as i32).max(0) as usize;
                for j in offset..number.len() {
                    if d > number[j] {
                        number[j] = d;
                        for k in j + 1..number.len() {
                            number[k] = 0;
                        }
                        break;
                    }
                }
            }
            let mut n = 0u64;
            for x in number {
                n = n * 10 + x as u64;
            }

            // println!("{line}: {number:?} {n}");
            n
        })
        .sum::<u64>();
    let result = sum;
    format!("{result}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                987654321111111
                811111111111119
                234234234234278
                818181911112111
            ",
        );
        assert_eq!(result, "357")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                987654321111111
                811111111111119
                234234234234278
                818181911112111
            ",
        );
        assert_eq!(result, "3121910778619")
    }
}

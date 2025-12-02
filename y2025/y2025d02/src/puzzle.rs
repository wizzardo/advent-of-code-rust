pub fn calculate1(input: &str) -> String {
    let mut sum: u64 = 0;
    input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            for range in line.split(',') {
                let (from, to) = range.split_once('-').unwrap();
                let from = from.parse::<u64>().unwrap();
                let to = to.parse::<u64>().unwrap();
                for i in from..=to {
                    let id = i.to_string();
                    if id.len() % 2 != 0 {
                        continue;
                    }
                    if id[0..id.len() / 2].eq(&id[id.len() / 2..]) {
                        sum += i;
                    }
                }
            }
        });
    let result = sum;
    format!("{result}")
}

pub fn calculate2(input: &str) -> String {
    let mut sum: u64 = 0;
    input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            for range in line.split(',') {
                let (from, to) = range.split_once('-').unwrap();
                let from = from.parse::<u64>().unwrap();
                let to = to.parse::<u64>().unwrap();
                for i in from..=to {
                    let id = i.to_string();
                    for len in 1..=id.len() / 2 {
                        if id.len() % len != 0 {
                            continue;
                        }
                        let seq = &id[0..len];
                        if (1..id.len() / len).all(|j| seq.eq(&id[len * j..len * (j + 1)])) {
                            // println!("invalid {}", i);
                            sum += i;
                            break;
                        }
                    }
                }
            }
        });
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
                11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
            ",
        );
        assert_eq!(result, "1227775554")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
            ",
        );
        assert_eq!(result, "4174379265")
    }
}

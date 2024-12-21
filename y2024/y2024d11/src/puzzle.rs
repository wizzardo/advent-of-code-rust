use std::collections::HashMap;

pub fn calculate1(input: &str) -> String {
    let mut stones = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            line.split(" ")
                .map(|it| it.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .last()
        .unwrap();

    for _i in 0..25 {
        let mut next_stones = Vec::with_capacity(stones.len() * 2);

        for it in stones {
            if it == 0 {
                next_stones.push(1);
            } else if len(it) % 2 == 0 {
                let (l, r) = split(it);
                next_stones.push(l);
                next_stones.push(r);
            } else {
                next_stones.push(it * 2024);
            }
        }

        stones = next_stones;
        // println!("{_i}: {:?}", stones);
    }

    let result = stones.len();
    format!("{result}")
}

fn len(mut v: u64) -> u8 {
    let mut len = 1;
    while v >= 10 {
        v /= 10;
        len += 1;
    }
    len
}

fn split(v: u64) -> (u64, u64) {
    let l = len(v);
    let mut multiplier = 1;
    for _ in 0..l / 2 {
        multiplier *= 10;
    }
    (v / multiplier, v % multiplier)
}

pub fn calculate2(input: &str) -> String {
    let stones = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            line.split(" ")
                .map(|it| it.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .last()
        .unwrap();

    let mut cache: HashMap<(u64, u8), u64> = HashMap::new();
    fn compute(stone: u64, steps_left: u8, cache: &mut HashMap<(u64, u8), u64>) -> u64 {
        if steps_left == 0 {
            return 1;
        }
        if let Some(n) = cache.get(&(stone, steps_left)) {
            return *n;
        }

        let n = if stone == 0 {
            compute(1, steps_left - 1, cache)
        } else if len(stone) % 2 == 0 {
            let (l, r) = split(stone);
            compute(l, steps_left - 1, cache) + compute(r, steps_left - 1, cache)
        } else {
            compute(stone * 2024, steps_left - 1, cache)
        };
        cache.insert((stone, steps_left), n);
        n
    }

    let mut count = 0;
    for x in stones {
        count += compute(x, 75, &mut cache);
    }

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
                125 17
            ",
        );
        assert_eq!(result, "55312")
    }

    // #[test]
    // fn test_2() {
    //     let result = calculate2(
    //         "
    //             125 17
    //         ",
    //     );
    //     assert_eq!(result, "55312")
    // }
}

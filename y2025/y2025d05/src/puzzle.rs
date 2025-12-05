pub fn calculate1(input: &str) -> String {
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut count = 0;
    input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            if line.contains('-') {
                let (from, to) = line.split_once('-').unwrap();
                ranges.push((from.parse::<usize>().unwrap(), to.parse::<usize>().unwrap()));
            } else {
                let id = line.parse::<usize>().unwrap();
                if ranges.iter().any(|(from, to)| { id >= *from && id <= *to }) {
                    count += 1;
                }
            }
        });
    let result = count;
    format!("{result}")
}

pub fn calculate2(input: &str) -> String {
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut count = 0;
    input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            if line.contains('-') {
                let (from, to) = line.split_once('-').unwrap();
                let from = from.parse::<usize>().unwrap();
                let to = to.parse::<usize>().unwrap();
                ranges.push((from, to));
            }
        });

    for i in 0..ranges.len() {
        if i >= ranges.len() {
            break;
        }
        'outer: loop {
            let (from, to) = ranges[i];
            for j in i + 1..ranges.len() {
                let (ff, tt) = ranges[j];
                if (ff >= from && ff <= to) || (tt >= from && tt <= to) || (from >= ff && from <= tt) || (to >= ff && to <= tt) {
                    let from = from.min(ff);
                    let to = to.max(tt);
                    ranges[i] = (from, to);
                    ranges.remove(j);
                    continue 'outer;
                }
            }
            break;
        }
    }
    for (from, to) in ranges {
        count += to - from + 1
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
                3-5
                10-14
                16-20
                12-18

                1
                5
                8
                11
                17
                32
            ",
        );
        assert_eq!(result, "3")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                3-5
                10-14
                16-20
                12-18

                1
                5
                8
                11
                17
                32
            ",
        );
        assert_eq!(result, "14")
    }
}

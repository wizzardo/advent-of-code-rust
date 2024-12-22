use regex::Regex;

pub fn calculate1(input: &str) -> String {
    let button_regex = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let result: i32 = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|line| {
            if !line[0].starts_with("Button A:") {
                panic!();
            }
            let button_a = button_regex.captures(line[0]).unwrap();
            let button_b = button_regex.captures(line[1]).unwrap();
            let prize = prize_regex.captures(line[2]).unwrap();

            let x1 = button_a.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let x2 = button_b.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let x = prize.get(1).unwrap().as_str().parse::<i32>().unwrap();

            let y1 = button_a.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let y2 = button_b.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let y = prize.get(2).unwrap().as_str().parse::<i32>().unwrap();

            // a*x1+b*x2=x
            // a*y1+b*y2=y

            // a*x1=x-b*x2
            // a=(x-b*x2)/x1

            // y1*(x-b*x2)/x1+b*y2=y
            // y1*(x-b*x2)+b*y2*x1=y*x1
            // y1*x-y1*b*x2+b*y2*x1=y*x1
            // b*y2*x1-y1*b*x2=y*x1-y1*x
            // b*(y2*x1-y1*x2)=y*x1-y1*x
            // b=(y*x1-y1*x)/(y2*x1-y1*x2)

            let t1 = y * x1 - y1 * x;
            let t2 = y2 * x1 - y1 * x2;

            if t2 == 0 || t1 % t2 != 0 {
                return 0;
            }
            let b = t1 / t2;
            let t1 = x - b * x2;
            if t1 % x1 != 0 {
                return 0;
            }
            let a = t1 / x1;

            a * 3 + b
        })
        .sum();
    format!("{result}")
}

pub fn calculate2(input: &str) -> String {
    let button_regex = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let result: i64 = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|line| {
            if !line[0].starts_with("Button A:") {
                panic!();
            }
            let button_a = button_regex.captures(line[0]).unwrap();
            let button_b = button_regex.captures(line[1]).unwrap();
            let prize = prize_regex.captures(line[2]).unwrap();

            let x1 = button_a.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let x2 = button_b.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let x = prize.get(1).unwrap().as_str().parse::<i64>().unwrap() + 10000000000000;

            let y1 = button_a.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let y2 = button_b.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let y = prize.get(2).unwrap().as_str().parse::<i64>().unwrap() + 10000000000000;

            // a*x1+b*x2=x
            // a*y1+b*y2=y

            // a*x1=x-b*x2
            // a=(x-b*x2)/x1

            // y1*(x-b*x2)/x1+b*y2=y
            // y1*(x-b*x2)+b*y2*x1=y*x1
            // y1*x-y1*b*x2+b*y2*x1=y*x1
            // b*y2*x1-y1*b*x2=y*x1-y1*x
            // b*(y2*x1-y1*x2)=y*x1-y1*x
            // b=(y*x1-y1*x)/(y2*x1-y1*x2)

            let t1 = y * x1 - y1 * x;
            let t2 = y2 * x1 - y1 * x2;

            if t2 == 0 || t1 % t2 != 0 {
                return 0;
            }
            let b = t1 / t2;
            let t1 = x - b * x2;
            if t1 % x1 != 0 {
                return 0;
            }
            let a = t1 / x1;

            a * 3 + b
        })
        .sum();
    format!("{result}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                Button A: X+94, Y+34
                Button B: X+22, Y+67
                Prize: X=8400, Y=5400

                Button A: X+26, Y+66
                Button B: X+67, Y+21
                Prize: X=12748, Y=12176

                Button A: X+17, Y+86
                Button B: X+84, Y+37
                Prize: X=7870, Y=6450

                Button A: X+69, Y+23
                Button B: X+27, Y+71
                Prize: X=18641, Y=10279
            ",
        );
        assert_eq!(result, "480")
    }
}

use regex::Regex;
use std::{thread, time};
use std::collections::HashSet;

pub fn calculate1(input: &str, width: usize, height: usize) -> String {
    let robot_regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let quads = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let c = robot_regex.captures(line).unwrap();
            let x = c[1].parse::<i32>().unwrap();
            let y = c[2].parse::<i32>().unwrap();
            let vx = c[3].parse::<i32>().unwrap();
            let vy = c[4].parse::<i32>().unwrap();

            let seconds = 100;
            // let seconds = 2;
            let fx = (width as i32 * seconds + x + seconds * vx) % width as i32;
            let fy = (height as i32 * seconds + y + seconds * vy) % height as i32;
            // println!("{} {}", fx, fy);

            let mut quad = [0, 0, 0, 0];
            if fx > width as i32 / 2 && fy < height as i32 / 2 {
                quad[0] = 1
            } else if fx < width as i32 / 2 && fy < height as i32 / 2 {
                quad[1] = 1
            } else if fx < width as i32 / 2 && fy > height as i32 / 2 {
                quad[2] = 1
            } else if fx > width as i32 / 2 && fy > height as i32 / 2 {
                quad[3] = 1
            }
            return quad;
        })
        .reduce(|a, b| [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]])
        .unwrap();
    let result = quads[0] * quads[1] * quads[2] * quads[3];
    format!("{result}")
}

pub fn calculate2(input: &str) -> String {
    let width = 101;
    let height = 103;
    let robot_regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let robots: Vec<[i32; 4]> = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let c = robot_regex.captures(line).unwrap();
            let x = c[1].parse::<i32>().unwrap();
            let y = c[2].parse::<i32>().unwrap();
            let vx = c[3].parse::<i32>().unwrap();
            let vy = c[4].parse::<i32>().unwrap();
            return [x, y, vx, vy];
        })
        .collect();

    let mut grid = ['.'; 101 * 103];

    for i in 0..20000 {
        grid.fill('.');
        for r in &robots {
            let fx = (width * i + r[0] + i * r[2]) % width;
            let fy = (height * i + r[1] + i * r[3]) % height;
            grid[(fx + fy * width) as usize] = '#';
        }

        println!("");
        println!("{i}");
        let mut tree = false;
        for y in 0..height {
            let mut count = 0;
            for x in 0..width {
                let c = grid[(x + y * width) as usize];
                print!("{}", c);
                if c == '#' {
                    count = count + 1;
                    if count > 10 {
                        tree = true;
                    }
                } else {
                    count = 0;
                }
            }
            println!();
        }

        if tree {
            return format!("{i}");
        }
    }

    format!("0")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                p=0,4 v=3,-3
                p=6,3 v=-1,-3
                p=10,3 v=-1,2
                p=2,0 v=2,-1
                p=0,0 v=1,3
                p=3,0 v=-2,-2
                p=7,6 v=-1,-3
                p=3,0 v=-1,-2
                p=9,3 v=2,3
                p=7,3 v=-1,2
                p=2,4 v=2,-3
                p=9,5 v=-3,-3
            ",
            11,
            7,
        );
        assert_eq!(result, "12")
    }
}

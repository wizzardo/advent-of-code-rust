use std::collections::HashSet;
use itertools::Itertools;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Asteroid {
    x: usize,
    y: usize,
}

pub fn calculate1(input: &str) -> String {
    let asteroids: HashSet<Asteroid> = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate()
                .filter(|(_, x)| *x == '#')
                .map(move |(x, _)| Asteroid { x, y })
        }).flatten().collect();


    // for x in asteroids.iter() {
    //     println!("{:?}", x);
    // }

    let max = asteroids.iter()
        .map(|it| {
            let count = asteroids.iter()
                .filter(|other| {
                    let mut dx = (it.x as i32) - (other.x as i32);
                    let mut dy = (it.y as i32) - (other.y as i32);

                    if dx == 0 && dy == 0 {
                        return false;
                    }

                    'outer: loop {
                        let mut min = dx.abs().min(dy.abs());
                        if min == 0 {
                            min = dx.abs().max(dy.abs())
                        }
                        for i in 2..=min {
                            if dx % i == 0 && dy % i == 0 {
                                dx /= i;
                                dy /= i;
                                continue 'outer;
                            }
                        }
                        break;
                    }

                    let mut x = other.x;
                    let mut y = other.y;

                    x = (x as i32 + dx) as usize;
                    y = (y as i32 + dy) as usize;

                    while x != it.x || y != it.y {
                        // if asteroids.iter().find(|a| a.x == x && a.y == y).is_some() {
                        //     return false;
                        // }
                        if asteroids.contains(&Asteroid { x, y }) {
                            return false;
                        }

                        x = (x as i32 + dx) as usize;
                        y = (y as i32 + dy) as usize;
                    }

                    true
                })
                .count();
            // println!("{:?}: {}", it, count);
            count
        })
        .max()
        .unwrap();

    return max.to_string();
}


pub fn calculate2(input: &str) -> String {
    let asteroids: HashSet<Asteroid> = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate()
                .filter(|(_, x)| *x == '#')
                .map(move |(x, _)| Asteroid { x, y })
        })
        .flatten()
        .collect();


    let (base_x, base_y, _count) = asteroids.iter()
        .map(|it| {
            let count = asteroids.iter()
                .filter(|other| {
                    let mut dx = (it.x as i32) - (other.x as i32);
                    let mut dy = (it.y as i32) - (other.y as i32);

                    if dx == 0 && dy == 0 {
                        return false;
                    }

                    'outer: loop {
                        let mut min = dx.abs().min(dy.abs());
                        if min == 0 {
                            min = dx.abs().max(dy.abs())
                        }
                        for i in 2..=min {
                            if dx % i == 0 && dy % i == 0 {
                                dx /= i;
                                dy /= i;
                                continue 'outer;
                            }
                        }
                        break;
                    }

                    let mut x = other.x;
                    let mut y = other.y;

                    x = (x as i32 + dx) as usize;
                    y = (y as i32 + dy) as usize;

                    while x != it.x || y != it.y {
                        if asteroids.contains(&Asteroid { x, y }) {
                            return false;
                        }

                        x = (x as i32 + dx) as usize;
                        y = (y as i32 + dy) as usize;
                    }

                    true
                })
                .count();
            // println!("{:?}: {}", it, count);
            (it.x, it.y, count)
        })
        .max_by(|a, b| a.2.cmp(&b.2))
        .unwrap();

    let mut asts: Vec<(f64, f64, &Asteroid)> = asteroids.iter()
        .filter(|it| it.x != base_x || it.y != base_y)
        .map(|it| {
            let dx = (it.x as f64) - (base_x as f64);
            let dy = (it.y as f64) - (base_y as f64);
            let r = (dx * dx + dy * dy).sqrt();
            let a = dy.atan2(dx);
            (a, r, it)
        })
        .sorted_by(|a, b| {
            if a.0 == b.0 {
                a.1.partial_cmp(&b.1).unwrap()
            } else {
                a.0.partial_cmp(&b.0).unwrap()
            }
        })
        .collect();

    let start_angle = -1.0_f64.atan2(0.0_f64);
    // let next_angle = -10.0_f64.atan2(1.0_f64);

    let mut cursor = 0;
    for i in 0..asts.len() {
        if asts[i].0 < start_angle {
            cursor += 1;
        } else {
            break;
        }
    }

    let mut counter = 0;
    let mut prev: f64 = 0.0;
    loop {
        if asts[cursor].0 != prev {
            counter += 1;
            let ast = asts.remove(cursor);
            // println!("{}: {:?}", counter, ast);
            prev = ast.0;
            if counter == 200 {
                return (ast.2.x * 100 + ast.2.y).to_string();
            }
        } else {
            cursor += 1;
        }
        if cursor >= asts.len() {
            cursor = 0;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1_1() {
        let result = calculate1("
                .#..#
                .....
                #####
                ....#
                ...##
            "
        );
        assert_eq!(result, "8")
    }

    #[test]
    fn test_1_2() {
        let result = calculate1("
                ......#.#.
                #..#.#....
                ..#######.
                .#.#.###..
                .#..#.....
                ..#....#.#
                #..#....#.
                .##.#..###
                ##...#..#.
                .#....####
            "
        );
        assert_eq!(result, "33")
    }

    #[test]
    fn test_1_3() {
        let result = calculate1("
                #.#...#.#.
                .###....#.
                .#....#...
                ##.#.#.#.#
                ....#.#.#.
                .##..###.#
                ..#...##..
                ..##....##
                ......#...
                .####.###.
            "
        );
        assert_eq!(result, "35")
    }

    #[test]
    fn test_1_4() {
        let result = calculate1("
                .#..#..###
                ####.###.#
                ....###.#.
                ..###.##.#
                ##.##.#.#.
                ....###..#
                ..#.#..#.#
                #..#.#.###
                .##...##.#
                .....#.#..
            "
        );
        assert_eq!(result, "41")
    }

    #[test]
    fn test_1_5() {
        let result = calculate1("
                .#..##.###...#######
                ##.############..##.
                .#.######.########.#
                .###.#######.####.#.
                #####.##.#.##.###.##
                ..#####..#.#########
                ####################
                #.####....###.#.#.##
                ##.#################
                #####.##.###..####..
                ..######..##.#######
                ####.##.####...##..#
                .#####..#.######.###
                ##...#.##########...
                #.##########.#######
                .####.#.###.###.#.##
                ....##.##.###..#####
                .#.#.###########.###
                #.#.#.#####.####.###
                ###.##.####.##.#..##
            "
        );
        assert_eq!(result, "210")
    }

    #[test]
    fn test_2_1() {
        let result = calculate2("
                .#..##.###...#######
                ##.############..##.
                .#.######.########.#
                .###.#######.####.#.
                #####.##.#.##.###.##
                ..#####..#.#########
                ####################
                #.####....###.#.#.##
                ##.#################
                #####.##.###..####..
                ..######..##.#######
                ####.##.####...##..#
                .#####..#.######.###
                ##...#.##########...
                #.##########.#######
                .####.#.###.###.#.##
                ....##.##.###..#####
                .#.#.###########.###
                #.#.#.#####.####.###
                ###.##.####.##.#..##
            "
        );
        assert_eq!(result, "802")
    }
}
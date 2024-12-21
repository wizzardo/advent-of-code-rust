use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    pub fn all_directions() -> [Vector; 4] {
        [
            Vector { x: 1, y: 0 },
            Vector { x: 0, y: -1 },
            Vector { x: -1, y: 0 },
            Vector { x: 0, y: 1 },
        ]
    }
}

pub fn calculate1(input: &str) -> String {
    let data: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| line.chars().map(|it| it as u8 - '0' as u8).collect())
        .collect();

    let width = data[0].len();
    let height = data.len();


    let data_ref = &data;
    let do_match: Box<dyn Fn(u8, i32, i32) -> bool> = Box::new(|elevation: u8, x: i32, y: i32| {
        if x < 0 || y < 0 || x as usize == width || y as usize == height {
            return false;
        }
        return data_ref[y as usize][x as usize] == elevation;
    });

    let mut trailheads: HashSet<(i32, i32, i32, i32)> = HashSet::new();

    for y in 0..height {
        for x in 0..width {
            if data[y][x] == 0 {
                do_search(&mut trailheads, 0, x as i32, y as i32, x as i32, y as i32, &do_match);
            }
        }
    }

    let result = trailheads.len();
    format!("{result}")
}

fn do_search(trailheads: &mut HashSet<(i32, i32, i32, i32)>, elevation: u8, x: i32, y: i32, from_x: i32, from_y: i32, check_elevation: &dyn Fn(u8, i32, i32) -> bool) {
    if elevation == 9 {
        trailheads.insert((from_x, from_y, x, y));
        return;
    }

    for v in Vector::all_directions() {
        if check_elevation(elevation + 1, x + v.x, y + v.y) {
            do_search(trailheads, elevation + 1, x + v.x, y + v.y, from_x, from_y, check_elevation);
        }
    }
}

pub fn calculate2(input: &str) -> String {
    let data: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| line.chars().map(|it| it as u8 - '0' as u8).collect())
        .collect();

    let width = data[0].len();
    let height = data.len();


    let data_ref = &data;
    let do_match: Box<dyn Fn(u8, i32, i32) -> bool> = Box::new(|elevation: u8, x: i32, y: i32| {
        if x < 0 || y < 0 || x as usize == width || y as usize == height {
            return false;
        }
        return data_ref[y as usize][x as usize] == elevation;
    });

    let mut total_rating = 0;

    for y in 0..height {
        for x in 0..width {
            if data[y][x] == 0 {
                total_rating += do_search_2(0, x as i32, y as i32, x as i32, y as i32, &do_match);
            }
        }
    }

    let result = total_rating;
    format!("{result}")
}

fn do_search_2(elevation: u8, x: i32, y: i32, from_x: i32, from_y: i32, check_elevation: &dyn Fn(u8, i32, i32) -> bool) -> i32 {
    if elevation == 9 {
        return 1;
    }

    let mut total_rating = 0;
    for v in Vector::all_directions() {
        if check_elevation(elevation + 1, x + v.x, y + v.y) {
            total_rating += do_search_2(elevation + 1, x + v.x, y + v.y, from_x, from_y, check_elevation);
        }
    }
    total_rating
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                89010123
                78121874
                87430965
                96549874
                45678903
                32019012
                01329801
                10456732
            ",
        );
        assert_eq!(result, "36")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                89010123
                78121874
                87430965
                96549874
                45678903
                32019012
                01329801
                10456732
            ",
        );
        assert_eq!(result, "81")
    }
}

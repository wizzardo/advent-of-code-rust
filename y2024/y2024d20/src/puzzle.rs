
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

    pub fn distance(&self, other: Vector) -> u32 {
        (self.x - other.x).abs() as u32 + (self.y - other.y).abs() as u32
    }

}

pub fn calculate1(input: &str, save_threshold: u32) -> String {
    let mut data: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| line.chars().collect())
        .collect();

    let width = data[0].len();
    let height = data.len();

    let mut start = Vector { x: -1, y: -1 };
    let mut end = Vector { x: -1, y: -1 };

    'outer: for y in 0..height {
        for x in 0..width {
            if data[y][x] == 'S' {
                start.x = x as i32;
                start.y = y as i32;
                data[y][x] = '.';
                if end.x != -1 { break 'outer; }
            }
            if data[y][x] == 'E' {
                end.x = x as i32;
                end.y = y as i32;
                data[y][x] = '.';
                if start.x != -1 { break 'outer; }
            }
        }
    }

    let mut step_counter: u32 = 0;
    let mut prev = start;
    let mut current = start;

    fn u32_as_char(c: u32) -> char {
        unsafe { char::from_u32_unchecked(c + ('0' as u32)) }
    }

    fn char_to_u32(c: char) -> u32 {
        (c as u32) - ('0' as u32)
    }

    data[start.y as usize][start.x as usize] = u32_as_char(step_counter);

    let directions = Vector::all_directions();
    'outer: loop {
        for d in &directions {
            let next = Vector { x: current.x + d.x, y: current.y + d.y };
            if next == prev || next.x < 0 || next.y < 0 || next.x as usize == width || next.y as usize == height || data[next.y as usize][next.x as usize] == '#' {
                continue;
            }

            prev = current;
            step_counter += 1;
            current = next;
            data[next.y as usize][next.x as usize] = u32_as_char(step_counter);

            if next == end {
                break 'outer;
            }

            break;
        }
    }

    let mut counter = 0;
    let mut prev = start;
    let mut current = start;
    'outer: loop {
        let current_steps = char_to_u32(data[current.y as usize][current.x as usize]);
        for d in &directions {
            let next = Vector { x: current.x + d.x * 2, y: current.y + d.y * 2 };
            if next.x < 0 || next.y < 0 || next.x as usize >= width || next.y as usize >= height || data[next.y as usize][next.x as usize] == '#' {
                continue;
            }

            let cheat_steps = char_to_u32(data[next.y as usize][next.x as usize]);
            if cheat_steps > current_steps && cheat_steps - current_steps >= save_threshold + 2 {
                counter += 1;
            }
        }
        for d in &directions {
            let next = Vector { x: current.x + d.x, y: current.y + d.y };
            if next == prev || next.x < 0 || next.y < 0 || next.x as usize == width || next.y as usize == height || data[next.y as usize][next.x as usize] == '#' {
                continue;
            }

            prev = current;
            step_counter += 1;
            current = next;

            if next == end {
                break 'outer;
            }

            break;
        }
    }

    let result = counter;
    format!("{result}")
}

pub fn calculate2(input: &str, save_threshold: u32) -> String {
    let mut data: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| line.chars().collect())
        .collect();

    let width = data[0].len();
    let height = data.len();

    let mut start = Vector { x: -1, y: -1 };
    let mut end = Vector { x: -1, y: -1 };

    'outer: for y in 0..height {
        for x in 0..width {
            if data[y][x] == 'S' {
                start.x = x as i32;
                start.y = y as i32;
                data[y][x] = '.';
                if end.x != -1 { break 'outer; }
            }
            if data[y][x] == 'E' {
                end.x = x as i32;
                end.y = y as i32;
                data[y][x] = '.';
                if start.x != -1 { break 'outer; }
            }
        }
    }

    let mut step_counter: u32 = 0;
    let mut prev = start;
    let mut current = start;

    fn u32_as_char(c: u32) -> char {
        unsafe { char::from_u32_unchecked(c + ('0' as u32)) }
    }

    fn char_to_u32(c: char) -> u32 {
        (c as u32) - ('0' as u32)
    }

    data[start.y as usize][start.x as usize] = u32_as_char(step_counter);

    let directions = Vector::all_directions();
    'outer: loop {
        for d in &directions {
            let next = Vector { x: current.x + d.x, y: current.y + d.y };
            if next == prev || next.x < 0 || next.y < 0 || next.x as usize == width || next.y as usize == height || data[next.y as usize][next.x as usize] == '#' {
                continue;
            }

            prev = current;
            step_counter += 1;
            current = next;
            data[next.y as usize][next.x as usize] = u32_as_char(step_counter);

            if next == end {
                break 'outer;
            }

            break;
        }
    }

    let mut counter = 0;
    let mut prev = start;
    let mut current = start;
    'outer: loop {
        let current_steps = char_to_u32(data[current.y as usize][current.x as usize]);
        let mut next = Vector { x: current.x, y: current.y };
        for y in -20..21 {
            next.y = current.y + y;
            if next.y < 0 || next.y >= height as i32 {
                continue;
            }
            let d = 20 - y.abs();
            for x in -d..d + 1 {
                next.x = current.x + x;
                if next.x < 0 || next.x  >= width as i32 || data[next.y as usize][next.x as usize] == '#' {
                    continue;
                }

                let cheat_steps = char_to_u32(data[next.y as usize][next.x as usize]);
                if cheat_steps > current_steps && cheat_steps - current_steps >= save_threshold + current.distance(next) {
                    counter += 1;
                }
            }
        }
        for d in &directions {
            let next = Vector { x: current.x + d.x, y: current.y + d.y };
            if next == prev || next.x < 0 || next.y < 0 || next.x as usize == width || next.y as usize == height || data[next.y as usize][next.x as usize] == '#' {
                continue;
            }

            prev = current;
            step_counter += 1;
            current = next;

            if next == end {
                break 'outer;
            }

            break;
        }
    }

    let result = counter;
    format!("{result}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let input = "
                ###############
                #...#...#.....#
                #.#.#.#.#.###.#
                #S#...#.#.#...#
                #######.#.#.###
                #######.#.#...#
                #######.#.###.#
                ###..E#...#...#
                ###.#######.###
                #...###...#...#
                #.#####.#.###.#
                #.#...#.#.#...#
                #.#.#.#.#.#.###
                #...#...#...###
                ###############
            ";
        assert_eq!(calculate1(input, 64), "1");
        assert_eq!(calculate1(input, 40), "2");
        assert_eq!(calculate1(input, 38), "3");
        assert_eq!(calculate1(input, 36), "4");
        assert_eq!(calculate1(input, 20), "5");
        assert_eq!(calculate1(input, 12), "8");
        assert_eq!(calculate1(input, 10), "10");
        assert_eq!(calculate1(input, 8), "14");
        assert_eq!(calculate1(input, 6), "16");
        assert_eq!(calculate1(input, 4), "30");
        assert_eq!(calculate1(input, 2), "44");
    }

    #[test]
    fn test_2() {
        let input = "
                ###############
                #...#...#.....#
                #.#.#.#.#.###.#
                #S#...#.#.#...#
                #######.#.#.###
                #######.#.#...#
                #######.#.###.#
                ###..E#...#...#
                ###.#######.###
                #...###...#...#
                #.#####.#.###.#
                #.#...#.#.#...#
                #.#.#.#.#.#.###
                #...#...#...###
                ###############
            ";
        assert_eq!(calculate2(input, 76), "3");
        assert_eq!(calculate2(input, 74), "7");
        assert_eq!(calculate2(input, 72), "29");
        assert_eq!(calculate2(input, 70), "41");
        assert_eq!(calculate2(input, 68), "55");
        assert_eq!(calculate2(input, 66), "67");
        assert_eq!(calculate2(input, 64), "86");
        assert_eq!(calculate2(input, 62), "106");
        assert_eq!(calculate2(input, 60), "129");
        assert_eq!(calculate2(input, 58), "154");
        assert_eq!(calculate2(input, 56), "193");
        assert_eq!(calculate2(input, 54), "222");
        assert_eq!(calculate2(input, 52), "253");
        assert_eq!(calculate2(input, 50), "285");
    }
}

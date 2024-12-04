#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    pub fn all_directions() -> [Vector; 8] {
        [
            Vector { x: 1, y: 0 },
            Vector { x: 1, y: -1 },
            Vector { x: 0, y: -1 },
            Vector { x: -1, y: -1 },
            Vector { x: -1, y: 0 },
            Vector { x: -1, y: 1 },
            Vector { x: 0, y: 1 },
            Vector { x: 1, y: 1 },
        ]
    }
    pub fn cross_directions() -> [Vector; 2] {
        [Vector { x: -1, y: 1 }, Vector { x: 1, y: 1 }]
    }
}

pub fn calculate1(input: &str) -> String {
    let data: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| line.chars().collect())
        .collect();

    let width = data[0].len();
    let height = data.len();

    let search = ['X', 'M', 'A', 'S'];
    let directions = Vector::all_directions();

    let mut count = 0;

    let do_match =
        |data: &Vec<Vec<char>>, search: char, x: i32, y: i32, width: usize, height: usize| {
            if x < 0 || y < 0 || x as usize == width || y as usize == height {
                return false;
            }
            return data[y as usize][x as usize] == search;
        };

    for y in 0..height {
        for x in 0..width {
            if data[y][x] == search[0] {
                'outer: for d in &directions {
                    let mut x = x as i32;
                    let mut y = y as i32;
                    for i in 1..search.len() {
                        x += d.x;
                        y += d.y;
                        if !do_match(&data, search[i], x, y, width, height) {
                            continue 'outer;
                        }
                    }
                    count += 1;
                }
            }
        }
    }

    format!("{count}")
}

pub fn calculate2(input: &str) -> String {
    let data: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| line.chars().collect())
        .collect();

    let width = data[0].len();
    let height = data.len();

    let directions = Vector::cross_directions();

    let mut count = 0;

    let data_ref = &data;
    let do_match = |search: char, x: i32, y: i32| {
        if x < 0 || y < 0 || x as usize == width || y as usize == height {
            return false;
        }
        return data_ref[y as usize][x as usize] == search;
    };

    for y in 0..height as i32 {
        'outer: for x in 0..width as i32 {
            if data[y as usize][x as usize] == 'A' {
                for d in &directions {
                    if !((do_match('M', x - d.x, y - d.y) && do_match('S', x + d.x, y + d.y))
                        || (do_match('S', x - d.x, y - d.y) && do_match('M', x + d.x, y + d.y))
                    ) {
                        continue 'outer;
                    }
                }

                count += 1;
            }
        }
    }

    format!("{count}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                MMMSXXMASM
                MSAMXMSMSA
                AMXSXMAAMM
                MSAMASMSMX
                XMASAMXAMM
                XXAMMXXAMA
                SMSMSASXSS
                SAXAMASAAA
                MAMMMXMMMM
                MXMXAXMASX
            ",
        );
        assert_eq!(result, "18")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                MMMSXXMASM
                MSAMXMSMSA
                AMXSXMAAMM
                MSAMASMSMX
                XMASAMXAMM
                XXAMMXXAMA
                SMSMSASXSS
                SAXAMASAAA
                MAMMMXMMMM
                MXMXAXMASX
            ",
        );
        assert_eq!(result, "9")
    }
}

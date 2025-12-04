pub fn calculate1(input: &str) -> String {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|it| it.chars().collect())
        .collect();

    let mut count = 0;
    let width = grid[0].len();
    let height = grid.len();

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == '@' && is_less_than_4_neighbors(&grid, width, height, x, y) {
                count += 1;
            }
        }
    }
    let result = count;
    format!("{result}")
}

pub fn calculate2(input: &str) -> String {
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|it| it.chars().collect())
        .collect();

    let mut count = 0;
    let width = grid[0].len();
    let height = grid.len();

    loop {
        let mut removed = false;
        for y in 0..height {
            for x in 0..width {
                if grid[y][x] == '@' && crate::puzzle::is_less_than_4_neighbors(&grid, width, height, x, y) {
                    count += 1;
                    grid[y][x] = '.';
                    removed = true;
                }
            }
        }
        if !removed {
            break;
        }
    }
    let result = count;
    format!("{result}")
}

fn is_less_than_4_neighbors(grid: &Vec<Vec<char>>, width: usize, height: usize, x: usize, y: usize) -> bool {
    let x: i32 = x as i32;
    let y: i32 = y as i32;
    let mut count = 0;
    for yy in y - 1..=y + 1 {
        if yy < 0 || yy >= height as i32 {
            continue;
        }
        for xx in x - 1..=x + 1 {
            if xx < 0 || xx >= width as i32 {
                continue;
            }
            if grid[yy as usize][xx as usize] == '@' {
                count += 1;
            }
        }
    }
    count <= 4
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                ..@@.@@@@.
                @@@.@.@.@@
                @@@@@.@.@@
                @.@@@@..@.
                @@.@@@@.@@
                .@@@@@@@.@
                .@.@.@.@@@
                @.@@@.@@@@
                .@@@@@@@@.
                @.@.@@@.@.
            ",
        );
        assert_eq!(result, "13")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                ..@@.@@@@.
                @@@.@.@.@@
                @@@@@.@.@@
                @.@@@@..@.
                @@.@@@@.@@
                .@@@@@@@.@
                .@.@.@.@@@
                @.@@@.@@@@
                .@@@@@@@@.
                @.@.@@@.@.
            ",
        );
        assert_eq!(result, "43")
    }
}

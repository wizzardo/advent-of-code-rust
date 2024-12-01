use common::{char_at, find_substring};

pub fn calculate1(input: &str) -> String {
    let pipes: Vec<&str> = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            line
        })
        // .inspect(|line| { dbg!(line); })
        .collect();

    let start = pipes.iter()
        .enumerate()
        .map(|(i, line)| find_substring(line, "S").map(|j| (i, j)))
        .filter(|x| x.is_some())
        .take(1)
        .map(|it| it.unwrap())
        .last();

    if start.is_none() {
        panic!("start not found")
    }

    let mut length = 0;
    let start = start.unwrap();
    let mut position: (usize, usize) = start;
    let mut prev: (usize, usize) = start;

    loop {
        let next = find_next_step(&pipes, &position, &prev);
        if next.is_none() {
            panic!("cannot find next step");
        }

        length += 1;
        prev = position;
        position = next.unwrap();
        if position == start {
            break;
        }
    }


    return (length / 2).to_string();
}

fn find_next_step(pipes: &Vec<&str>, position: &(usize, usize), prev: &(usize, usize)) -> Option<(usize, usize)> {
    let (y, x) = position;
    let y = *y;
    let x = *x;

    if y > 0 && prev.0 != y - 1 && is_connected(pipes, x, y, x, y - 1) {
        return Some((y - 1, x));
    }
    if y < pipes.len() - 1 && prev.0 != y + 1 && is_connected(pipes, x, y, x, y + 1) {
        return Some((y + 1, x));
    }
    if x > 0 && prev.1 != x - 1 && is_connected(pipes, x, y, x - 1, y) {
        return Some((y, x - 1));
    }
    if x < pipes.get(y).unwrap().len() - 1 && prev.1 != x + 1 && is_connected(pipes, x, y, x + 1, y) {
        return Some((y, x + 1));
    }

    return None;
}

fn is_connected(pipes: &Vec<&str>, x_a: usize, y_a: usize, x_b: usize, y_b: usize) -> bool {
    let line_a = pipes.get(y_a);
    if line_a.is_none() {
        return false;
    }

    let line_b = pipes.get(y_b);
    if line_b.is_none() {
        return false;
    }

    let line_a = *line_a.unwrap();
    let line_b = *line_b.unwrap();

    let pipe_a = char_at(line_a, x_a);
    if pipe_a.is_none() {
        return false;
    }
    let pipe_b = char_at(line_b, x_b);
    if pipe_b.is_none() {
        return false;
    }

    let a = pipe_a.unwrap();
    let b = pipe_b.unwrap();

    if x_a < x_b && (a == '-' || a == 'L' || a == 'F' || a == 'S') && (b == '7' || b == '-' || b == 'J' || b == 'S') {
        return true;
    }
    if x_a > x_b && (a == '-' || a == 'J' || a == '7' || a == 'S') && (b == 'L' || b == '-' || b == 'F' || b == 'S') {
        return true;
    }

    if y_a < y_b && (a == '|' || a == '7' || a == 'F' || a == 'S') && (b == '|' || b == 'L' || b == 'J' || b == 'S') {
        return true;
    }
    if y_a > y_b && (a == '|' || a == 'L' || a == 'J' || a == 'S') && (b == '|' || b == '7' || b == 'F' || b == 'S') {
        return true;
    }

    return false;
}


pub fn calculate2(input: &str) -> String {
    let pipes: Vec<&str> = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            line
        })
        // .inspect(|line| { dbg!(line); })
        .collect();

    let start = pipes.iter()
        .enumerate()
        .map(|(i, line)| find_substring(line, "S").map(|j| (i, j)))
        .filter(|x| x.is_some())
        .take(1)
        .map(|it| it.unwrap())
        .last();

    if start.is_none() {
        panic!("start not found")
    }

    let start = start.unwrap();
    let mut position: (usize, usize) = start;
    let mut prev: (usize, usize) = start;

    let height = pipes.len() * 3;
    let width = pipes.get(0).unwrap().len() * 3;
    let mut mask_vec: Vec<u8> = Vec::with_capacity(height * width);
    for _i in 0..width * height {
        mask_vec.push(0);
    }
    let mut mask = mask_vec.as_mut_slice();
    // 0 - unchecked
    // 1 - pipe
    // 2 - checked, outside
    // 3 - checked, inside

    loop {
        let next = find_next_step(&pipes, &position, &prev);
        if next.is_none() {
            panic!("cannot find next step");
        }

        prev = position;
        position = next.unwrap();


        let p = char_at(pipes.get(position.0).unwrap(), position.1).unwrap();
        if p == '-' {
            mask[(position.0 * 3 + 1) * width + position.1 * 3 + 0] = 1;
            mask[(position.0 * 3 + 1) * width + position.1 * 3 + 1] = 1;
            mask[(position.0 * 3 + 1) * width + position.1 * 3 + 2] = 1;
        } else if p == '|' {
            mask[(position.0 * 3 + 0) * width + position.1 * 3 + 1] = 1;
            mask[(position.0 * 3 + 1) * width + position.1 * 3 + 1] = 1;
            mask[(position.0 * 3 + 2) * width + position.1 * 3 + 1] = 1;
        } else if p == 'J' {
            mask[(position.0 * 3 + 0) * width + position.1 * 3 + 1] = 1;
            mask[(position.0 * 3 + 1) * width + position.1 * 3 + 1] = 1;
            mask[(position.0 * 3 + 1) * width + position.1 * 3 + 0] = 1;
        } else if p == '7' {
            mask[(position.0 * 3 + 1) * width + position.1 * 3 + 0] = 1;
            mask[(position.0 * 3 + 1) * width + position.1 * 3 + 1] = 1;
            mask[(position.0 * 3 + 2) * width + position.1 * 3 + 1] = 1;
        } else if p == 'L' {
            mask[(position.0 * 3 + 0) * width + position.1 * 3 + 1] = 1;
            mask[(position.0 * 3 + 1) * width + position.1 * 3 + 1] = 1;
            mask[(position.0 * 3 + 1) * width + position.1 * 3 + 2] = 1;
        } else if p == 'F' {
            mask[(position.0 * 3 + 1) * width + position.1 * 3 + 2] = 1;
            mask[(position.0 * 3 + 1) * width + position.1 * 3 + 1] = 1;
            mask[(position.0 * 3 + 2) * width + position.1 * 3 + 1] = 1;
        } else if p == 'S' {
            mask[(position.0 * 3 + 1) * width + position.1 * 3 + 1] = 1;
            if prev.1 < position.1 {
                mask[(position.0 * 3 + 1) * width + position.1 * 3 + 0] = 1;
            } else if prev.1 > position.1 {
                mask[(position.0 * 3 + 1) * width + position.1 * 3 + 2] = 1;
            }else if prev.0 < position.0 {
                mask[(position.0 * 3 + 0) * width + position.1 * 3 + 1] = 1;
            } else if prev.0 > position.0 {
                mask[(position.0 * 3 + 2) * width + position.1 * 3 + 1] = 1;
            }
            let next = find_next_step(&pipes, &position, &prev).unwrap();
            if next.1 < position.1 {
                mask[(position.0 * 3 + 1) * width + position.1 * 3 + 0] = 1;
            } else if next.1 > position.1 {
                mask[(position.0 * 3 + 1) * width + position.1 * 3 + 2] = 1;
            }else if next.0 < position.0 {
                mask[(position.0 * 3 + 0) * width + position.1 * 3 + 1] = 1;
            } else if next.0 > position.0 {
                mask[(position.0 * 3 + 2) * width + position.1 * 3 + 1] = 1;
            }
        }

        if position == start {
            break;
        }
    }

    for y in 0..height / 3 {
        println!("{}", pipes.get(y).unwrap())
    }
    print_mask(height, width, mask);

    for x in 0..width {
        mark_simple(&mut mask, x, 0, width, height);
        mark_simple(&mut mask, x, height - 1, width, height);
    }
    for y in 0..height {
        mark_simple(&mut mask, 0, y, width, height);
        mark_simple(&mut mask, width - 1, y, width, height);
    }

    print_mask(height, width, mask);

    let mut count = 0;
    for y in (0..height).step_by(3) {
        for x in (0..width).step_by(3) {
            if mask[(y+1)*width+x+1]==0{
                count+=1;
            }
        }
    }

    return count.to_string();
}

fn mark_simple(mask: &mut [u8], x: usize, y: usize, width: usize, height: usize) {
    if x >= width || y >= height {
        return;
    }
    if mask[y * width + x] != 0 {
        return;
    }

    if x == 0 || x > 0 && mask[y * width + x - 1] == 2 {
        mask[y * width + x] = 2;
    } else if x == width - 1 || x < width - 1 && mask[y * width + x + 1] == 2 {
        mask[y * width + x] = 2;
    } else if y == 0 || y > 0 && mask[(y - 1) * width + x] == 2 {
        mask[y * width + x] = 2;
    } else if y == height - 1 || y < height - 1 && mask[(y + 1) * width + x] == 2 {
        mask[y * width + x] = 2;
    }
    // else if mask[(y + 1) * width + x + 1] == 2 {
    //     mask[y * width + x] = 2;
    // } else if mask[(y + 1) * width + x - 1] == 2 {
    //     mask[y * width + x] = 2;
    // } else if mask[(y - 1) * width + x + 1] == 2 {
    //     mask[y * width + x] = 2;
    // } else if mask[(y - 1) * width + x - 1] == 2 {
    //     mask[y * width + x] = 2;
    // }

    if x < width - 1 {
        mark_simple(mask, x + 1, y, width, height);
        // mark_simple(pipes, mask, x + 1, y + 1, width, height);
        // if y > 0 {
        //     mark_simple(pipes, mask, x + 1, y - 1, width, height);
        // }
    }
    if x > 0 {
        mark_simple(mask, x - 1, y, width, height);
        // mark_simple(pipes, mask, x - 1, y + 1, width, height);
        // if y > 0 {
        //     mark_simple(pipes, mask, x - 1, y - 1, width, height);
        // }
    }
    if y < height - 1 {
        mark_simple(mask, x, y + 1, width, height)
    }
    if y > 0 {
        mark_simple(mask, x, y - 1, width, height)
    }
}

fn print_mask(height: usize, width: usize, mask: &mut [u8]) {
    println!("");
    for y in 0..height {
        for x in 0..width {
            print!("{}", mask[y * width + x])
        }
        println!("")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1("
                -L|F7
                7S-7|
                L|7||
                -L-J|
                L|-JF
            "
        );
        assert_eq!(result, "4")
    }

    #[test]
    fn test_1_2() {
        let result = calculate1("
                7-F7-
                .FJ|7
                SJLL7
                |F--J
                LJ.LJ
            "
        );
        assert_eq!(result, "8")
    }


    #[test]
    fn test_2() {
        let result = calculate2("
                ..........
                .S------7.
                .|F----7|.
                .||OOOO||.
                .||OOOO||.
                .|L-7F-J|.
                .|II||II|.
                .L--JL--J.
                ..........
            "
        );
        assert_eq!(result, "4")
    }

    #[test]
    fn test_2_1() {
        let result = calculate2("
                ...........
                .S-------7.
                .|F-----7|.
                .||OOOOO||.
                .||OOOOO||.
                .|L-7OF-J|.
                .|II|O|II|.
                .L--JOL--J.
                .....O.....
            "
        );
        assert_eq!(result, "4")
    }

    #[test]
    fn test_2_2() {
        let result = calculate2("
                OF----7F7F7F7F-7OOOO
                O|F--7||||||||FJOOOO
                O||OFJ||||||||L7OOOO
                FJL7L7LJLJ||LJIL-7OO
                L--JOL7IIILJS7F-7L7O
                OOOOF-JIIF7FJ|L7L7L7
                OOOOL7IF7||L7|IL7L7|
                OOOOO|FJLJ|FJ|F7|OLJ
                OOOOFJL-7O||O||||OOO
                OOOOL---JOLJOLJLJOOO
            "
        );
        assert_eq!(result, "8")
    }
}
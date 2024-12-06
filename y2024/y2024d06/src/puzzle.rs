use std::collections::{HashMap, HashSet};

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn right(&self) -> Vector {
        if self.x == 1 {
            return Vector { x: 0, y: 1 };
        }
        if self.x == -1 {
            return Vector { x: 0, y: -1 };
        }
        if self.y == 1 {
            return Vector { x: -1, y: 0 };
        }
        if self.y == -1 {
            return Vector { x: 1, y: 0 };
        }

        panic!()
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
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

    let mut position: Point = Point { x: 0, y: 0 };

    'outer: for y in 0..height {
        for x in 0..width {
            if data[y][x] == '^' {
                position.x = x as i32;
                position.y = y as i32;
                break 'outer;
            }
        }
    }

    let mut direction = Vector { x: 0, y: -1 };
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(position.clone());

    #[allow(unused)]
    let print_map = |visited: &HashSet<Point>| {
        for y in 0..height {
            for x in 0..width {
                if visited.contains(&Point {
                    x: x as i32,
                    y: y as i32,
                }) {
                    print!("X")
                } else {
                    print!("{}", data[y][x])
                }
            }
            println!("");
        }
        println!("");
    };

    loop {
        let mut next = Point {
            x: position.x + direction.x,
            y: position.y + direction.y,
        };
        if next.x < 0 || next.y < 0 || next.x == width as i32 || next.y == height as i32 {
            // print_map(&visited);
            break;
        }

        if data[next.y as usize][next.x as usize] == '#' {
            direction = direction.right();
            next = Point {
                x: position.x + direction.x,
                y: position.y + direction.y,
            };
            // print_map(&visited);
        }

        if next.x < 0 || next.y < 0 || next.x == width as i32 || next.y == height as i32 {
            // print_map(&visited);
            break;
        }

        visited.insert(next.clone());
        position = next;
    }

    // println!("exit at {:?}", position);

    let result = visited.len();
    format!("{result}")
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

    let mut start: Point = Point { x: -1, y: -1 };

    'outer: for y in 0..height {
        for x in 0..width {
            if data[y][x] == '^' {
                start.x = x as i32;
                start.y = y as i32;
                break 'outer;
            }
        }
    }

    #[allow(unused)]
    let print_map = |visited: &HashMap<Point, Vec<Vector>>| {
        for y in 0..height {
            for x in 0..width {
                if visited.contains_key(&Point {
                    x: x as i32,
                    y: y as i32,
                }) {
                    print!("X")
                } else {
                    print!("{}", data[y][x])
                }
            }
            println!("");
        }
        println!("");
    };
    let mut count = 0;

    // for j in 0..height {
    //     for i in 0..width {
    //         if data[j][i] == '#' || data[j][i] == '^' {
    //             continue;
    //         }
    //
    //         // println!("{}/{}", j * width + i, width * height);
    //         let mut position = start.clone();
    //
    //         let mut direction = Vector { x: 0, y: -1 };
    //         let mut visited: HashMap<Point, Vec<Vector>> = HashMap::new();
    //         visited.insert(position.clone(), vec![direction.clone()]);
    //
    //         let block = Point {
    //             x: i as i32,
    //             y: j as i32,
    //         };
    //
    //         loop {
    //             let mut next = Point {
    //                 x: position.x + direction.x,
    //                 y: position.y + direction.y,
    //             };
    //             if next.x < 0 || next.y < 0 || next.x == width as i32 || next.y == height as i32 {
    //                 // print_map(&visited);
    //                 break;
    //             }
    //
    //             if next == block || data[next.y as usize][next.x as usize] == '#' {
    //                 direction = direction.right();
    //                 visited.get_mut(&position).unwrap().push(direction.clone());
    //                 // next = Point {
    //                 //     x: position.x + direction.x,
    //                 //     y: position.y + direction.y,
    //                 // };
    //                 // print_map(&visited);
    //                 continue;
    //             }
    //
    //             if next.x < 0 || next.y < 0 || next.x == width as i32 || next.y == height as i32 {
    //                 // print_map(&visited);
    //                 break;
    //             }
    //
    //             position = next;
    //
    //             let dirs = visited.entry(next.clone()).or_insert_with(Vec::new);
    //             if dirs.contains(&direction) {
    //                 count += 1;
    //                 println!("Point{{x:{},y:{}}},", block.x, block.y);
    //                 break;
    //             }
    //             dirs.push(direction.clone());
    //         }
    //     }
    // }

    let mut position = start.clone();
    let mut direction = Vector { x: 0, y: -1 };
    let mut visited: HashMap<Point, Vec<Vector>> = HashMap::new();
    visited.insert(position.clone(), vec![direction.clone()]);

    let mut loop_visited: Vec<Option<Vec<Vector>>> = Vec::with_capacity(130 * 130);
    for _ in 0..loop_visited.capacity() {
        loop_visited.push(None);
    }
    let tmp = loop_visited.as_mut_slice();

    // let mut found_loops: HashSet<Point> = HashSet::new();
    loop {
        let next = Point {
            x: position.x + direction.x,
            y: position.y + direction.y,
        };
        if next.x < 0 || next.y < 0 || next.x == width as i32 || next.y == height as i32 {
            break;
        }

        if data[next.y as usize][next.x as usize] == '#' {
            direction = direction.right();
            visited.get_mut(&position).unwrap().push(direction.clone());
            if is_loop(
                &position, &direction, &visited, &data, width, height,
                // &validated_blocks,
                // &mut found_loops
                tmp,
            ) {
                count += 1;
            }
            continue;
        }

        position = next;

        let dirs = visited.entry(next.clone()).or_insert_with(Vec::new);
        dirs.push(direction.clone());

        if is_loop(
            &position, &direction, &visited, &data, width, height,
            // &validated_blocks,
            // &mut found_loops
            tmp,
        ) {
            count += 1;
        }
    }

    // println!("exit at {:?}", position);
    // println!("visited {:?}", visited.len());

    let result = count;
    format!("{result}")
}

fn is_loop(
    position: &Point,
    direction: &Vector,
    visited: &HashMap<Point, Vec<Vector>>,
    data: &Vec<Vec<char>>,
    width: usize,
    height: usize,
    tmp: &mut [Option<Vec<Vector>>],
) -> bool {
    let block = Point {
        x: position.x + direction.x,
        y: position.y + direction.y,
    };

    if visited.contains_key(&block) {
        return false;
    }

    if block.x < 0
        || block.y < 0
        || block.x == width as i32
        || block.y == height as i32
        || data[block.y as usize][block.x as usize] == '#'
    {
        return false;
    }

    let mut right = direction.right();
    let mut p = position.clone();

    let mut visited: Vec<usize> = vec![];

    // let mut loop_visited: HashMap<Point, Vec<Vector>> = HashMap::new();
    // loop_visited.insert(p.clone(), vec![right.clone()]);

    'outer: loop {
        p.x += right.x;
        p.y += right.y;
        if p.x < 0 || p.y < 0 || p.x == width as i32 || p.y == height as i32 {
            break;
        }
        if block == p || data[p.y as usize][p.x as usize] == '#' {
            p.x -= right.x;
            p.y -= right.y;
            right = right.right();
            continue 'outer;
        }

        // let dirs = loop_visited.entry(p.clone()).or_insert_with(Vec::new);
        // if dirs.contains(&right) {
        //     // found_loops.insert(block.clone());
        //     return true;
        // }
        // dirs.push(right.clone());

        let x = (p.x + p.y * (width as i32)) as usize;
        if let Some(dirs) = &mut tmp[x] {
            if dirs.contains(&right) {
                // found_loops.insert(block.clone());
                clear(tmp, &visited);
                return true;
            }
            dirs.push(right.clone());
        } else {
            tmp[x] = Some(vec![right.clone()]);
        }
        visited.push(x);
    }

    clear(tmp, &visited);
    false
}

fn clear(tmp: &mut [Option<Vec<Vector>>], indexes: &Vec<usize>) {
    for i in indexes {
        // tmp[*i] = None
        tmp[*i].as_mut().unwrap().clear()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                ....#.....
                .........#
                ..........
                ..#.......
                .......#..
                ..........
                .#..^.....
                ........#.
                #.........
                ......#...
            ",
        );
        assert_eq!(result, "41")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                ....#.....
                .........#
                ..........
                ..#.......
                .......#..
                ..........
                .#..^.....
                ........#.
                #.........
                ......#...
            ",
        );
        assert_eq!(result, "6")
    }
}

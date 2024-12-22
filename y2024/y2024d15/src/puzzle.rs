
#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    pub fn from(c: char) -> Vector {
        match c {
            '>' => { Vector { x: 1, y: 0 } }
            '<' => { Vector { x: -1, y: 0 } }
            '^' => { Vector { x: 0, y: -1 } }
            'v' => { Vector { x: 0, y: 1 } }
            _ => { panic!() }
        }
    }
}

pub fn calculate1(input: &str) -> String {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut movements: Vec<Vector> = Vec::new();
    input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            if line.starts_with('<') || line.starts_with('>') || line.starts_with('^') || line.starts_with('v') {
                line.chars().map(|c| Vector::from(c)).for_each(|it| movements.push(it));
            } else {
                map.push(line.chars().collect());
            }
        });

    let mut position = Vector { x: 0, y: 0 };
    'outer: for y in 1..map.len() {
        for x in 1..map[y].len() {
            if map[y][x] == '@' {
                position = Vector { x: x as i32, y: y as i32 };
                break 'outer;
            }
        }
    }

    let find_empty = |mut position: Vector, direction: Vector, map: &Vec<Vec<char>>| -> Option<Vector>{
        loop {
            position.x += direction.x;
            position.y += direction.y;
            if map[position.y as usize][position.x as usize] == '.' {
                return Some(position);
            }
            if map[position.y as usize][position.x as usize] == '#' {
                return None;
            }
        }
    };

    for m in movements {
        let next = Vector { x: position.x + m.x, y: position.y + m.y };
        if map[next.y as usize][next.x as usize] == '#' {
            continue
        }
        if map[next.y as usize][next.x as usize] == '.' {
            map[next.y as usize][next.x as usize] = '@';
            map[position.y as usize][position.x as usize] = '.';
            position = next;
            continue
        }
        if map[next.y as usize][next.x as usize] == 'O' {
            if let Some(to) = find_empty(next, m, &map) {
                map[next.y as usize][next.x as usize] = '@';
                map[position.y as usize][position.x as usize] = '.';
                map[to.y as usize][to.x as usize] = 'O';
                position = next;
            }
            continue
        }
    }


    let mut result = 0;
    for y in 1..map.len() {
        for x in 1..map[0].len() {
            if map[y][x] == 'O' {
                result += y * 100 + x;
            }
        }
    }
    format!("{result}")
}

pub fn calculate2(input: &str) -> String {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut movements: Vec<Vector> = Vec::new();
    input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            if line.starts_with('<') || line.starts_with('>') || line.starts_with('^') || line.starts_with('v') {
                line.chars().map(|c| Vector::from(c)).for_each(|it| movements.push(it));
            } else {
                let mut row:Vec<char> = Vec::with_capacity(line.len()*2);
                line.chars().for_each(|c| {
                    match c {
                        '#' => {
                            row.push('#');
                            row.push('#');
                        }
                        '.' => {
                            row.push('.');
                            row.push('.');
                        }
                        'O' => {
                            row.push('[');
                            row.push(']');
                        }
                        '@' => {
                            row.push('@');
                            row.push('.');
                        }
                        _ => {panic!()}
                    }
                });
                map.push(row);
            }
        });

    let mut position = Vector { x: 0, y: 0 };
    'outer: for y in 1..map.len() {
        for x in 1..map[y].len() {
            if map[y][x] == '@' {
                position = Vector { x: x as i32, y: y as i32 };
                break 'outer;
            }
        }
    }

    fn can_push(position: Vector, direction: Vector, map: &Vec<Vec<char>>) -> bool {
        if map[position.y as usize][position.x as usize] == '#' {
            return false;
        }
        if map[position.y as usize][position.x as usize] == '.' {
            return true;
        }

        if direction.y != 0 {
            let next = Vector { x: position.x + direction.x, y: position.y + direction.y };
            if map[position.y as usize][position.x as usize] == '[' {
                can_push(next, direction, map) && can_push(Vector { x: next.x + 1, y: next.y }, direction, map)
            } else {
                can_push(next, direction, map) && can_push(Vector { x: next.x - 1, y: next.y }, direction, map)
            }
        } else {
            if direction.x == 1 {
                if map[position.y as usize][position.x as usize] == '[' {
                    can_push(Vector { x: position.x + 2, y: position.y }, direction, map)
                } else {
                    can_push(Vector { x: position.x + 1, y: position.y }, direction, map)
                }
            } else {
                if map[position.y as usize][position.x as usize] == '[' {
                    can_push(Vector { x: position.x - 1, y: position.y }, direction, map)
                } else {
                    can_push(Vector { x: position.x - 2, y: position.y }, direction, map)
                }
            }
        }
    }

    fn do_push(position: Vector, direction: Vector, map: &mut Vec<Vec<char>>) {
        if map[position.y as usize][position.x as usize] == '.' {
            return;
        }
        if map[position.y as usize][position.x as usize] == '#' {
            return;
        }

        if direction.y != 0 {
            let next = Vector { x: position.x + direction.x, y: position.y + direction.y };
            if map[position.y as usize][position.x as usize] == '[' {
                do_push(next, direction, map);
                do_push(Vector { x: next.x + 1, y: next.y }, direction, map);
            } else if map[position.y as usize][position.x as usize] == ']' {
                do_push(next, direction, map);
                do_push(Vector { x: next.x - 1, y: next.y }, direction, map);
            }
        } else {
            if map[position.y as usize][position.x as usize] == '[' {
                if direction.x == 1 {
                    do_push(Vector { x: position.x + 2, y: position.y }, direction, map);
                } else {
                    do_push(Vector { x: position.x - 1, y: position.y }, direction, map);
                }
            } else if map[position.y as usize][position.x as usize] == ']' {
                if direction.x == 1 {
                    do_push(Vector { x: position.x + 1, y: position.y }, direction, map);
                } else {
                    do_push(Vector { x: position.x - 2, y: position.y }, direction, map);
                }
            }
        }

        if map[position.y as usize][position.x as usize] == '[' {
            let next = Vector { x: position.x + direction.x, y: position.y + direction.y };
            if map[next.y as usize][next.x as usize] == '.' && map[next.y as usize][next.x as usize + 1] == '.' {
                map[position.y as usize][position.x as usize] = '.';
                map[position.y as usize][position.x as usize + 1] = '.';
                map[next.y as usize][next.x as usize] = '[';
                map[next.y as usize][next.x as usize + 1] = ']';
            } else {
                map[position.y as usize][position.x as usize] = '.';
                map[position.y as usize][position.x as usize + 1] = '.';
                map[next.y as usize][next.x as usize] = '[';
                map[next.y as usize][next.x as usize + 1] = ']';
            }
        } else {
            let position = Vector { x: position.x - 1, y: position.y };
            let next = Vector { x: position.x + direction.x, y: position.y + direction.y };
            if map[next.y as usize][next.x as usize] == '.' && map[next.y as usize][next.x as usize - 1] == '.' {
                map[position.y as usize][position.x as usize] = '.';
                map[position.y as usize][position.x as usize + 1] = '.';
                map[next.y as usize][next.x as usize] = '[';
                map[next.y as usize][next.x as usize + 1] = ']';
            } else {
                map[position.y as usize][position.x as usize] = '.';
                map[position.y as usize][position.x as usize + 1] = '.';
                map[next.y as usize][next.x as usize] = '[';
                map[next.y as usize][next.x as usize + 1] = ']';
            }
        }
    }

    for m in movements {
        // for row in &map {
        //     for x in row {
        //         print!("{}", x);
        //     }
        //     println!();
        // }
        // println!();
        // println!("{:?}", m);

        let next = Vector { x: position.x + m.x, y: position.y + m.y };
        if map[next.y as usize][next.x as usize] == '#' {
            continue
        }
        if map[next.y as usize][next.x as usize] == '.' {
            map[next.y as usize][next.x as usize] = '@';
            map[position.y as usize][position.x as usize] = '.';
            position = next;
            continue
        }
        if map[next.y as usize][next.x as usize] == '[' || map[next.y as usize][next.x as usize] == ']' {
            if can_push(next, m, &map) {
                do_push(next, m, &mut map);
                map[next.y as usize][next.x as usize] = '@';
                map[position.y as usize][position.x as usize] = '.';
                position = next;
            }
            continue
        }
    }

    // for row in &map {
    //     for x in row {
    //         print!("{}", x);
    //     }
    //     println!();
    // }
    // println!();



    let mut result = 0;
    for y in 1..map.len() {
        for x in 1..map[0].len() {
            if map[y][x] == '[' {
                result += y * 100 + x;
            }
        }
    }
    format!("{result}")
}

#[cfg(test)]
mod test {
    use std::thread;
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                ########
                #..O.O.#
                ##@.O..#
                #...O..#
                #.#.O..#
                #...O..#
                #......#
                ########

                <^^>>>vv<v>>v<<
            ",
        );
        assert_eq!(result, "2028")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                ##########
                #..O..O.O#
                #......O.#
                #.OO..O.O#
                #..O@..O.#
                #O#..O...#
                #O..O..O.#
                #.OO.O.OO#
                #....O...#
                ##########

                <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
                vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
                ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
                <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
                ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
                ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
                >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
                <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
                ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
                v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
            ",
        );
        assert_eq!(result, "9021")
    }

    #[test]
    fn test_print_2() {
        calculate2(
            "
                #######
                #...#.#
                #.....#
                #..OO@#
                #..O..#
                #.....#
                #######

                <vv<<^^<<^^
            ",
        );
    }
}

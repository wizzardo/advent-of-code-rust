use std::collections::{HashMap, HashSet};

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}


impl Point {
    fn neighbors(&self) -> impl Iterator<Item=Point> + '_ {
        let mut index = 0;
        std::iter::from_fn(move || {
            let offsets = [
                (-1, 0),
                (1, 0),
                (0, -1),
                (0, 1),
            ];

            if index < offsets.len() {
                let (dx, dy) = offsets[index];
                index += 1;
                Some(Point { x: self.x + dx, y: self.y + dy })
            } else {
                None
            }
        })
    }

    fn distance(&self, other: &Point) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }

    fn direction_to(&self, next: &Point) -> Direction {
        if self.x == next.x {
            if self.y == next.y - 1 {
                Direction::North
            } else if self.y == next.y + 1 {
                Direction::South
            } else {
                panic!()
            }
        } else {
            if self.x == next.x - 1 {
                Direction::East
            } else if self.x == next.x + 1 {
                Direction::West
            } else {
                panic!()
            }
        }
    }
}

#[derive(Eq, PartialEq)]
enum Tile {
    Wall,
    OxygenSystem,
    Empty,
    Oxygen,
}

impl Tile {
    fn of(v: i64) -> Tile {
        match v {
            0 => { Tile::Wall }
            1 => { Tile::Empty }
            2 => { Tile::OxygenSystem }
            n => { panic!("unknown tile type: {}", n) }
        }
    }
}

enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn to_command(&self) -> i64 {
        match self {
            Direction::North => { 1 }
            Direction::South => { 2 }
            Direction::West => { 3 }
            Direction::East => { 4 }
        }
    }
}

fn reconstruct_path(came_from: &HashMap<Point, Point>, current: &Point) -> Vec<Point> {
    let mut path: Vec<Point> = vec![];
    path.push(*current);

    let mut c = current;
    loop {
        match came_from.get(c) {
            None => { break; }
            Some(p) => {
                path.push(*p);
                c = p;
            }
        }
    }

    path.reverse();
    return path;
}

fn a_star<H, P>(start: &Point, goal: &Point, h: H, is_passable: P) -> Option<Vec<Point>>
    where
        H: Fn(&Point, &Point) -> u32,
        P: Fn(&Point) -> bool,
{
    if start.distance(goal) == 1 {
        return Some(vec![*start, *goal]);
    }

    let mut open_set: Vec<Point> = vec![];
    open_set.push(*start);

    let mut came_from: HashMap<Point, Point> = HashMap::new();
    let mut g_score: HashMap<Point, u32> = HashMap::new();
    let mut f_score: HashMap<Point, u32> = HashMap::new();

    g_score.insert(*start, 0);
    g_score.insert(*start, h(start, goal));

    while !open_set.is_empty() {
        // let (i, point) = open_set.iter().enumerate().min_by(|(_, a), (_, b)| h(a, goal).cmp(&h(b, goal))).unwrap();
        let current = open_set.remove(open_set.len() - 1);
        if current.eq(goal) {
            return Some(reconstruct_path(&came_from, &current));
        }

        for neighbor in current.neighbors() {
            if !is_passable(&neighbor) {
                continue;
            }

            let tentative_g_score = g_score.get(&current).unwrap() + 1; // + d(current, neighbor), but d returns 1
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, tentative_g_score + h(&neighbor, goal));

                if !open_set.contains(&neighbor) {
                    open_set.push(neighbor);
                    open_set.sort_by(|a, b| h(b, &goal).cmp(&h(a, &goal)));
                }
            }
        };
    }

    None
}

pub fn calculate1(input: &str) -> String {
    let code: Vec<i64> = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            line.split(",").map(|it| it.parse::<i64>().unwrap()).collect()
        })
        // .inspect(|line| { dbg!(line); })
        .last()
        .unwrap();

    let mut output = vec![];

    let mut executor = Executor::new(&code);

    let result = executor.run(&mut output);
    if result != ExecutorState::WaitingForInput {
        panic!("unexpected state: {:?}", result)
    }

    let mut map: HashMap<Point, Tile> = HashMap::new();
    let mut current_x = 0;
    let mut current_y = 0;
    let mut target_x = i32::MAX;
    let mut target_y = i32::MAX;

    let mut undiscovered: HashSet<Point> = HashSet::new();

    map.insert(Point { x: current_x, y: current_y }, Tile::Empty);

    Point { x: current_x, y: current_y }.neighbors()
        .for_each(|x| {
            undiscovered.insert(x);
        });

    'outer: while !undiscovered.is_empty() {
        let start = Point { x: current_x, y: current_y };
        let goal = *undiscovered.iter().min_by(|a, b| a.distance(&start).cmp(&b.distance(&start))).unwrap();
        let path = a_star(
            &start,
            &goal,
            |a, b| a.distance(b),
            |p| { *map.get(p).unwrap_or(&Tile::Empty) != Tile::Wall },
        ).unwrap();


        let mut current = &start;

        for i in 1..path.len() {
            let next = &path[i];
            let command = current.direction_to(next).to_command();

            output.clear();
            let result = executor.run_input(command, &mut output);
            if result != ExecutorState::WaitingForInput {
                panic!("unexpected state: {:?}", result)
            }

            if i == path.len() - 1 {
                let tile = Tile::of(*output.get(0).unwrap());
                if tile != Tile::Wall {
                    current_x = next.x;
                    current_y = next.y;
                    if tile == Tile::OxygenSystem {
                        target_x = next.x;
                        target_y = next.y;
                    }

                    next.neighbors().filter(|p| !map.contains_key(p)).for_each(|p| { undiscovered.insert(p); });
                } else {
                    current_x = current.x;
                    current_y = current.y;

                    current.neighbors().filter(|p| !map.contains_key(p)).for_each(|p| { undiscovered.insert(p); });
                }
                map.insert(*next, tile);
            } else if Tile::of(*output.get(0).unwrap()) == Tile::Wall {
                let next_tile = map.get(next);
                if next_tile.is_none() {
                    current_x = current.x;
                    current_y = current.y;

                    undiscovered.remove(&next);
                    map.insert(*next, Tile::Wall);

                    current.neighbors().filter(|p| !map.contains_key(p)).for_each(|p| { undiscovered.insert(p); });
                    continue 'outer;
                }
                let _current_tile = map.get(current);
                panic!();
            }

            current = next;
        }

        undiscovered.remove(&goal);
    }


    let mut min_x = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_y = 0;

    for p in map.keys() {
        if p.x > max_x {
            max_x = p.x;
        }
        if p.x < min_x {
            min_x = p.x
        }
        if p.y > max_y {
            max_y = p.y;
        }
        if p.y < min_y {
            min_y = p.y;
        }
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if x == 0 && y == 0 {
                print!("S")
            } else {
                match map.get(&Point { x: x, y: y }).unwrap_or(&Tile::Wall) {
                    Tile::Wall => { print!("#") }
                    Tile::OxygenSystem => { print!("X") }
                    Tile::Empty => { print!(" ") }
                    Tile::Oxygen => { print!("O") }
                }
            }
        }
        println!("")
    }

    let path = a_star(
        &Point { x: 0, y: 0 },
        &Point { x: target_x, y: target_y },
        |a, b| a.distance(b),
        |p| { *map.get(p).unwrap_or(&Tile::Empty) != Tile::Wall },
    ).unwrap();

    return (path.len() - 1).to_string();
}


#[derive(Debug, PartialEq, Copy, Clone)]
enum ExecutorState {
    Running,
    WaitingForInput,
    Halted,
}

struct Executor {
    instruction_position: usize,
    relative_base: usize,
    state: ExecutorState,
    code: Vec<i64>,
}


impl Executor {
    fn new(code: &Vec<i64>) -> Executor {
        Executor {
            instruction_position: 0,
            relative_base: 0,
            state: ExecutorState::Running,
            code: code.clone(),
        }
    }

    fn reset(&mut self, code: &Vec<i64>) {
        self.code.clear();
        self.code.resize(code.len(), 0);
        self.code.copy_from_slice(code.as_slice());
        self.instruction_position = 0;
        self.state = ExecutorState::Running;
    }

    fn run_input(&mut self, input: i64, outputs: &mut Vec<i64>) -> ExecutorState {
        if self.state == ExecutorState::WaitingForInput {
            let mut instruction = self.code[self.instruction_position];
            let _opcode = instruction % 100;
            instruction /= 100;
            let index = get_index_to_write_to(instruction, 1, &self.code, self.instruction_position, self.relative_base);
            resize(&mut self.code, index + 1);
            self.code[index] = input;
            self.instruction_position += 2;
            self.state = ExecutorState::Running;
        } else {
            panic!("Amp is not waiting for an input, state: {:?}", self.state)
        }

        self.run(outputs)
    }

    fn run(&mut self, output: &mut Vec<i64>) -> ExecutorState {
        let mut position: usize = self.instruction_position;
        let mut relative_base: usize = self.relative_base;
        let code = &mut self.code;
        loop {
            let mut instruction = code[position];
            let opcode = instruction % 100;
            instruction /= 100;
            match opcode {
                1 => { //add
                    let a = get_parameter(instruction, 1, code, position, relative_base);
                    let b = get_parameter(instruction, 2, code, position, relative_base);
                    let index = get_index_to_write_to(instruction, 3, code, position, relative_base);

                    resize(code, index + 1);

                    code[index] = a + b;
                    position += 4
                }
                2 => { //multiply
                    let a = get_parameter(instruction, 1, code, position, relative_base);
                    let b = get_parameter(instruction, 2, code, position, relative_base);
                    let c = get_index_to_write_to(instruction, 3, code, position, relative_base);

                    resize(code, c + 1);

                    code[c] = a * b;
                    position += 4
                }
                3 => { //read input
                    // let index = get_index_to_write_to(instruction, 1, code, position, relative_base);
                    // resize(code, index + 1);
                    //
                    // code[index] = *input.get(input_index).unwrap();
                    // input_index += 1;
                    // position += 2;
                    self.state = ExecutorState::WaitingForInput;
                    break;
                }
                4 => { //print output
                    let a = get_parameter(instruction, 1, code, position, relative_base);
                    // println!("output: {}", output);
                    output.push(a);
                    position += 2;
                }
                5 => { //jump-if-true
                    let a = get_parameter(instruction, 1, code, position, relative_base);
                    let b = get_parameter(instruction, 2, code, position, relative_base);
                    if a != 0 {
                        position = b as usize;
                    } else {
                        position += 3;
                    }
                }
                6 => { //jump-if-false
                    let a = get_parameter(instruction, 1, code, position, relative_base);
                    let b = get_parameter(instruction, 2, code, position, relative_base);
                    if a == 0 {
                        position = b as usize;
                    } else {
                        position += 3;
                    }
                }
                7 => { //less than
                    let a = get_parameter(instruction, 1, code, position, relative_base);
                    let b = get_parameter(instruction, 2, code, position, relative_base);
                    let index = get_index_to_write_to(instruction, 3, code, position, relative_base);
                    resize(code, index + 1);
                    code[index] = if a < b { 1 } else { 0 };
                    position += 4;
                }
                8 => { //equals
                    let a = get_parameter(instruction, 1, code, position, relative_base);
                    let b = get_parameter(instruction, 2, code, position, relative_base);
                    let index = get_index_to_write_to(instruction, 3, code, position, relative_base);
                    resize(code, index + 1);
                    code[index] = if a == b { 1 } else { 0 };
                    position += 4;
                }
                9 => { //adjusts the relative base
                    let base = get_parameter(instruction, 1, code, position, relative_base);
                    relative_base = (relative_base as i64 + base) as usize;
                    position += 2;
                }
                99 => {
                    self.state = ExecutorState::Halted;
                    break;
                }
                opcode => { panic!("unknown opcode {}", opcode) }
            }
        }
        self.instruction_position = position;
        self.relative_base = relative_base;
        return self.state;
    }
}


fn resize(arr: &mut Vec<i64>, size: usize) {
    if arr.len() < size {
        arr.resize(size, 0)
    }
}

fn get_parameter(instruction: i64, i: usize, code: &Vec<i64>, position: usize, relative_base: usize) -> i64 {
    let n = match i {
        1 => { 1 }
        2 => { 10 }
        3 => { 100 }
        4 => { 1000 }
        n => { panic!("do not support parameter {}", n) }
    };

    let mode = instruction / n % 10;
    return match mode {
        0 => { *code.get(*code.get(position + i).unwrap_or(&0) as usize).unwrap_or(&0) }
        1 => { *code.get(position + i).unwrap_or(&0) }
        2 => { *code.get((relative_base as i64 + code.get(position + i).unwrap_or(&0)) as usize).unwrap_or(&0) }
        _ => { panic!("unsupported mode: {}", mode) }
    };
}

fn get_index_to_write_to(instruction: i64, i: usize, code: &Vec<i64>, position: usize, relative_base: usize) -> usize {
    let n = match i {
        1 => { 1 }
        2 => { 10 }
        3 => { 100 }
        4 => { 1000 }
        n => { panic!("do not support parameter {}", n) }
    };
    let mode = instruction / n % 10;
    return match mode {
        0 => { *code.get(position + i).unwrap_or(&0) as usize }
        2 => { (relative_base as i64 + code.get(position + i).unwrap_or(&0)) as usize }
        _ => { panic!("unsupported mode: {}", mode) }
    };
}


pub fn calculate2(input: &str) -> String {
    let code: Vec<i64> = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            line.split(",").map(|it| it.parse::<i64>().unwrap()).collect()
        })
        // .inspect(|line| { dbg!(line); })
        .last()
        .unwrap();

    let mut output = vec![];

    let mut executor = Executor::new(&code);

    let result = executor.run(&mut output);
    if result != ExecutorState::WaitingForInput {
        panic!("unexpected state: {:?}", result)
    }

    let mut map: HashMap<Point, Tile> = HashMap::new();
    let mut current_x = 0;
    let mut current_y = 0;
    let mut target_x = i32::MAX;
    let mut target_y = i32::MAX;

    let mut undiscovered: HashSet<Point> = HashSet::new();

    map.insert(Point { x: current_x, y: current_y }, Tile::Empty);

    Point { x: current_x, y: current_y }.neighbors()
        .for_each(|x| {
            undiscovered.insert(x);
        });

    'outer: while !undiscovered.is_empty() {
        let start = Point { x: current_x, y: current_y };
        let goal = *undiscovered.iter().min_by(|a, b| a.distance(&start).cmp(&b.distance(&start))).unwrap();
        let path = a_star(
            &start,
            &goal,
            |a, b| a.distance(b),
            |p| { *map.get(p).unwrap_or(&Tile::Empty) != Tile::Wall },
        ).unwrap();


        let mut current = &start;

        for i in 1..path.len() {
            let next = &path[i];
            let command = current.direction_to(next).to_command();

            output.clear();
            let result = executor.run_input(command, &mut output);
            if result != ExecutorState::WaitingForInput {
                panic!("unexpected state: {:?}", result)
            }

            if i == path.len() - 1 {
                let tile = Tile::of(*output.get(0).unwrap());
                if tile != Tile::Wall {
                    current_x = next.x;
                    current_y = next.y;
                    if tile == Tile::OxygenSystem {
                        target_x = next.x;
                        target_y = next.y;
                    }

                    next.neighbors().filter(|p| !map.contains_key(p)).for_each(|p| { undiscovered.insert(p); });
                } else {
                    current_x = current.x;
                    current_y = current.y;

                    current.neighbors().filter(|p| !map.contains_key(p)).for_each(|p| { undiscovered.insert(p); });
                }
                map.insert(*next, tile);
            } else if Tile::of(*output.get(0).unwrap()) == Tile::Wall {
                let next_tile = map.get(next);
                if next_tile.is_none() {
                    current_x = current.x;
                    current_y = current.y;

                    undiscovered.remove(&next);
                    map.insert(*next, Tile::Wall);

                    current.neighbors().filter(|p| !map.contains_key(p)).for_each(|p| { undiscovered.insert(p); });
                    continue 'outer;
                }
                let _current_tile = map.get(current);
                panic!();
            }

            current = next;
        }

        undiscovered.remove(&goal);
    }


    let mut min_x = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_y = 0;

    for p in map.keys() {
        if p.x > max_x {
            max_x = p.x;
        }
        if p.x < min_x {
            min_x = p.x
        }
        if p.y > max_y {
            max_y = p.y;
        }
        if p.y < min_y {
            min_y = p.y;
        }
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if x == 0 && y == 0 {
                print!("S")
            } else {
                match map.get(&Point { x: x, y: y }).unwrap_or(&Tile::Wall) {
                    Tile::Wall => { print!("#") }
                    Tile::OxygenSystem => { print!("X") }
                    Tile::Empty => { print!(" ") }
                    Tile::Oxygen => { print!("O") }
                }
            }
        }
        println!("")
    }

    let mut cursors: Vec<Point> = vec![];
    cursors.push(Point { x: target_x, y: target_y });

    let mut steps = 0;
    let mut to_remove: Vec<usize> = vec![];
    loop {
        steps += 1;
        for i in 0..cursors.len() {
            let mut remove = true;

            let mut j = 0;
            let point = cursors[i];
            for p in point.neighbors() {
                if !(*map.get(&p).unwrap_or(&Tile::Wall) == Tile::Empty) {
                    continue;
                }

                map.insert(p, Tile::Oxygen);
                if j == 0 {
                    cursors[i] = p;
                    remove = false;
                } else {
                    cursors.push(p)
                }
                j += 1;
            }

            if remove {
                to_remove.push(i);
            }
        }
        if to_remove.len() > 0 {
            to_remove.iter().rev().for_each(|j| {
                cursors.remove(*j);
            });
            to_remove.clear();
        }

        if cursors.is_empty() {
            steps -= 1;
            break;
        }

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if x == 0 && y == 0 {
                    print!("S")
                } else {
                    match map.get(&Point { x: x, y: y }).unwrap_or(&Tile::Wall) {
                        Tile::Wall => { print!("#") }
                        Tile::OxygenSystem => { print!("X") }
                        Tile::Empty => { print!(" ") }
                        Tile::Oxygen => { print!("O") }
                    }
                }
            }
            println!("")
        }
        println!("{}", steps);
        println!("");
    }

    return steps.to_string();
}


#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_point_neighbors() {
        let point = Point { x: 0, y: 0 };
        let neighbors: Vec<Point> = point.neighbors().collect();
        assert_eq!(neighbors.len(), 4);
        assert_eq!(neighbors.contains(&Point { x: -1, y: 0 }), true);
        assert_eq!(neighbors.contains(&Point { x: 1, y: 0 }), true);
        assert_eq!(neighbors.contains(&Point { x: 0, y: -1 }), true);
        assert_eq!(neighbors.contains(&Point { x: 0, y: 1 }), true);
    }

    #[test]
    fn test_a_star() {
        let start = Point { x: 0, y: 0 };
        let goal = Point { x: 2, y: 1 };
        let path = a_star(
            &start,
            &goal,
            |a, b| ((a.x - b.x).abs() + (a.y - b.y).abs()) as u32,
            |_| true,
        );
        println!("{:?}", path);

        assert_eq!(path.is_some(), true);
        let path = path.unwrap();
        assert_eq!(path.len(), 4);
        assert_eq!(path[0], start);
        assert_eq!(path[1], Point { x: 0, y: 1 });
        assert_eq!(path[2], Point { x: 1, y: 1 });
        assert_eq!(path[3], goal);
    }
}
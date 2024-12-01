use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::puzzle::Movement::{L, R};

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
                Direction::South
            } else if self.y == next.y + 1 {
                Direction::North
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

#[derive(Eq, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug)]
enum Movement {
    R(u8),
    L(u8),
}


impl Direction {
    fn to_movement(&self, next: Direction, steps: u8) -> Movement {
        match self {
            Direction::North => {
                match next {
                    Direction::North => { panic!() }
                    Direction::South => { panic!() }
                    Direction::West => { L(steps) }
                    Direction::East => { R(steps) }
                }
            }
            Direction::South => {
                match next {
                    Direction::North => { panic!() }
                    Direction::South => { panic!() }
                    Direction::West => { R(steps) }
                    Direction::East => { L(steps) }
                }
            }
            Direction::West => {
                match next {
                    Direction::North => { R(steps) }
                    Direction::South => { L(steps) }
                    Direction::West => { panic!() }
                    Direction::East => { panic!() }
                }
            }
            Direction::East => {
                match next {
                    Direction::North => { L(steps) }
                    Direction::South => { R(steps) }
                    Direction::West => { panic!() }
                    Direction::East => { panic!() }
                }
            }
        }
    }

    fn to_command(&self) -> i64 {
        match self {
            Direction::North => { 1 }
            Direction::South => { 2 }
            Direction::West => { 3 }
            Direction::East => { 4 }
        }
    }
    fn is_opposite(&self, d: &Direction) -> bool {
        match self {
            Direction::North => { *d == Direction::South }
            Direction::South => { *d == Direction::North }
            Direction::West => { *d == Direction::East }
            Direction::East => { *d == Direction::West }
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
    if result != ExecutorState::Halted {
        panic!("unexpected state: {:?}", result)
    }

    let string = output.iter().map(|it| { (*it as u8 as char).to_string() }).join("");
    println!("{}", string);

    let sum = locate_intersections(&output);

    return sum.to_string();
}

fn locate_intersections(output: &Vec<i64>) -> i32 {
    let mut sum = 0;
    let (width, _) = output.iter().find_position(|it| **it == ('\n' as i64)).unwrap();
    let width = width + 1;

    for i in 0..output.len() {
        if output[i] == ('#' as i64) {
            let x: i32 = (i % width) as i32;
            let y: i32 = (i / width) as i32;
            let mut adjacent_scaffold = 0;
            for p in (Point { x, y }.neighbors()) {
                if p.x < 0 || p.y < 0 || (p.y as usize) * width + (p.x as usize) >= output.len() {
                    continue;
                }

                if output[(p.y as usize) * width + (p.x as usize)] == ('#' as i64) {
                    adjacent_scaffold += 1;
                }
            }

            if adjacent_scaffold >= 3 {
                sum += x * y;
            }
        }
    }
    sum
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
    let mut code: Vec<i64> = input.lines()
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
    if result != ExecutorState::Halted {
        panic!("unexpected state: {:?}", result)
    }

    let string = output.iter().map(|it| { (*it as u8 as char).to_string() }).join("");
    println!("{}", string);

    let (width, _) = output.iter().find_position(|it| **it == ('\n' as i64)).unwrap();
    let width = width + 1;

    let (start, _) = output.iter().find_position(|it| **it == ('^' as i64)).unwrap();

    let mut path: Vec<Movement> = vec![];
    let mut direction: Direction = Direction::North;
    let mut x: i32 = (start % width) as i32;
    let mut y: i32 = (start / width) as i32;
    loop {
        let point = Point { x, y };
        let mut next_direction = direction;
        for p in point.neighbors() {
            if p.x < 0 || p.y < 0 || (p.y as usize) * width + (p.x as usize) >= output.len() {
                continue;
            }

            if output[(p.y as usize) * width + (p.x as usize)] == ('#' as i64) && !point.direction_to(&p).is_opposite(&direction) {
                next_direction = point.direction_to(&p);
                break;
            }
        }

        if next_direction == direction {
            break;
        }

        let mut steps: u8 = 0;
        loop {
            match next_direction {
                Direction::North => { y -= 1 }
                Direction::South => { y += 1 }
                Direction::West => { x -= 1 }
                Direction::East => { x += 1 }
            }

            if x < 0 || y < 0 || (y as usize) * width + (x as usize) >= output.len() || output[(y as usize) * width + (x as usize)] != ('#' as i64) {
                match next_direction {
                    Direction::North => { y += 1 }
                    Direction::South => { y -= 1 }
                    Direction::West => { x += 1 }
                    Direction::East => { x -= 1 }
                }
                break;
            }
            steps += 1;
        }

        path.push(direction.to_movement(next_direction, steps));
        direction = next_direction;
    }

    println!("{:?}", path);
    println!();

    for x in path.iter() {
        match x {
            R(n) => { print!("R,{},", n) }
            L(n) => { print!("L,{},", n) }
        }
    }
    println!();

    // todo write an actual algorithm to find these values
    // L,12,L,12,L,6,L,6
    // R,8,R,4,L,12
    // L,12,L,6,R,12,R,8

    let a = "L,12,L,12,L,6,L,6\n";
    let b = "R,8,R,4,L,12\n";
    let c = "L,12,L,6,R,12,R,8\n";

    let routine = "A,B,A,C,B,A,C,B,A,C\n";

    code[0] = 2;

    let mut executor = Executor::new(&code);

    output.clear();
    let result = executor.run(&mut output);
    if result != ExecutorState::WaitingForInput {
        panic!("unexpected state: {:?}", result)
    }

    let string = output.iter().map(|it| { (*it as u8 as char).to_string() }).join("");
    println!("{}", string);
    output.clear();

    for x in routine.chars() {
        executor.run_input(x as i64, &mut output);
    }

    let string = output.iter().map(|it| { (*it as u8 as char).to_string() }).join("");
    println!("{}", string);
    output.clear();

    for x in a.chars() {
        executor.run_input(x as i64, &mut output);
    }

    let string = output.iter().map(|it| { (*it as u8 as char).to_string() }).join("");
    println!("{}", string);
    output.clear();

    for x in b.chars() {
        executor.run_input(x as i64, &mut output);
    }

    let string = output.iter().map(|it| { (*it as u8 as char).to_string() }).join("");
    println!("{}", string);
    output.clear();

    for x in c.chars() {
        executor.run_input(x as i64, &mut output);
    }

    let string = output.iter().map(|it| { (*it as u8 as char).to_string() }).join("");
    println!("{}", string);
    output.clear();

    executor.run_input('n' as i64, &mut output);
    executor.run_input('\n' as i64, &mut output);


    let string = output.iter().map(|it| { (*it as u8 as char).to_string() }).join("");
    println!("{}", string);
    // println!("{:?}", &output.as_slice()[output.len() - 100..output.len()]);

    let result = output[output.len()-1];
    output.clear();
    return result.to_string();
}


#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_point_neighbors() {
        let chars = "..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..
".chars().map(|it| it as i64).collect();
        let intersections = locate_intersections(&chars);

        assert_eq!(intersections, 76)
    }
}
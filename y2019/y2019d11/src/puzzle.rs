use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(PartialEq)]
enum Color {
    Black,
    White,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next(&self) -> Direction {
        return match self {
            Direction::Up => { Direction::Right }
            Direction::Right => { Direction::Down }
            Direction::Down => { Direction::Left }
            Direction::Left => { Direction::Up }
        };
    }
    fn prev(&self) -> Direction {
        return match self {
            Direction::Up => { Direction::Left }
            Direction::Right => { Direction::Up }
            Direction::Down => { Direction::Right }
            Direction::Left => { Direction::Down }
        };
    }
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

    let mut map: HashMap<Point, Color> = HashMap::new();
    let mut current_x: i32 = 0;
    let mut current_y: i32 = 0;
    let mut direction = Direction::Up;

    if executor.run(&mut output) != ExecutorState::WaitingForInput {
        panic!("unexpected state")
    }

    loop {
        let color = map.get(&Point { x: current_x, y: current_y }).unwrap_or(&Color::Black);
        let state = executor.run_input(if *color == Color::Black { 0 } else { 1 }, &mut output);

        if output.len() != 2 {
            panic!("no output")
        }

        // println!("{:?}", output);

        let color = match output[0] {
            0 => { Color::Black }
            1 => { Color::White }
            n => { panic!("unknown color {}", n) }
        };

        direction = match output[1] {
            0 => { direction.prev() }
            1 => { direction.next() }
            n => { panic!("unknown rotation {}", n) }
        };


        map.insert(Point { x: current_x, y: current_y }, color);

        match direction {
            Direction::Up => { current_y += 1 }
            Direction::Right => { current_x += 1 }
            Direction::Down => { current_y -= 1 }
            Direction::Left => { current_x -= 1 }
        }

        output.clear();

        if state == ExecutorState::Halted {
            break;
        }
    }

    return map.len().to_string();
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

    let mut map: HashMap<Point, Color> = HashMap::new();
    let mut current_x: i32 = 0;
    let mut current_y: i32 = 0;
    let mut direction = Direction::Up;

    if executor.run(&mut output) != ExecutorState::WaitingForInput {
        panic!("unexpected state")
    }

    loop {
        let color = map.get(&Point { x: current_x, y: current_y }).unwrap_or(if map.is_empty() { &Color::White } else { &Color::Black });
        let state = executor.run_input(if *color == Color::Black { 0 } else { 1 }, &mut output);

        if output.len() != 2 {
            panic!("no output")
        }

        // println!("{:?}", output);

        let color = match output[0] {
            0 => { Color::Black }
            1 => { Color::White }
            n => { panic!("unknown color {}", n) }
        };

        direction = match output[1] {
            0 => { direction.prev() }
            1 => { direction.next() }
            n => { panic!("unknown rotation {}", n) }
        };


        map.insert(Point { x: current_x, y: current_y }, color);

        match direction {
            Direction::Up => { current_y += 1 }
            Direction::Right => { current_x += 1 }
            Direction::Down => { current_y -= 1 }
            Direction::Left => { current_x -= 1 }
        }

        output.clear();

        if state == ExecutorState::Halted {
            break;
        }
    }

    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    map.keys().for_each(|p| {
        if p.x > max_x {
            max_x = p.x;
        }
        if p.y > max_y {
            max_y = p.y;
        }
        if p.x < min_x {
            min_x = p.x
        }
        if p.y < min_y {
            min_y = p.y
        }
    });

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let color = map.get(&Point { x, y }).unwrap_or(&Color::Black);
            match color {
                Color::Black => { print!(" ") }
                Color::White => { print!("#") }
            }
        }
        println!()
    }

    return map.len().to_string();
}

#[cfg(test)]
mod test {
    use super::*;
}
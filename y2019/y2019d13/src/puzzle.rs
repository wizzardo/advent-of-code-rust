
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

    let blocks = output.iter().enumerate()
        .filter(|(i, v)| { (i + 1) % 3 == 0 && **v == 2 })
        .count();

    return blocks.to_string();
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

    code[0] = 2;

    let mut output = vec![];

    let mut executor = Executor::new(&code);

    let _state = executor.run(&mut output);


    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    let mut max_x = i64::MIN;
    let mut max_y = i64::MIN;

    output.chunks(3)
        .for_each(|it| {
            let x = it[0];
            if x < 0 {
                return;
            }

            let y = it[1];
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
            if x < min_x {
                min_x = x
            }
            if y < min_y {
                min_y = y
            }
        });

    if min_x != 0 || min_y != 0 {
        panic!("min_x != 0 || min_y != 0")
    }
    max_x += 1;
    max_y += 1;

    let mut map: Vec<u8> = Vec::with_capacity((max_x * max_y) as usize);
    map.resize((max_x * max_y) as usize, 0);

    let mut score = 0;
    let mut ball_x: usize = 0;
    let mut ball_y: usize = 0;
    let mut paddle_x: usize = 0;
    let mut paddle_y: usize = 0;

    output.chunks(3)
        .for_each(|it| {
            let x = it[0];
            let y = it[1];
            let v = it[2];
            if x < 0 {
                score = v;
                return;
            }

            if v == 3 {
                paddle_x = x as usize;
                paddle_y = y as usize;
            }
            if v == 4 {
                ball_x = x as usize;
                ball_y = y as usize;
            }

            map[(y * max_x + x) as usize] = v as u8;
        });

    // print_map(max_x, max_y, &map, score);

    let mut ball_direction: i32 = -1;
    let mut next_move = 0;

    loop {
        output.clear();
        let state = executor.run_input(next_move, &mut output);

        output.chunks(3)
            .for_each(|it| {
                let x = it[0];
                let y = it[1];
                let v = it[2];
                if x < 0 {
                    score = v;
                    return;
                }

                if v == 3 {
                    paddle_x = x as usize;
                    paddle_y = y as usize;
                }
                if v == 4 {
                    ball_direction = if x as usize > ball_x { 1 } else { -1 };
                    ball_x = x as usize;
                    ball_y = y as usize;
                }
                map[(y * max_x + x) as usize] = v as u8;
            });


        let mut to_x: i32 = ball_x as i32;
        to_x += ball_direction * ((paddle_y as i32) - 1 - (ball_y as i32));
        if to_x <= 0 {
            to_x = to_x.abs() + 1;
        }
        if to_x >= (max_x - 1) as i32 {
            to_x -= to_x - (max_x as i32) - 1;
        }

        if paddle_x as i32 != to_x {
            if paddle_x < to_x as usize {
                next_move = 1;
            } else {
                next_move = -1;
            }
        } else {
            next_move = 0;
        }

        // print_map(max_x, max_y, &map, score);

        if state == ExecutorState::Halted {
            break;
        }
    }

    return score.to_string();
}

fn print_map(max_x: i64, max_y: i64, map: &Vec<u8>, score: i64) {
    for y in 0..max_y {
        for x in 0..max_x {
            match map[(y * max_x + x) as usize] {
                0 => { print!(" ") }
                1 => { print!("#") }
                2 => { print!("B") }
                3 => { print!("T") }
                4 => { print!("o") }
                x => { panic!("unknown state: {}", x) }
            }
        }
        println!("")
    }
    println!("score: {}", score);
}

#[cfg(test)]
mod test {
    use super::*;
}
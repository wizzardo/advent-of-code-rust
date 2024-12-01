
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

    let mut counter = 0;
    for y in 0..50 {
        for x in 0..50 {
            executor.reset(&code);
            executor.run(&mut output);
            executor.run_input(x, &mut output);
            executor.run_input(y, &mut output);

            match output.get(0) {
                None => { panic!() }
                Some(state) => {
                    match state {
                        0 => {}
                        1 => { counter += 1 }
                        _ => { panic!() }
                    }
                }
            }
            output.clear()
        }
    }

    return counter.to_string();
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

    // for y in 0..50 {
    //     for x in 0..50 {
    //         let beam = check_beam(&code, &mut output, &mut executor, y, x);
    //
    //         match beam {
    //             0 => { print!(".") }
    //             1 => { print!("#") }
    //             _ => { panic!() }
    //         }
    //     }
    //     println!()
    // }

    // y = ax+b
    let mut min_y = i64::MAX;
    let mut max_y = i64::MAX;

    for y in 0..50 {
        let beam = check_beam(&code, &mut output, &mut executor, y, 50);
        if beam != 0 {
            if min_y == i64::MAX {
                min_y = y;
            } else {
                max_y = y;
            }
        }
    }

    println!("{}", min_y);
    println!("{}", max_y);

    let a1 = (min_y as f64) / 50.0;
    let a2 = (max_y as f64) / 50.0;

    println!("{}", a1);
    println!("{}", a2);

    // y1 = a1 * x1
    // y2 = a2 * x2
    // y2 = y1 + 100
    // x2 = x1 - 100
    // a2 * (x1 - 100) = a1 * x1 + 100
    // a2*x1 - 100*a2 = a1*x1 + 100
    // x(a2-a1) = 100a2 +100


    let  x = (100.0 * a2 + 100.0) / (a2 - a1);

    println!("{}", x);

    let y = a1 * x;

    let mut y = y.ceil() as i64;
    let  y2 = (a2 * x).ceil() as i64;
    let mut x = x.ceil() as i64;

    let offset = 99;
    loop {
        let beam = check_beam(&code, &mut output, &mut executor, y, x);
        let beam2 = check_beam(&code, &mut output, &mut executor, y2, x - offset);
        println!("{}x{}: {}  -  {}x{}: {}", x, y, beam, x - offset, y2, beam2);

        if beam == 0 {
            x -= 1;
            break;
        }
        x += 1;
    }


    loop {
        let beam = check_beam(&code, &mut output, &mut executor, y + offset, x - offset);
        if beam == 0 {
            break;
        }
        y -= 1;
        // println!("{}x{}", x, y);
        loop {
            let beam = check_beam(&code, &mut output, &mut executor, y, x);
            if beam == 1 {
                break;
            }
            x -= 1;
            // println!("{}x{}", x, y);
        }
    }

    println!("{}x{}", x, y);

    y -= 1;
    x -= 1;

    for i in 0..5 {
        for j in 0..5 {
            let b = check_beam(&code, &mut output, &mut executor, y - 2 + i, x - 2 + j);
            match b {
                0 => { print!(".") }
                1 => { print!("#") }
                _ => {}
            }
        }
        println!()
    }
    println!();
    for i in 0..5 {
        for j in 0..5 {
            let b = check_beam(&code, &mut output, &mut executor, y + offset - 2 + i, x - offset - 2 + j);
            match b {
                0 => { print!(".") }
                1 => { print!("#") }
                _ => {}
            }
        }
        println!()
    }

    let b1 = check_beam(&code, &mut output, &mut executor, y, x);
    let b2 = check_beam(&code, &mut output, &mut executor, y + offset, x - offset);

    println!("{}x{}", b1, b2);
    println!("at {}x{}", x, y);

    let result = (x - offset) * 10000 + y;

    return result.to_string();
}

pub fn calculate2_test(input: &str) -> String {
    let map: Vec<Vec<char>> = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            line.chars().map(|x| match x {
                // 'O' => { '#' }
                c => { c }
            }).collect()
        })
        // .inspect(|line| { dbg!(line); })
        .collect();

    // for line in map {
    //     for c in line {
    //         print!("{}", c);
    //     }
    //     println!()
    // }


    // y = ax+b
    let mut min_y = i64::MAX;
    let mut max_y = i64::MAX;

    for y in 0..20 {
        let beam = check_beam_test(&map, y, 20);
        if beam != 0 {
            if min_y == i64::MAX {
                min_y = y;
            } else {
                max_y = y;
            }
        }
    }

    println!("{}", min_y);
    println!("{}", max_y);

    let  y = 20;
    let  x = 25 + 9;

    println!("{}x{}", x, y);

    for i in 0..5 {
        for j in 0..5 {
            print!("{}", map[(y as usize) - 2 + i][(x as usize) - 2 + j]);
            // let b = check_beam_test(&map, y - 2 + i, x - 2 + j);
            // match b {
            //     0 => { print!(".") }
            //     1 => { print!("#") }
            //     _ => {}
            // }
        }
        println!()
    }
    println!();
    for i in 0..5 {
        for j in 0..5 {
            print!("{}", map[(y as usize) + 9 - 2 + i][(x as usize) - 9 - 2 + j]);
            // let b = check_beam_test(&map, y + 10 - 2 + i, x - 10 - 2 + j);
            // match b {
            //     0 => { print!(".") }
            //     1 => { print!("#") }
            //     _ => {}
            // }
        }
        println!()
    }

    let b1 = check_beam_test(&map, y, x);
    let b2 = check_beam_test(&map, y + 9, x - 9);

    println!("{}x{}", b1, b2);
    println!("at {}x{}", x, y);

    let result = (x - (10-1)) * 10000 + y;

    return result.to_string();
}

fn check_beam(code: &Vec<i64>, mut output: &mut Vec<i64>, executor: &mut Executor, y: i64, x: i64) -> i64 {
    executor.reset(&code);
    executor.run(&mut output);
    executor.run_input(x, &mut output);
    executor.run_input(y, &mut output);

    let result = match output.get(0) {
        None => { panic!() }
        Some(result) => {
            match result {
                0 => { 0 }
                1 => { 1 }
                _ => { panic!() }
            }
        }
    };
    output.clear();
    result
}

fn check_beam_test(map: &Vec<Vec<char>>, y: i64, x: i64) -> i64 {
    match map[y as usize][x as usize] {
        '#' => { 1 }
        'O' => { 1 }
        _ => { 0 }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2() {
        let result = calculate2_test("
                #.......................................
                .#......................................
                ..##....................................
                ...###..................................
                ....###.................................
                .....####...............................
                ......#####.............................
                ......######............................
                .......#######..........................
                ........########........................
                .........#########......................
                ..........#########.....................
                ...........##########...................
                ...........############.................
                ............############................
                .............#############..............
                ..............##############............
                ...............###############..........
                ................###############.........
                ................#################.......
                .................########OOOOOOOOOO.....
                ..................#######OOOOOOOOOO#....
                ...................######OOOOOOOOOO###..
                ....................#####OOOOOOOOOO#####
                .....................####OOOOOOOOOO#####
                .....................####OOOOOOOOOO#####
                ......................###OOOOOOOOOO#####
                .......................##OOOOOOOOOO#####
                ........................#OOOOOOOOOO#####
                .........................OOOOOOOOOO#####
                ..........................##############
                ..........................##############
                ...........................#############
                ............................############
                .............................###########
            "
        );
        assert_eq!(result, "250020")
    }
}
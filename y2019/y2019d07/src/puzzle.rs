use crate::puzzle::AmpState::{Halted, Running, WaitingForInput};

pub fn calculate1(input: &str) -> String {
    let vec: Vec<i32> = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            line.split(",").map(|it| it.parse::<i32>().unwrap()).collect()
        })
        // .inspect(|line| { dbg!(line); })
        .last()
        .unwrap();

    let mut max = 0;

    for a in 0..5 {
        for b in 0..5 {
            if b == a {
                continue;
            }
            for c in 0..5 {
                if c == a || c == b {
                    continue;
                }
                for d in 0..5 {
                    if d == a || d == b || d == c {
                        continue;
                    }
                    for e in 0..5 {
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }

                        let mut vec1 = vec.clone();
                        let arr = vec1.as_mut_slice();
                        let result = execute_2(arr, &[a, 0]);

                        let mut vec1 = vec.clone();
                        let arr = vec1.as_mut_slice();
                        let i = *result.get(0).unwrap();
                        let result = execute_2(arr, &[b, i]);

                        let mut vec1 = vec.clone();
                        let arr = vec1.as_mut_slice();
                        let i = *result.get(0).unwrap();
                        let result = execute_2(arr, &[c, i]);

                        let mut vec1 = vec.clone();
                        let arr = vec1.as_mut_slice();
                        let i = *result.get(0).unwrap();
                        let result = execute_2(arr, &[d, i]);

                        let mut vec1 = vec.clone();
                        let arr = vec1.as_mut_slice();
                        let i = *result.get(0).unwrap();
                        let result = execute_2(arr, &[e, i]);

                        if *result.get(0).unwrap() > max {
                            max = *result.get(0).unwrap();
                        }
                    }
                }
            }
        }
    }

    return max.to_string();
}


fn execute_2(arr: &mut [i32], input: &[i32]) -> Vec<i32> {
    let mut position: usize = 0;
    let mut outputs: Vec<i32> = vec![];
    let mut input_index: usize = 0;
    loop {
        let mut instruction = arr[position];
        let opcode = instruction % 100;
        instruction /= 100;
        match opcode {
            1 => { //add
                let a = get_parameter(instruction, 1, arr, position);
                let b = get_parameter(instruction, 2, arr, position);
                let c = get_parameter_immediate(3, arr, position);

                arr[c as usize] = a + b;
                position += 4
            }
            2 => { //multiply
                let a = get_parameter(instruction, 1, arr, position);
                let b = get_parameter(instruction, 2, arr, position);
                let c = get_parameter_immediate(3, arr, position);
                arr[c as usize] = a * b;
                position += 4
            }
            3 => { //read input
                let index = get_parameter_immediate(1, arr, position);
                arr[index as usize] = *input.get(input_index).unwrap();
                input_index += 1;
                position += 2;
            }
            4 => { //print output
                let output = get_parameter(instruction, 1, arr, position);
                // println!("output: {}", output);
                outputs.push(output);
                position += 2;
            }
            5 => { //jump-if-true
                let a = get_parameter(instruction, 1, arr, position);
                let b = get_parameter(instruction, 2, arr, position);
                if a != 0 {
                    position = b as usize;
                } else {
                    position += 3;
                }
            }
            6 => { //jump-if-false
                let a = get_parameter(instruction, 1, arr, position);
                let b = get_parameter(instruction, 2, arr, position);
                if a == 0 {
                    position = b as usize;
                } else {
                    position += 3;
                }
            }
            7 => { //less than
                let a = get_parameter(instruction, 1, arr, position);
                let b = get_parameter(instruction, 2, arr, position);
                let index = get_parameter_immediate(3, arr, position);
                arr[index as usize] = if a < b { 1 } else { 0 };
                position += 4;
            }
            8 => { //equals
                let a = get_parameter(instruction, 1, arr, position);
                let b = get_parameter(instruction, 2, arr, position);
                let index = get_parameter_immediate(3, arr, position);
                arr[index as usize] = if a == b { 1 } else { 0 };
                position += 4;
            }
            99 => { break; }
            opcode => { panic!("unknown opcode {}", opcode) }
        }
    }

    outputs
}

fn get_parameter(instruction: i32, i: usize, arr: &[i32], position: usize) -> i32 {
    let n = match i {
        1 => { 1 }
        2 => { 10 }
        3 => { 100 }
        4 => { 1000 }
        n => { panic!("do not support parameter {}", n) }
    };

    return if instruction / n % 10 == 0 {
        arr[arr[position + i] as usize]
    } else {
        arr[position + i]
    };
}

fn get_parameter_immediate(i: usize, arr: &[i32], position: usize) -> i32 {
    return arr[position + i];
}


struct Amp {
    instruction_position: usize,
    state: AmpState,
    code: Vec<i32>,
}

#[derive(Debug, PartialEq)]
enum AmpState {
    Running,
    WaitingForInput,
    Halted,
}

impl Amp {
    fn new(code: &Vec<i32>) -> Amp {
        Amp { instruction_position: 0, state: Running, code: code.clone() }
    }

    fn reset(&mut self, code: &Vec<i32>) {
        self.code.clear();
        self.code.resize(code.len(), 0);
        self.code.copy_from_slice(code.as_slice());
        self.instruction_position = 0;
        self.state = Running;
    }

    fn run_input(&mut self, input: i32, outputs: &mut Vec<i32>) {
        if self.state == WaitingForInput {
            let index = get_parameter_immediate(1, self.code.as_slice(), self.instruction_position);
            self.code[index as usize] = input;
            self.instruction_position += 2;
            self.state = Running;
        } else {
            panic!("Amp is not waiting for an input, state: {:?}", self.state)
        }

        self.run(outputs)
    }

    fn run(&mut self, outputs: &mut Vec<i32>) {
        let mut position: usize = self.instruction_position;
        let arr = self.code.as_mut_slice();
        loop {
            let mut instruction = arr[position];
            let opcode = instruction % 100;
            instruction /= 100;
            match opcode {
                1 => { //add
                    let a = get_parameter(instruction, 1, arr, position);
                    let b = get_parameter(instruction, 2, arr, position);
                    let c = get_parameter_immediate(3, arr, position);

                    arr[c as usize] = a + b;
                    position += 4
                }
                2 => { //multiply
                    let a = get_parameter(instruction, 1, arr, position);
                    let b = get_parameter(instruction, 2, arr, position);
                    let c = get_parameter_immediate(3, arr, position);
                    arr[c as usize] = a * b;
                    position += 4
                }
                3 => { //read input
                    // let index = get_parameter_immediate(1, arr, position);
                    // arr[index as usize] = *input.get(input_index).unwrap();
                    // input_index += 1;
                    // position += 2;
                    self.state = WaitingForInput;
                    break;
                }
                4 => { //print output
                    let output = get_parameter(instruction, 1, arr, position);
                    // println!("output: {}", output);
                    outputs.push(output);
                    position += 2;
                }
                5 => { //jump-if-true
                    let a = get_parameter(instruction, 1, arr, position);
                    let b = get_parameter(instruction, 2, arr, position);
                    if a != 0 {
                        position = b as usize;
                    } else {
                        position += 3;
                    }
                }
                6 => { //jump-if-false
                    let a = get_parameter(instruction, 1, arr, position);
                    let b = get_parameter(instruction, 2, arr, position);
                    if a == 0 {
                        position = b as usize;
                    } else {
                        position += 3;
                    }
                }
                7 => { //less than
                    let a = get_parameter(instruction, 1, arr, position);
                    let b = get_parameter(instruction, 2, arr, position);
                    let index = get_parameter_immediate(3, arr, position);
                    arr[index as usize] = if a < b { 1 } else { 0 };
                    position += 4;
                }
                8 => { //equals
                    let a = get_parameter(instruction, 1, arr, position);
                    let b = get_parameter(instruction, 2, arr, position);
                    let index = get_parameter_immediate(3, arr, position);
                    arr[index as usize] = if a == b { 1 } else { 0 };
                    position += 4;
                }
                99 => {
                    self.state = Halted;
                    break;
                }
                opcode => { panic!("unknown opcode {}", opcode) }
            }
        }

        self.instruction_position = position;
    }
}

pub fn calculate2(input: &str) -> String {
    let vec: Vec<i32> = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            line.split(",").map(|it| it.parse::<i32>().unwrap()).collect()
        })
        // .inspect(|line| { dbg!(line); })
        .last()
        .unwrap();

    let mut max = 0;

    let mut amp_a = Amp::new(&vec);
    let mut amp_b = Amp::new(&vec);
    let mut amp_c = Amp::new(&vec);
    let mut amp_d = Amp::new(&vec);
    let mut amp_e = Amp::new(&vec);

    for a in 5..10 {
        for b in 5..10 {
            if b == a {
                continue;
            }
            for c in 5..10 {
                if c == a || c == b {
                    continue;
                }
                for d in 5..10 {
                    if d == a || d == b || d == c {
                        continue;
                    }
                    for e in 5..10 {
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }

                        // let mut amp_a = Amp::new(&vec);
                        // let mut amp_b = Amp::new(&vec);
                        // let mut amp_c = Amp::new(&vec);
                        // let mut amp_d = Amp::new(&vec);
                        // let mut amp_e = Amp::new(&vec);
                        amp_a.reset(&vec);
                        amp_b.reset(&vec);
                        amp_c.reset(&vec);
                        amp_d.reset(&vec);
                        amp_e.reset(&vec);

                        let mut input = 0;
                        let mut output = vec![];

                        amp_a.run(&mut output);
                        amp_b.run(&mut output);
                        amp_c.run(&mut output);
                        amp_d.run(&mut output);
                        amp_e.run(&mut output);

                        let _result = amp_a.run_input(a, &mut output);
                        let _result = amp_b.run_input(b, &mut output);
                        let _result = amp_c.run_input(c, &mut output);
                        let _result = amp_d.run_input(d, &mut output);
                        let _result = amp_e.run_input(e, &mut output);


                        loop {
                            if amp_a.state == Halted {
                                break;
                            }

                            amp_a.run_input(input, &mut output);

                            input = *output.get(0).unwrap();
                            output.clear();

                            amp_b.run_input(input, &mut output);

                            input = *output.get(0).unwrap();
                            output.clear();

                            amp_c.run_input(input, &mut output);

                            input = *output.get(0).unwrap();
                            output.clear();

                            amp_d.run_input(input, &mut output);

                            input = *output.get(0).unwrap();
                            output.clear();

                            amp_e.run_input(input, &mut output);

                            input = *output.get(0).unwrap();
                            output.clear();
                        }

                        if input > max {
                            max = input;
                        }
                    }
                }
            }
        }
    }

    return max.to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1_1() {
        let result = calculate1("
                3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0
            "
        );
        assert_eq!(result, "43210")
    }

    #[test]
    fn test_1_2() {
        let result = calculate1("
                3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0
            "
        );
        assert_eq!(result, "54321")
    }

    #[test]
    fn test_1_3() {
        let result = calculate1("
                3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0
            "
        );
        assert_eq!(result, "65210")
    }


    #[test]
    fn test_2_1() {
        let result = calculate2("
                3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5
            "
        );
        assert_eq!(result, "139629729")
    }

    #[test]
    fn test_2_3() {
        let result = calculate2("
                3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10
            "
        );
        assert_eq!(result, "18216")
    }
}
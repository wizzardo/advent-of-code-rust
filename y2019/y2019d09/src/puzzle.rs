pub fn calculate1(input: &str) -> String {
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
    execute(&mut code, &[1], &mut output);

    let strings: Vec<String> = output.iter().map(|x| x.to_string()).collect();
    return strings.join(", ").to_string();
}


fn execute(arr: &mut Vec<i64>, input: &[i64], output: &mut Vec<i64>) {
    let mut position: usize = 0;
    let mut relative_base: usize = 0;
    let mut input_index: usize = 0;
    loop {
        let mut instruction = arr[position];
        let opcode = instruction % 100;
        instruction /= 100;
        match opcode {
            1 => { //add
                let a = get_parameter(instruction, 1, arr, position, relative_base);
                let b = get_parameter(instruction, 2, arr, position, relative_base);
                let index = get_index_to_write_to(instruction, 3, arr, position, relative_base);

                resize(arr, index + 1);

                arr[index] = a + b;
                position += 4
            }
            2 => { //multiply
                let a = get_parameter(instruction, 1, arr, position, relative_base);
                let b = get_parameter(instruction, 2, arr, position, relative_base);
                let c = get_index_to_write_to(instruction, 3, arr, position, relative_base);

                resize(arr, c + 1);

                arr[c] = a * b;
                position += 4
            }
            3 => { //read input
                let index = get_index_to_write_to(instruction, 1, arr, position, relative_base);
                resize(arr, index + 1);

                arr[index] = *input.get(input_index).unwrap();
                input_index += 1;
                position += 2;
            }
            4 => { //print output
                let a = get_parameter(instruction, 1, arr, position, relative_base);
                // println!("output: {}", output);
                output.push(a);
                position += 2;
            }
            5 => { //jump-if-true
                let a = get_parameter(instruction, 1, arr, position, relative_base);
                let b = get_parameter(instruction, 2, arr, position, relative_base);
                if a != 0 {
                    position = b as usize;
                } else {
                    position += 3;
                }
            }
            6 => { //jump-if-false
                let a = get_parameter(instruction, 1, arr, position, relative_base);
                let b = get_parameter(instruction, 2, arr, position, relative_base);
                if a == 0 {
                    position = b as usize;
                } else {
                    position += 3;
                }
            }
            7 => { //less than
                let a = get_parameter(instruction, 1, arr, position, relative_base);
                let b = get_parameter(instruction, 2, arr, position, relative_base);
                let index = get_index_to_write_to(instruction, 3, arr, position, relative_base);
                resize(arr, index + 1);
                arr[index] = if a < b { 1 } else { 0 };
                position += 4;
            }
            8 => { //equals
                let a = get_parameter(instruction, 1, arr, position, relative_base);
                let b = get_parameter(instruction, 2, arr, position, relative_base);
                let index = get_index_to_write_to(instruction, 3, arr, position, relative_base);
                resize(arr, index + 1);
                arr[index] = if a == b { 1 } else { 0 };
                position += 4;
            }
            9 => { //adjusts the relative base
                let base = get_parameter(instruction, 1, arr, position, relative_base);
                relative_base = (relative_base as i64 + base) as usize;
                position += 2;
            }
            99 => {
                break;
            }
            opcode => { panic!("unknown opcode {}", opcode) }
        }
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
    execute(&mut code, &[2], &mut output);

    let strings: Vec<String> = output.iter().map(|x| x.to_string()).collect();
    return strings.join(", ").to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1_1() {
        let mut output: Vec<i64> = vec![];
        let mut code: Vec<i64> = vec![104, 1125899906842624, 99];
        execute(&mut code, &[], &mut output);
        assert_eq!(output.get(0), Some(&1125899906842624))
    }

    #[test]
    fn test_1_2() {
        let mut output: Vec<i64> = vec![];
        let mut code: Vec<i64> = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        execute(&mut code, &[], &mut output);
        assert_eq!(output.get(0), Some(&1219070632396864))
    }

    #[test]
    fn test_1_3() {
        let mut output: Vec<i64> = vec![];
        let mut code: Vec<i64> = vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99];
        let initial_code_length = code.len();
        execute(&mut code, &[], &mut output);
        assert_eq!(output.len(), initial_code_length);
        assert_eq!(output.as_slice(), &code[0..initial_code_length]);
    }
}
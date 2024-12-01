pub fn calculate1(input: &str) -> String {
    let mut vec: Vec<i32> = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            line.split(",").map(|it| it.parse::<i32>().unwrap()).collect()
        })
        // .inspect(|line| { dbg!(line); })
        .last()
        .unwrap();

    let arr = vec.as_mut_slice();

    execute(arr);

    return arr[0].to_string();
}

fn execute(arr: &mut [i32]) {
    let mut position: usize = 0;
    loop {
        let mut instruction = arr[position];
        let opcode = instruction % 100;
        instruction /= 100;
        match opcode {
            1 => {
                let a = get_parameter(instruction, 1, arr, position);
                let b = get_parameter(instruction, 2, arr, position);
                let c = get_parameter_immediate(3, arr, position);

                arr[c as usize] = a + b;
                position += 4
            }
            2 => {
                let a = get_parameter(instruction, 1, arr, position);
                let b = get_parameter(instruction, 2, arr, position);
                let c = get_parameter_immediate(3, arr, position);
                arr[c as usize] = a * b;
                position += 4
            }
            3 => {
                let index = get_parameter_immediate(1, arr, position);
                arr[index as usize] = 1; // input
                position += 2;
            }
            4 => {
                let index = get_parameter_immediate(1, arr, position);
                let output = arr[index as usize];
                println!("output: {}", output);
                position += 2;
            }
            99 => { break; }
            opcode => { panic!("unknown opcode {}", opcode) }
        }
    }
}

fn execute_2(arr: &mut [i32], input: i32) -> Vec<i32> {
    let mut position: usize = 0;
    let mut outputs: Vec<i32> = vec![];
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
                arr[index as usize] = input;
                position += 2;
            }
            4 => { //print output
                let output = get_parameter(instruction,1, arr, position);
                println!("output: {}", output);
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


pub fn calculate2(input: &str) -> String {
    let mut vec: Vec<i32> = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            line.split(",").map(|it| it.parse::<i32>().unwrap()).collect()
        })
        // .inspect(|line| { dbg!(line); })
        .last()
        .unwrap();

    let arr = vec.as_mut_slice();

    let result = execute_2(arr, 5);

    return result.get(0).unwrap().to_string();
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn execute_1_1() {
        let mut arr = [1002, 4, 3, 4, 33];
        execute(&mut arr);
        assert_eq!(arr, [1002, 4, 3, 4, 99])
    }

    #[test]
    fn execute_1_2() {
        let mut arr: [i32; 5] = [1101, 100, -1, 4, 0];
        execute(&mut arr);
        assert_eq!(arr, [1101, 100, -1, 4, 99])
    }

    #[test]
    fn execute_1_3() {
        let mut arr: [i32; 5] = [3, 0, 4, 0, 99];
        execute(&mut arr);
    }


    #[test]
    fn execute_2_equal_to_8() {
        let mut arr: [i32; 11] = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let vec = execute_2(&mut arr, 8);
        assert_eq!(vec.get(0), Some::<&i32>(&1));

        let vec = execute_2(&mut arr, 5);
        assert_eq!(vec.get(0), Some::<&i32>(&0));
    }

    #[test]
    fn execute_2_equal_to_8_im() {
        let mut arr: [i32; 9] = [3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let vec = execute_2(&mut arr, 8);
        assert_eq!(vec.get(0), Some::<&i32>(&1));

        let vec = execute_2(&mut arr, 5);
        assert_eq!(vec.get(0), Some::<&i32>(&0));
    }

    #[test]
    fn execute_2_less_then_8() {
        let mut arr: [i32; 11] = [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let vec = execute_2(&mut arr, 5);
        assert_eq!(vec.get(0), Some::<&i32>(&1));

        let vec = execute_2(&mut arr, 8);
        assert_eq!(vec.get(0), Some::<&i32>(&0));
    }

    #[test]
    fn execute_2_less_then_8_im() {
        let mut arr: [i32; 9] = [3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let vec = execute_2(&mut arr, 5);
        assert_eq!(vec.get(0), Some::<&i32>(&1));

        let vec = execute_2(&mut arr, 8);
        assert_eq!(vec.get(0), Some::<&i32>(&0));
    }

    #[test]
    fn execute_2_jump() {
        let mut arr: [i32; 16] = [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        // let vec = execute_2(&mut arr, 5);
        // assert_eq!(vec.get(0), Some::<&i32>(&1));

        let vec = execute_2(&mut arr, 0);
        assert_eq!(vec.get(0), Some::<&i32>(&0));
    }

    #[test]
    fn execute_2_jump_im() {
        let mut arr: [i32; 13] = [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let vec = execute_2(&mut arr, 5);
        assert_eq!(vec.get(0), Some::<&i32>(&1));

        let vec = execute_2(&mut arr, 0);
        assert_eq!(vec.get(0), Some::<&i32>(&0));
    }

    #[test]
    fn execute_2_large() {
        let mut arr: [i32; 47] = [
            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
        ];
        let vec = execute_2(&mut arr, 7);
        assert_eq!(vec.get(0), Some::<&i32>(&999));

        let vec = execute_2(&mut arr, 8);
        assert_eq!(vec.get(0), Some::<&i32>(&1000));

        let vec = execute_2(&mut arr, 9);
        assert_eq!(vec.get(0), Some::<&i32>(&1001));
    }
}
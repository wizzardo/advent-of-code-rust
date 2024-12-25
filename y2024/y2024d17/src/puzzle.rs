struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

pub fn calculate1(input: &str) -> String {
    let mut registers = Registers { a: 0, b: 0, c: 0 };
    let mut program: Vec<u8> = Vec::with_capacity(0);
    input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .enumerate()
        .for_each(|(i, line)| {
            let (_l, r) = line.split_once(": ").unwrap();
            match i {
                0 => registers.a = r.parse::<u64>().unwrap(),
                1 => registers.b = r.parse::<u64>().unwrap(),
                2 => registers.c = r.parse::<u64>().unwrap(),
                3 => {
                    program = r.split(',').map(|x| x.parse::<u8>().unwrap()).collect();
                }
                _ => {
                    panic!()
                }
            }
        });

    let mut out: Vec<u8> = Vec::new();
    let mut i = 0;
    loop {
        let operand = program[i + 1];
        match program[i] {
            0 => {
                let operand = get_combo_operand(&mut registers, operand);
                registers.a /= 2u64.pow(operand as u32);
            }
            1 => {
                registers.b ^= operand as u64;
            }
            2 => {
                let operand = get_combo_operand(&mut registers, operand);
                registers.b = (operand % 8) as u64;
            }
            3 => {
                if registers.a != 0 {
                    i = operand as usize;
                    continue;
                }
            }
            4 => {
                registers.b ^= registers.c;
            }
            5 => {
                let operand = get_combo_operand(&registers, operand);
                out.push((operand % 8) as u8);
            }
            6 => {
                let operand = get_combo_operand(&registers, operand);
                registers.b = registers.a / 2u64.pow(operand as u32);
            }
            7 => {
                let operand = get_combo_operand(&registers, operand);
                registers.c = registers.a / 2u64.pow(operand as u32);
            }
            x => {
                panic!("Unexpected opcode: {}", x)
            }
        }

        i += 2;
        if i >= program.len() {
            break;
        }
    }

    let result = out
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");
    format!("{result}")
}

fn get_combo_operand(registers: &Registers, operand: u8) -> u64 {
    match operand {
        0 | 1 | 2 | 3 => operand as u64,
        4 => registers.a,
        5 => registers.b,
        6 => registers.c,
        x => {
            panic!("Unknown operand: {}", x);
        }
    }
}

pub fn calculate2(input: &str) -> String {
    let mut registers = Registers { a: 0, b: 0, c: 0 };
    let mut program: Vec<u8> = Vec::with_capacity(0);
    input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .enumerate()
        .for_each(|(i, line)| {
            let (_l, r) = line.split_once(": ").unwrap();
            match i {
                0 => registers.a = r.parse::<u64>().unwrap(),
                1 => registers.b = r.parse::<u64>().unwrap(),
                2 => registers.c = r.parse::<u64>().unwrap(),
                3 => {
                    program = r.split(',').map(|x| x.parse::<u8>().unwrap()).collect();
                }
                _ => {
                    panic!()
                }
            }
        });

    fn find(a: u64, index: usize, program: &Vec<u8>, shift: usize) -> Option<u64> {
        let result = program[index];
        // for _ in 0..shift {
        //     print!(" ");
        // }
        // println!("looking for {result} at {index}, a: {a}, {:#b}", a);
        'outer: for ending in 0..8 {
            let mut registers = Registers {
                a: (a << 3) + ending,
                b: 0,
                c: 0,
            };
            let mut i = 0;
            loop {
                let operand = program[i + 1];
                match program[i] {
                    0 => {
                        let operand = get_combo_operand(&mut registers, operand);
                        registers.a = registers.a >> operand;
                    }
                    1 => {
                        registers.b ^= operand as u64;
                    }
                    2 => {
                        let operand = get_combo_operand(&mut registers, operand);
                        registers.b = operand & 7;
                    }
                    3 => {
                        // if registers.a != 0 {
                        //     i = operand as usize;
                        //     continue;
                        // }
                    }
                    4 => {
                        registers.b ^= registers.c;
                    }
                    5 => {
                        let operand = get_combo_operand(&registers, operand);

                        // for _ in 0..shift {
                        //     print!(" ");
                        // }
                        // println!("{result}; i: {ending} -> b: {}", operand & 7);
                        if result == (operand & 7) as u8 {
                            let a = (a << 3) + ending;
                            if index > 0 {
                                if let Some(a) = find(a, index - 1, program, shift + 1) {
                                    return Some(a);
                                } else {
                                    continue 'outer;
                                }
                            } else {
                                return Some(a);
                            }
                        } else {
                            continue 'outer;
                        }
                    }
                    6 => {
                        let operand = get_combo_operand(&registers, operand);
                        registers.b = registers.a >> operand;
                    }
                    7 => {
                        let operand = get_combo_operand(&registers, operand);
                        registers.c = registers.a >> operand;
                    }
                    x => {
                        panic!("Unexpected opcode: {}", x)
                    }
                }

                i += 2;
                if i >= program.len() {
                    break;
                }
            }
        }

        // for _ in 0..shift {
        //     print!(" ");
        // }
        // println!("none for {result} at {index}, a: {a}, {:#b}", a);
        None
    }

    let result = find(0, program.len() - 1, &program,0).unwrap();
    format!("{result}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                Register A: 729
                Register B: 0
                Register C: 0

                Program: 0,1,5,4,3,0
            ",
        );
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0")
    }

    #[test]
    fn test_1_2() {
        let result = calculate1(
            "
                Register A: 10
                Register B: 0
                Register C: 0

                Program: 5,0,5,1,5,4
            ",
        );
        assert_eq!(result, "0,1,2")
    }

    #[test]
    fn test_1_3() {
        let result = calculate1(
            "
                Register A: 2024
                Register B: 0
                Register C: 0

                Program: 0,1,5,4,3,0
            ",
        );
        assert_eq!(result, "4,2,5,6,7,7,7,7,3,1,0")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                Register A: 2024
                Register B: 0
                Register C: 0

                Program: 0,3,5,4,3,0
            ",
        );
        assert_eq!(result, "117440")
    }

    #[test]
    fn test_2__() {
        let result = calculate1(
            "
                Register A: 202975183645226
                Register B: 0
                Register C: 0

                Program: 2,4,1,1,7,5,0,3,1,4,4,4,5,5,3,0
            ",
        );
        assert_eq!(result, "2,4,1,1,7,5,0,3,1,4,4,4,5,5,3,0")
    }


    #[test]
    fn test_2_() {
        // 2,4,1,1,7,5,0,3,1,4,4,4,5,5,3,0
        // b = a & 7
        // b = b ^ 1
        // c = a >> b
        // a = a >> 3
        // b = b ^ 4
        // b = b ^ c
        // out << b & 7

        // let result = 0;

        // 0 ; c == a>>b = 0
        // b = b ^ c -> b==b==0
        // b^4 = 0 -> b == 4
        // b^1==4 -> b==5
        //
        // 3; a=101xxx
        //

        // let a = 5 << 3;
        let a = 0;
        for i in 0..8 {
            let a = a + i;
            let mut b = a & 7;
            b = b ^ 1;
            let c = a >> b;
            b = b ^ 4;
            b = b ^ c;
            println!("i: {i} -> b: {}", b & 7);
        }
    }
}

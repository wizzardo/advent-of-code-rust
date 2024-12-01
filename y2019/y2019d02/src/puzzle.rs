pub fn calculate1(input: &str) -> String {
    let mut vec: Vec<u32> = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            line.split(",").map(|it| it.parse::<u32>().unwrap()).collect()
        })
        // .inspect(|line| { dbg!(line); })
        .last()
        .unwrap();

    let arr = vec.as_mut_slice();

    let mut position: usize = 0;
    loop {
        match arr[position] {
            1 => {
                let a = arr[arr[position + 1] as usize];
                let b = arr[arr[position + 2] as usize];
                arr[arr[position + 3] as usize] = a + b;
                position += 4
            }
            2 => {
                let a = arr[arr[position + 1] as usize];
                let b = arr[arr[position + 2] as usize];
                arr[arr[position + 3] as usize] = a * b;
                position += 4
            }
            99 => { break; }
            _ => { panic!("unknown opcode {}", arr[position]) }
        }
    }

    return arr[0].to_string();
}


pub fn calculate2(input: &str) -> String {
    let vec: Vec<u32> = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            line.split(",").map(|it| it.parse::<u32>().unwrap()).collect()
        })
        // .inspect(|line| { dbg!(line); })
        .last()
        .unwrap();


    for noun in 1..99 {
        for verb in 1..99 {
            let mut copy = vec.clone();
            let arr = copy.as_mut_slice();

            arr[1] = noun;
            arr[2] = verb;

            let mut position: usize = 0;
            loop {
                match arr[position] {
                    1 => {
                        let a = arr[arr[position + 1] as usize];
                        let b = arr[arr[position + 2] as usize];
                        arr[arr[position + 3] as usize] = a + b;
                        position += 4
                    }
                    2 => {
                        let a = arr[arr[position + 1] as usize];
                        let b = arr[arr[position + 2] as usize];
                        arr[arr[position + 3] as usize] = a * b;
                        position += 4
                    }
                    99 => { break; }
                    _ => { panic!("unknown opcode {}", arr[position]) }
                }
            }

            if arr[0] == 19690720 {
                return (100 * noun + verb).to_string();
            }
        }
    }

    panic!("pair not found");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1("
                1,0,0,0,99
            "
        );
        assert_eq!(result, "2")
    }

    #[test]
    fn test_1_2() {
        let result = calculate1("
                2,3,0,3,99
            "
        );
        assert_eq!(result, "2")
    }


    #[test]
    fn test_1_3() {
        let result = calculate1("
                1,1,1,4,99,5,6,0,99
            "
        );
        assert_eq!(result, "30")
    }


    // #[test]
    // fn test_2_1() {
    //     let result = calculate2("
    //             1969
    //         "
    //     );
    //     assert_eq!(result, "966")
    // }
    //
    // #[test]
    // fn test_2_2() {
    //     let result = calculate2("
    //             100756
    //         "
    //     );
    //     assert_eq!(result, "50346")
    // }
}
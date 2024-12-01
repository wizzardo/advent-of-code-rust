pub fn calculate1(input: &str) -> String {
    let sum: u32 = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let module: u32 = line.parse().unwrap();
            calculate_fuel(module)
        })
        // .inspect(|line| { dbg!(line); })
        .sum();


    return sum.to_string();
}

fn calculate_fuel(mut module: u32) -> u32 {
    module = module / 3;
    module = module.checked_sub(2).unwrap_or(0);
    module
}


pub fn calculate2(input: &str) -> String {
    let sum: u32 = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let module: u32 = line.parse().unwrap();
            let mut sum: u32 = 0;
            let mut additional_fuel: u32;
            additional_fuel = calculate_fuel(module);
            sum += additional_fuel;
            while additional_fuel > 0 {
                additional_fuel = calculate_fuel(additional_fuel);
                sum += additional_fuel;
            }
            sum
        })
        // .inspect(|line| { dbg!(line); })
        .sum();


    return sum.to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1("
                1969
            "
        );
        assert_eq!(result, "654")
    }

    #[test]
    fn test_1_2() {
        let result = calculate1("
                100756
            "
        );
        assert_eq!(result, "33583")
    }


    #[test]
    fn test_2_1() {
        let result = calculate2("
                1969
            "
        );
        assert_eq!(result, "966")
    }

    #[test]
    fn test_2_2() {
        let result = calculate2("
                100756
            "
        );
        assert_eq!(result, "50346")
    }
}
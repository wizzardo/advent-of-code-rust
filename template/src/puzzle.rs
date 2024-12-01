
pub fn calculate1(input: &str) -> String {
    input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
        });
    format!("{result}")
}

pub fn calculate2(input: &str) -> String {
    input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
        });
    format!("{result}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                to be replace
            ",
        );
        assert_eq!(result, "to be replace")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                to be replace
            ",
        );
        assert_eq!(result, "to be replace")
    }
}

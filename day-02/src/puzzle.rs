use nom::FindSubstring;

pub fn calculate1(input: &str) -> String {
    let sum: u32 = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let max_red = 12;
            let max_green = 13;
            let max_blue = 14;

            let game = get_game_number(line);
            let red = get_max_cubes_number(line, COLOR_RED);
            let green = get_max_cubes_number(line, COLOR_GREED);
            let blue = get_max_cubes_number(line, COLOR_BLUE);

            if red <= max_red && green <= max_green && blue <= max_blue {
                return game;
            } else {
                return 0;
            };
        })
        // .inspect(|line| { dbg!(line); })
        .sum();

    return sum.to_string();
}

pub fn calculate2(input: &str) -> String {
    let sum: u32 = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let red = get_max_cubes_number(line, COLOR_RED);
            let green = get_max_cubes_number(line, COLOR_GREED);
            let blue = get_max_cubes_number(line, COLOR_BLUE);
            return red * green * blue;
        })
        // .inspect(|line| { dbg!(line); })
        .sum();

    return sum.to_string();
}

fn get_game_number(line: &str) -> u32 {
    let start = line.find_substring(" ").unwrap() + 1;
    let end = line.find_substring(":").unwrap();
    let int: u32 = (&line[start..end]).parse().unwrap();
    return int;
}

static COLOR_RED: &str = "red";
static COLOR_GREED: &str = "green";
static COLOR_BLUE: &str = "blue";

fn get_max_cubes_number(line: &str, color: &str) -> u32 {
    let mut max: u32 = 0;
    let mut substr = line;

    loop {
        let find = substr.find_substring(color);
        if let None = find {
            break;
        }

        let number_end = find.unwrap() - 1;
        let number_start = get_last_from_end_digit_char_index(substr, number_end);
        let count: u32 = (&substr[number_start..number_end]).parse().unwrap();
        if count > max {
            max = count;
        }
        substr = &substr[find.unwrap() + color.len()..];
    }

    return max;
}

fn get_last_from_end_digit_char_index(s: &str, from: usize) -> usize {
    let mut last_index = from;
    while last_index > 0 && s.chars().nth(last_index - 1).unwrap().is_digit(10) {
        last_index = last_index - 1;
    }

    return last_index;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1("
                Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
            "
        );
        assert_eq!(result, "8")
    }

    #[test]
    fn test_2() {
        let result = calculate2("
                Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
            "
        );
        assert_eq!(result, "2286")
    }

    #[test]
    fn test_get_game_number() {
        assert_eq!(1, get_game_number("Game 1: 3 blue, 4 red;"));
        assert_eq!(13, get_game_number("Game 13: 3 blue, 4 red;"));
    }

    #[test]
    fn test_get_last_from_end_digit_char_index() {
        assert_eq!(1, get_last_from_end_digit_char_index(" 1: 3 blue, 4 red;", 1));
        assert_eq!(1, get_last_from_end_digit_char_index(" 12: 3 blue, 4 red;", 2));
        assert_eq!(0, get_last_from_end_digit_char_index("12: 3 blue, 4 red;", 1));
    }

    #[test]
    fn test_get_max_cubes_number() {
        assert_eq!(4, get_max_cubes_number("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", COLOR_RED));
        assert_eq!(2, get_max_cubes_number("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", COLOR_GREED));
        assert_eq!(6, get_max_cubes_number("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", COLOR_BLUE));
    }
}
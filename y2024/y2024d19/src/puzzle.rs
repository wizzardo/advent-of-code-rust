use std::collections::{HashMap, HashSet};

pub fn calculate1(input: &str) -> String {
    let mut patterns: HashSet<&str> = HashSet::new();
    let mut max_pattern_length = 0;
    let result = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .enumerate()
        .filter(|(i, line)| {
            if *i == 0 {
                line.split(", ").for_each(|it| {
                    if it.len() > max_pattern_length {
                        max_pattern_length = it.len();
                    }
                    patterns.insert(it);
                });
                return false;
            }

            fn match_pattern(design: &str, from: usize, to: usize, patterns: &HashSet<&str>) -> bool {
                patterns.contains(&design[from..to])
            }

            fn match_all<'a>(design: &'a str, patterns: &HashSet<&str>, max_pattern_length: usize, not_matched_cache: &mut HashSet<&'a str>) -> bool {
                if design.is_empty() {
                    return true;
                }
                for i in (0..design.len().min(max_pattern_length)).rev() {
                    if match_pattern(&design, 0, i + 1, patterns) {
                        let substr = &design[i + 1..];
                        if !not_matched_cache.contains(substr) && match_all(substr, &patterns, max_pattern_length, not_matched_cache) {
                            return true;
                        } else {
                            not_matched_cache.insert(substr);
                        }
                    }
                }
                false
            }

            let mut not_matched_cache: HashSet<&str> = HashSet::new();
            match_all(line, &patterns, max_pattern_length, &mut not_matched_cache)
        }).count();
    format!("{result}")
}

pub fn calculate2(input: &str) -> String {
    let mut patterns: HashSet<&str> = HashSet::new();
    let mut max_pattern_length = 0;
    let result: usize = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .enumerate()
        .map(|(i, line)| {
            if i == 0 {
                line.split(", ").for_each(|it| {
                    if it.len() > max_pattern_length {
                        max_pattern_length = it.len();
                    }
                    patterns.insert(it);
                });
                return 0;
            }

            fn match_pattern(design: &str, from: usize, to: usize, patterns: &HashSet<&str>) -> bool {
                patterns.contains(&design[from..to])
            }

            fn match_all<'a>(design: &'a str, patterns: &HashSet<&str>, max_pattern_length: usize, not_matched_cache: &mut HashSet<&'a str>, matched_cache: &mut HashMap<&'a str, usize>) -> usize {
                if design.is_empty() {
                    return 1;
                }
                if let Some(count) = matched_cache.get(design) {
                    return *count;
                }

                let mut count = 0;
                for i in (0..design.len().min(max_pattern_length)).rev() {
                    if match_pattern(&design, 0, i + 1, patterns) {
                        let substr = &design[i + 1..];
                        if !not_matched_cache.contains(substr) {
                            let c = match_all(substr, &patterns, max_pattern_length, not_matched_cache, matched_cache);
                            if c == 0 {
                                not_matched_cache.insert(substr);
                            } else {
                                matched_cache.insert(substr, c);
                                count += c;
                            }
                        }
                    }
                }
                count
            }

            let mut not_matched_cache: HashSet<&str> = HashSet::new();
            let mut matched_cache: HashMap<&str, usize> = HashMap::new();
            match_all(line, &patterns, max_pattern_length, &mut not_matched_cache, &mut matched_cache)
        })
        .sum();
    format!("{result}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                r, wr, b, g, bwu, rb, gb, br

                brwrr
                bggr
                gbbr
                rrbgbr
                ubwu
                bwurrg
                brgr
                bbrgwb
            ",
        );
        assert_eq!(result, "6")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                r, wr, b, g, bwu, rb, gb, br

                brwrr
                bggr
                gbbr
                rrbgbr
                ubwu
                bwurrg
                brgr
                bbrgwb
            ",
        );
        assert_eq!(result, "16")
    }
}

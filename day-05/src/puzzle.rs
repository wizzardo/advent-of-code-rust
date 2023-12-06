use std::cmp::{max, min};
use std::slice::Iter;
use nom::FindSubstring;

pub fn calculate1(input: &str) -> String {
    let mut numbers: [u64; 20] = [u64::MAX; 20];
    let mut mapper: [u64; 3] = [0; 3];
    let mut count: usize = 0;
    let mut checks: [bool; 20] = [false; 20];

    input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            if count == 0 && line.starts_with("seeds:") {
                let offset = line.find_substring(":").unwrap() + 1;
                count = read_numbers(line, offset, line.len(), &mut numbers);
                return;
            }
            if !line.chars().nth(0).unwrap().is_digit(10) {
                for i in 0..count {
                    checks[i] = false;
                }
                return;
            }
            if line.chars().nth(0).unwrap().is_digit(10) {
                read_numbers(line, 0, line.len(), &mut mapper);
                map_numbers(&mut numbers[0..count], &mapper, &mut checks)
            }
        });

    let min = numbers.iter().take(count).min().unwrap();
    return min.to_string();
}

fn map_numbers(numbers: &mut [u64], mapper: &[u64], checks: &mut [bool]) {
    for i in 0..numbers.len() {
        if checks[i] {
            continue;
        }

        let n = numbers[i];
        if n >= mapper[1] && n < mapper[1] + mapper[2] {
            numbers[i] = mapper[0] + n - mapper[1];
            checks[i] = true;
        }
    }
}

fn read_numbers(line: &str, mut from: usize, to: usize, into: &mut [u64]) -> usize {
    let mut count = 0;
    while from < to {
        if line.chars().nth(from).unwrap() <= ' ' {
            from += 1;
            continue;
        }

        let end = get_last_digit_char_index(line, from);
        into[count] = (&line[from..end]).parse().unwrap();
        count += 1;
        from = end
    }
    return count;
}

#[derive(Debug, Copy, Clone)]
struct Range {
    from: u64,
    length: u64,
}

pub fn calculate2(input: &str) -> String {
    let mut source: Vec<Range> = vec![];
    let mut dest: Vec<Range> = vec![];

    let mut mappings: [u64; 20] = [0; 20];
    let mut count: usize = 0;

    input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            if count == 0 && line.starts_with("seeds:") {
                let offset = line.find_substring(":").unwrap() + 1;
                count = read_numbers(line, offset, line.len(), &mut mappings);
                for i in (0..count).step_by(2) {
                    source.push(Range { from: mappings[i], length: mappings[i + 1] })
                }
                return;
            }

            if !line.chars().nth(0).unwrap().is_digit(10) {
                source.extend(dest.drain(..));
                return;
            }
            if line.chars().nth(0).unwrap().is_digit(10) {
                read_numbers(line, 0, line.len(), &mut mappings);
                map_ranges(&mut source, &mappings[0..3], &mut dest)
            }
        });

    source.extend(dest.drain(..));
    source.sort_by(|a, b| a.from.cmp(&b.from));

    return source[0].from.to_string();
}

fn map_ranges(source: &mut Vec<Range>, mapping: &[u64], dest: &mut Vec<Range>) {
    let mut i: usize = 0;
    let to = mapping[0];
    let b = &Range { from: mapping[1], length: mapping[2] };
    let mut tmp = [Range { from: 0, length: 0 }; 3];
    while i < source.len() {
        let a = source.get(i).unwrap();
        if overlaps(a, b) {
            if contains(b, &a) {
                dest.push(Range { from: to + a.from - b.from, length: a.length });
            } else {
                split(a, b, &mut tmp).for_each(|a: &Range| {
                    if overlaps(a, b) {
                        dest.push(Range { from: to + a.from - b.from, length: a.length });
                    } else {
                        source.push(*a);
                    }
                });
            }

            source.remove(i);
        } else {
            i += 1;
        }
    }
}


fn split<'a>(a: &Range, b: &Range, into: &'a mut [Range; 3]) -> Iter<'a, Range> {
    let mut count = 0;
    if a.from < b.from {
        into[count] = Range { from: a.from, length: b.from - a.from };
        count += 1;
        let l = if a.from + a.length > b.from + b.length {
            b.length
        } else {
            a.from + a.length - b.from
        };
        if l != 0 {
            into[count] = Range { from: b.from, length: l };
            count += 1;
        }
    } else {
        let l = b.from + b.length - a.from;
        if l != 0 {
            into[count] = Range { from: a.from, length: l };
            count += 1;
        }
    }
    if a.from + a.length > b.from + b.length {
        into[count] = Range { from: b.from + b.length, length: a.from + a.length - (b.from + b.length) };
        count += 1;
    }
    return (&into[0..count]).iter();
}

fn overlaps(a: &Range, b: &Range) -> bool {
    return min(a.from + a.length, b.from + b.length) > max(a.from, b.from);
}

fn contains(a: &Range, b: &Range) -> bool {
    return a.from <= b.from && b.from + b.length <= a.from + a.length;
}

// fn get_first_digit_char_index(s: &str, mid: usize) -> usize {
//     let mut i = mid;
//     while i > 0 && s.chars().nth(i - 1).unwrap().is_digit(10) {
//         i = i - 1;
//     }
//     return i;
// }

fn get_last_digit_char_index(s: &str, mid: usize) -> usize {
    let mut i = mid;
    while i < s.len() && s.chars().nth(i).unwrap().is_digit(10) {
        i = i + 1;
    }
    return i;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1("
                seeds: 79 14 55 13

                seed-to-soil map:
                50 98 2
                52 50 48

                soil-to-fertilizer map:
                0 15 37
                37 52 2
                39 0 15

                fertilizer-to-water map:
                49 53 8
                0 11 42
                42 0 7
                57 7 4

                water-to-light map:
                88 18 7
                18 25 70

                light-to-temperature map:
                45 77 23
                81 45 19
                68 64 13

                temperature-to-humidity map:
                0 69 1
                1 0 69

                humidity-to-location map:
                60 56 37
                56 93 4
            "
        );
        assert_eq!(result, "35")
    }

    #[test]
    fn test_2() {
        let result = calculate2("
                seeds: 79 14 55 13

                seed-to-soil map:
                50 98 2
                52 50 48

                soil-to-fertilizer map:
                0 15 37
                37 52 2
                39 0 15

                fertilizer-to-water map:
                49 53 8
                0 11 42
                42 0 7
                57 7 4

                water-to-light map:
                88 18 7
                18 25 70

                light-to-temperature map:
                45 77 23
                81 45 19
                68 64 13

                temperature-to-humidity map:
                0 69 1
                1 0 69

                humidity-to-location map:
                60 56 37
                56 93 4
            "
        );
        assert_eq!(result, "46")
    }

    #[test]
    fn test_overlaps() {
        assert_eq!(true, overlaps(&Range { from: 0, length: 2 }, &Range { from: 1, length: 1 }));
        assert_eq!(true, overlaps(&Range { from: 0, length: 3 }, &Range { from: 1, length: 1 }));
        assert_eq!(false, overlaps(&Range { from: 0, length: 1 }, &Range { from: 1, length: 1 }));
        assert_eq!(true, overlaps(&Range { from: 0, length: 4 }, &Range { from: 1, length: 2 }));
        assert_eq!(true, overlaps(&Range { from: 1, length: 1 }, &Range { from: 1, length: 4 }));
        assert_eq!(true, overlaps(&Range { from: 2, length: 1 }, &Range { from: 1, length: 4 }));
        assert_eq!(true, overlaps(&Range { from: 2, length: 5 }, &Range { from: 1, length: 4 }));
        assert_eq!(false, overlaps(&Range { from: 2, length: 1 }, &Range { from: 1, length: 1 }));
    }


    #[test]
    fn test_split_1() {
        let mut tmp = [Range { from: 0, length: 0 }; 3];
        let result = split(&Range { from: 57, length: 13 }, &Range { from: 53, length: 8 }, &mut tmp).as_slice();
        assert_eq!(2, result.len());
        assert_eq!(57, result[0].from);
        assert_eq!(4, result[0].length);
        assert_eq!(61, result[1].from);
        assert_eq!(9, result[1].length);
    }

    #[test]
    fn test_split_2() {
        let mut tmp = [Range { from: 0, length: 0 }; 3];
        let result = split(&Range { from: 10, length: 10 }, &Range { from: 14, length: 1 }, &mut tmp).as_slice();
        assert_eq!(3, result.len());
        assert_eq!(10, result[0].from);
        assert_eq!(4, result[0].length);
        assert_eq!(14, result[1].from);
        assert_eq!(1, result[1].length);
        assert_eq!(15, result[2].from);
        assert_eq!(5, result[2].length);
    }

    #[test]
    fn test_split_3() {
        let mut tmp = [Range { from: 0, length: 0 }; 3];
        let result = split(&Range { from: 10, length: 10 }, &Range { from: 15, length: 10 }, &mut tmp).as_slice();
        assert_eq!(2, result.len());
        assert_eq!(10, result[0].from);
        assert_eq!(5, result[0].length);
        assert_eq!(15, result[1].from);
        assert_eq!(5, result[1].length);
    }
}
use std::collections::{HashMap, HashSet};

pub fn calculate1(input: &str) -> String {
    let data: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| line.chars().collect())
        .collect();

    let width = data[0].len();
    let height = data.len();

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for y in 0..height {
        for x in 0..width {
            let it = data[y][x];
            if it.is_ascii_alphabetic() || it.is_ascii_digit() {
                antennas.entry(it).or_insert(vec![]).push((x as i32, y as i32));
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    let is_valid_point = |point: (i32, i32)| {
        if point.0 < 0 || point.1 < 0 || point.0 >= width as i32 || point.1 >= height as i32 {
            return false;
        }
        true
    };

    antennas.iter().for_each(|(_, coords)| {
        for i in 0..coords.len() - 1 {
            for j in i + 1..coords.len() {
                let l = coords[i];
                let r = coords[j];
                let diff = (r.0 - l.0, r.1 - l.1);

                let ll = (l.0 - diff.0, l.1 - diff.1);
                let rr = (r.0 + diff.0, r.1 + diff.1);
                if is_valid_point(ll) {
                    antinodes.insert(ll);
                }
                if is_valid_point(rr) {
                    antinodes.insert(rr);
                }
            }
        }
    });


    let result = antinodes.len();
    format!("{result}")
}

pub fn calculate2(input: &str) -> String {
    let data: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| line.chars().collect())
        .collect();

    let width = data[0].len();
    let height = data.len();

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for y in 0..height {
        for x in 0..width {
            let it = data[y][x];
            if it.is_ascii_alphabetic() || it.is_ascii_digit() {
                antennas.entry(it).or_insert(vec![]).push((x as i32, y as i32));
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    let is_valid_point = |point: (i32, i32)| {
        if point.0 < 0 || point.1 < 0 || point.0 >= width as i32 || point.1 >= height as i32 {
            return false;
        }
        true
    };

    antennas.iter().for_each(|(_, coords)| {
        for i in 0..coords.len() - 1 {
            let l = coords[i];
            antinodes.insert(l.clone());
            for j in i + 1..coords.len() {
                let r = coords[j];
                antinodes.insert(r.clone());
                let diff = (r.0 - l.0, r.1 - l.1);

                let mut ll = (l.0 - diff.0, l.1 - diff.1);
                while is_valid_point(ll) {
                    antinodes.insert(ll);
                    ll = (ll.0 - diff.0, ll.1 - diff.1);
                }
                let mut rr = (r.0 + diff.0, r.1 + diff.1);
                while is_valid_point(rr) {
                    antinodes.insert(rr);
                    rr = (rr.0 + diff.0, rr.1 + diff.1);
                }
            }
        }
    });


    let result = antinodes.len();
    format!("{result}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                ............
                ........0...
                .....0......
                .......0....
                ....0.......
                ......A.....
                ............
                ............
                ........A...
                .........A..
                ............
                ............
            ",
        );
        assert_eq!(result, "14")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                ............
                ........0...
                .....0......
                .......0....
                ....0.......
                ......A.....
                ............
                ............
                ........A...
                .........A..
                ............
                ............
            ",
        );
        assert_eq!(result, "34")
    }
}

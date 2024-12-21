use std::collections::{HashMap, HashSet};

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    pub fn all_directions() -> [Vector; 4] {
        [
            Vector { x: 1, y: 0 },
            Vector { x: 0, y: -1 },
            Vector { x: -1, y: 0 },
            Vector { x: 0, y: 1 },
        ]
    }
}

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

    let mut regions: HashMap<char, Vec<HashSet<(i32, i32)>>> = HashMap::new();
    fn expand_region(region: &mut HashSet<(i32, i32)>, x: i32, y: i32, name: char, data: &Vec<Vec<char>>, width: usize, height: usize) {
        if x < 0 || y < 0 || x as usize == width || y as usize == height || data[y as usize][x as usize] != name {
            return;
        }
        if !region.insert((x, y)) {
            return;
        }
        for v in Vector::all_directions() {
            expand_region(region, x + v.x, y + v.y, name, data, width, height);
        }
    }

    fn calculate_perimeter(region: &HashSet<(i32, i32)>) -> usize {
        let mut perimeter = 0;

        for point in region.iter() {
            for v in Vector::all_directions() {
                if !region.contains(&(point.0 + v.x, point.1 + v.y)) {
                    perimeter += 1;
                }
            }
        }

        perimeter
    }

    for y in 0..height {
        for x in 0..width {
            let it = data[y][x];
            let r = regions.entry(it).or_insert(Vec::new());
            let point = (x as i32, y as i32);
            if !r.iter().any(|region| region.contains(&point)) {
                let mut region = HashSet::new();
                expand_region(&mut region, x as i32, y as i32, it, &data, width, height);
                r.push(region);
            }
        }
    }

    let mut sum = 0;
    for (_c, regions) in regions.iter() {
        // println!("{c}: {:?}", regions);
        for region in regions {
            sum += region.len() * calculate_perimeter(&region);
        }
    }

    let result = sum;
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

    let mut regions: HashMap<char, Vec<HashSet<(i32, i32)>>> = HashMap::new();
    fn expand_region(region: &mut HashSet<(i32, i32)>, x: i32, y: i32, name: char, data: &Vec<Vec<char>>, width: usize, height: usize) {
        if x < 0 || y < 0 || x as usize == width || y as usize == height || data[y as usize][x as usize] != name {
            return;
        }
        if !region.insert((x, y)) {
            return;
        }
        for v in Vector::all_directions() {
            expand_region(region, x + v.x, y + v.y, name, data, width, height);
        }
    }

    fn calculate_perimeter(region: &HashSet<(i32, i32)>) -> usize {
        let mut sides: HashMap<(i32, i32), Vec<Vec<(i32, i32)>>> = HashMap::new();

        for point in region.iter() {
            for v in Vector::all_directions() {
                if !region.contains(&(point.0 + v.x, point.1 + v.y)) {
                    match v {
                        Vector { x: 1, y: 0 } => {
                            let sides = sides.entry((point.0 + v.x, -1)).or_insert(Vec::new());
                            if let Some(side) = sides.iter_mut().find(|side| side.iter().any(|(x, y)| { *x == point.0 && (point.1 - y).abs() == 1 })) {
                                side.push(point.clone());
                            } else {
                                sides.push(vec![point.clone()]);
                            }
                        }
                        Vector { x: -1, y: 0 } => {
                            let sides = sides.entry((point.0, -1)).or_insert(Vec::new());
                            if let Some(side) = sides.iter_mut().find(|side| side.iter().any(|(x, y)| { *x == point.0 && (point.1 - y).abs() == 1 })) {
                                side.push(point.clone());
                            } else {
                                sides.push(vec![point.clone()]);
                            }
                        }
                        Vector { x: 0, y: -1 } => {
                            let sides = sides.entry((-1, point.1)).or_insert(Vec::new());
                            if let Some(side) = sides.iter_mut().find(|side| side.iter().any(|(x, y)| { *y == point.1 && (point.0 - x).abs() == 1 })) {
                                side.push(point.clone());
                            } else {
                                sides.push(vec![point.clone()]);
                            }
                        }
                        Vector { x: 0, y: 1 } => {
                            let sides = sides.entry((-1, point.1 + v.y)).or_insert(Vec::new());
                            if let Some(side) = sides.iter_mut().find(|side| side.iter().any(|(x, y)| { *y == point.1 && (point.0 - x).abs() == 1 })) {
                                side.push(point.clone());
                            } else {
                                sides.push(vec![point.clone()]);
                            }
                        }
                        _ => { panic!() }
                    };
                }
            }
        }

        //merge continuous sides
        for (_, sides) in sides.iter_mut() {
            if sides.len() > 1 {
                'outer: for i in (1..sides.len()).rev() {
                    let side = &sides[i];
                    for j in 0..i {
                        let other = &sides[j];
                        if other.iter().any(|a| side.iter().any(|b| (a.0 - b.0).abs() + (a.1 - b.1).abs() == 1)) {
                            let removed = sides.remove(i);
                            sides.get_mut(j).unwrap().extend(removed);
                            continue 'outer;
                        }
                    }
                }
            }
        }

        // println!("sides: {:?}", sides);

        sides.iter().map(|(_, sides)| sides.len()).sum()
    }

    for y in 0..height {
        for x in 0..width {
            let it = data[y][x];
            let r = regions.entry(it).or_insert(Vec::new());
            let point = (x as i32, y as i32);
            if !r.iter().any(|region| region.contains(&point)) {
                let mut region = HashSet::new();
                expand_region(&mut region, x as i32, y as i32, it, &data, width, height);
                r.push(region);
            }
        }
    }

    let mut sum = 0;
    for (_c, regions) in regions.iter() {
        // println!("{c}: {:?}", regions);
        for region in regions {
            let perimeter = calculate_perimeter(&region);
            // println!("{_c}: {:?}; p: {}", regions, perimeter);
            sum += region.len() * perimeter;
        }
    }

    let result = sum;
    format!("{result}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                AAAA
                BBCD
                BBCC
                EEEC
            ",
        );
        assert_eq!(result, "140")
    }

    #[test]
    fn test_1_2() {
        let result = calculate1(
            "
                RRRRIICCFF
                RRRRIICCCF
                VVRRRCCFFF
                VVRCCCJFFF
                VVVVCJJCFE
                VVIVCCJJEE
                VVIIICJJEE
                MIIIIIJJEE
                MIIISIJEEE
                MMMISSJEEE
            ",
        );
        assert_eq!(result, "1930")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                AAAA
                BBCD
                BBCC
                EEEC
            ",
        );
        assert_eq!(result, "80")
    }

    #[test]
    fn test_2_2() {
        let result = calculate2(
            "
                AAAAAA
                AAABBA
                AAABBA
                ABBAAA
                ABBAAA
                AAAAAA
            ",
        );
        assert_eq!(result, "368")
    }

    #[test]
    fn test_2_23() {
        let result = calculate2(
            "
                RRRRIICCFF
                RRRRIICCCF
                VVRRRCCFFF
                VVRCCCJFFF
                VVVVCJJCFE
                VVIVCCJJEE
                VVIIICJJEE
                MIIIIIJJEE
                MIIISIJEEE
                MMMISSJEEE
            ",
        );
        assert_eq!(result, "1206")
    }
}

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

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

    pub fn distance(&self, other: Vector) -> usize {
        (self.x - other.x).abs() as usize + (self.y - other.y).abs() as usize
    }
}

#[derive(Eq, PartialEq, Debug)]
struct VectorScore {
    v: Vector,
    score: usize,
}

impl PartialOrd<Self> for VectorScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VectorScore {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

pub fn calculate1(input: &str, width: usize, height: usize, limit: usize) -> String {
    let bytes : Vec<(u8,u8)> = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<u8>().unwrap(), y.parse::<u8>().unwrap())
        }).collect();

    let mut map: Vec<Vec<char>> = Vec::with_capacity(height);
    for _ in 0..height {
        map.push(vec!['.'; width]);
    }

    for i in 0..limit {
        let (x, y) = bytes[i];
        map[y as usize][x as usize] = '#';
    }

    // for y in 0..height {
    //     for x in 0..width {
    //         print!("{}", map[y][x]);
    //     }
    //     println!("");
    // }
    // print!("");

    let start = Vector { x: 0, y: 0 };
    let to = Vector { x: (width - 1) as i32, y: (height - 1) as i32 };

    let heuristic = |v: Vector| -> usize { v.distance(to) };

    fn find_path(from: Vector, to: Vector, map: &Vec<Vec<char>>, heuristic: &dyn Fn(Vector) -> usize) -> Option<Vec<Vector>> {
        let mut queue: BinaryHeap<VectorScore> = BinaryHeap::new();
        let mut prev_steps: HashMap<Vector, Vector> = HashMap::new();
        let mut scores: HashMap<Vector, usize> = HashMap::new();
        // let mut estimated_scores: HashMap<Vector, usize> = HashMap::new();

        scores.insert(from, 0);
        // estimated_scores.insert(from, heuristic(from));
        queue.push(VectorScore { v: from, score: heuristic(from) });

        loop {
            let current = queue.pop();
            if current.is_none() {
                break;
            }
            let current = current.unwrap();
            if current.v == to {
                let mut path: Vec<Vector> = Vec::new();
                let mut step = to;
                path.push(to);
                while let Some(v) = prev_steps.get(&step) {
                    path.push(*v);
                    step = *v;
                }

                return Some(path);
            }

            for d in Vector::all_directions() {
                let next = Vector { x: d.x + current.v.x, y: d.y + current.v.y };
                if next.x < 0 || next.y < 0 || next.y as usize == map.len() || next.x as usize == map[0].len() || map[next.y as usize][next.x as usize] == '#' {
                    continue;
                }
                let tentative_score = scores.get(&current.v).unwrap() + 1;
                if tentative_score < *scores.get(&next).unwrap_or(&usize::MAX) {
                    prev_steps.insert(next, current.v);
                    scores.insert(next, tentative_score);
                    // estimated_scores.insert(next, tentative_score + heuristic(next));
                    // if neighbor not in openSet
                    // openSet.add(neighbor)
                    queue.push(VectorScore { v: next, score: tentative_score + heuristic(next) });
                }
            }
        }

        None
    }

    let path = find_path(start, to, &map, &heuristic).unwrap();

    for v in &path {
        map[v.y as usize][v.x as usize] = 'O';
    }

    for y in 0..height {
        for x in 0..width {
            print!("{}", map[y][x]);
        }
        println!("");
    }
    print!("");

    let result = path.len() - 1;
    format!("{result}")
}

pub fn calculate2(input: &str, width: usize, height: usize, check_each_after: usize) -> String {
    let bytes : Vec<(u8,u8)> = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<u8>().unwrap(), y.parse::<u8>().unwrap())
        }).collect();

    let mut map: Vec<Vec<char>> = Vec::with_capacity(height);
    for _ in 0..height {
        map.push(vec!['.'; width]);
    }

    for i in 0..check_each_after {
        let (x, y) = bytes[i];
        map[y as usize][x as usize] = '#';
    }

    let start = Vector { x: 0, y: 0 };
    let to = Vector { x: (width - 1) as i32, y: (height - 1) as i32 };

    let heuristic = |v: Vector| -> usize { v.distance(to) };

    fn find_path(from: Vector, to: Vector, map: &Vec<Vec<char>>, heuristic: &dyn Fn(Vector) -> usize) -> Option<Vec<Vector>> {
        let mut queue: BinaryHeap<VectorScore> = BinaryHeap::new();
        let mut prev_steps: HashMap<Vector, Vector> = HashMap::new();
        let mut scores: HashMap<Vector, usize> = HashMap::new();
        // let mut estimated_scores: HashMap<Vector, usize> = HashMap::new();

        scores.insert(from, 0);
        // estimated_scores.insert(from, heuristic(from));
        queue.push(VectorScore { v: from, score: heuristic(from) });

        loop {
            let current = queue.pop();
            if current.is_none() {
                break;
            }
            let current = current.unwrap();
            if current.v == to {
                let mut path: Vec<Vector> = Vec::new();
                let mut step = to;
                path.push(to);
                while let Some(v) = prev_steps.get(&step) {
                    path.push(*v);
                    step = *v;
                }

                return Some(path);
            }

            for d in Vector::all_directions() {
                let next = Vector { x: d.x + current.v.x, y: d.y + current.v.y };
                if next.x < 0 || next.y < 0 || next.y as usize == map.len() || next.x as usize == map[0].len() || map[next.y as usize][next.x as usize] == '#' {
                    continue;
                }
                let tentative_score = scores.get(&current.v).unwrap() + 1;
                if tentative_score < *scores.get(&next).unwrap_or(&usize::MAX) {
                    prev_steps.insert(next, current.v);
                    scores.insert(next, tentative_score);
                    // estimated_scores.insert(next, tentative_score + heuristic(next));
                    // if neighbor not in openSet
                    // openSet.add(neighbor)
                    queue.push(VectorScore { v: next, score: tentative_score + heuristic(next) });
                }
            }
        }

        None
    }

    let mut result = "".to_string();
    for i in check_each_after..bytes.len() {
        let (x, y) = bytes[i];
        map[y as usize][x as usize] = '#';
        if find_path(start, to, &map, &heuristic).is_none() {
            result = format!("{x},{y}");
            break
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                5,4
                4,2
                4,5
                3,0
                2,1
                6,3
                2,4
                1,5
                0,6
                3,3
                2,6
                5,1
                1,2
                5,5
                2,5
                6,5
                1,4
                0,4
                6,4
                1,1
                6,1
                1,0
                0,5
                1,6
                2,0
            ", 7, 7, 12,
        );
        assert_eq!(result, "22")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                5,4
                4,2
                4,5
                3,0
                2,1
                6,3
                2,4
                1,5
                0,6
                3,3
                2,6
                5,1
                1,2
                5,5
                2,5
                6,5
                1,4
                0,4
                6,4
                1,1
                6,1
                1,0
                0,5
                1,6
                2,0
            ", 7, 7, 12,
        );
        assert_eq!(result, "6,1")
    }
}

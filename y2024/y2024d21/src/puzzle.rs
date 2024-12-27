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
            Vector { x: -1, y: 0 },
            Vector { x: 0, y: -1 },
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


struct Path {
    nodes: Vec<Vector>,
}

pub fn calculate1(input: &str) -> String {
    let result: usize = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let size = find_min_path(line);
            let n = &line[0..line.len() - 1].parse::<usize>().unwrap();
            // println!("{line}: {size}");
            n * size
        })
        .sum();
    format!("{result}")
}

pub fn calculate2(input: &str) -> String {
    let result: usize = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let paths = find_num_pad_path(line);
            let size = paths.iter().map(|x| {
                let i = 25;
                let mut cache = HashMap::new();
                calculate_min_dir_path(x.as_str(), i, &mut cache)
            }).min().unwrap();

            // println!("{line}: {size}");
            let n = &line[0..line.len() - 1].parse::<usize>().unwrap();
            n * size
        })
        .sum();
    format!("{result}")
}

fn find_paths(from: Vector, to: Vector, width: usize, height: usize, map: &[char]) -> Option<Vec<Path>> {
    let mut min: HashMap<Vector, Vec<(usize, Vector)>> = HashMap::new();
    let mut queue: BinaryHeap<VectorScore> = BinaryHeap::new();

    min.insert(from, vec![(0, from)]);

    queue.push(VectorScore { score: 0, v: from });
    let mut min_score: usize = usize::MAX;

    let directions = Vector::all_directions();

    loop {
        let pop = queue.pop();
        if pop.is_none() {
            break;
        }

        let current = pop.unwrap();
        if current.score > min_score {
            continue;
        }

        for d in &directions {
            let c = Vector { x: current.v.x + d.x, y: current.v.y + d.y };
            if c.x < 0 || c.y < 0 || c.y as usize == height || c.x as usize == width || map[(c.y as usize) * width + c.x as usize] == '#' {
                continue;
            }

            let next_score = current.score + (c.x - current.v.x).abs() as usize + (c.y - current.v.y).abs() as usize;
            if next_score > min_score {
                continue;
            }
            if c == to && next_score < min_score {
                min_score = next_score;
            }


            let old = min.get_mut(&c);
            match old {
                None => {
                    min.insert(c, vec![(next_score, current.v)]);
                    queue.push(VectorScore { score: next_score, v: c });
                }
                Some(old) => {
                    if old[0].0 > next_score {
                        min.insert(c, vec![(next_score, current.v)]);
                        queue.push(VectorScore { score: next_score, v: c });
                    } else if old[0].0 == next_score {
                        if !old.contains(&(next_score, current.v)) {
                            old.push((next_score, current.v))
                        }
                    }
                }
            }
        }
    }

    let mut paths: Vec<Vec<Vector>> = Vec::new();
    paths.push(vec![to]);
    loop {
        if paths.iter().all(|path| *path.last().unwrap() == from) {
            break;
        }
        for i in 0..paths.len() {
            if *paths[i].last().unwrap() == from {
                continue;
            }
            let position = paths[i].last().unwrap();
            let options = min.get(position).unwrap();
            if options.len() > 1 {
                for j in 1..options.len() {
                    let mut another = paths[i].clone();
                    another.push(options[j].1);
                    paths.push(another);
                }
                paths[i].push(options[0].1);
            } else {
                paths[i].push(options[0].1);
            }
        }
    }

    Some(paths.into_iter().map(|mut nodes| {
        nodes.reverse();
        Path { nodes: nodes }
    }).collect())
}


fn find_path(from: Vector, to: Vector, width: usize, height:usize, map: &[char], heuristic: &dyn Fn(Vector, Vector) -> usize) -> Option<Vec<Vector>> {
    let mut queue: BinaryHeap<VectorScore> = BinaryHeap::new();
    let mut prev_steps: HashMap<Vector, Vector> = HashMap::new();
    let mut scores: HashMap<Vector, usize> = HashMap::new();
    // let mut estimated_scores: HashMap<Vector, usize> = HashMap::new();

    scores.insert(from, 0);
    // estimated_scores.insert(from, heuristic(from));
    queue.push(VectorScore { v: from, score: heuristic(from, from) });

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
            path.reverse();

            return Some(path);
        }

        for d in Vector::all_directions() {
            let next = Vector { x: d.x + current.v.x, y: d.y + current.v.y };
            if next.x < 0 || next.y < 0 || next.y as usize == height || next.x as usize == width || map[(next.y as usize) * width + next.x as usize] == '#' {
                continue;
            }
            let tentative_score = scores.get(&current.v).unwrap() + 1;
            if tentative_score < *scores.get(&next).unwrap_or(&usize::MAX) {
                prev_steps.insert(next, current.v);
                scores.insert(next, tentative_score);
                // estimated_scores.insert(next, tentative_score + heuristic(next));
                // if neighbor not in openSet
                // openSet.add(neighbor)
                let score = tentative_score + heuristic(next, prev_steps.get(&current.v).unwrap_or(&current.v).clone());
                queue.push(VectorScore { v: next, score: score });
            }
        }
    }

    None
}

fn find_num_pad_path(code: &str) -> Vec<String> {
    let num_pad = [
        '7', '8', '9',
        '4', '5', '6',
        '1', '2', '3',
        '#', '0', 'A'
    ];
    let width = 3;
    let height = 4;
    let mut char_positions: [i8; 128] = [-1; 128];
    for i in 0..num_pad.len() {
        char_positions[num_pad[i] as usize] = i as i8;
    }

    let start = char_positions['A' as usize] as usize;
    let mut start = Vector { x: (start % width) as i32, y: (start / width) as i32 };


    // let mut steps: Vec<char> = Vec::new();
    let mut steps: Vec<Vec<char>> = Vec::new();

    for c in code.chars() {
        let to = char_positions[c as usize] as usize;
        let to = Vector { x: (to % width) as i32, y: (to / width) as i32 };

        // let heuristic = |v: Vector, prev: Vector| -> usize { v.distance(to) + if prev.x != v.x && prev.y != v.y { 1 } else { 0 } };
        // let path = find_path(start, to, width, height, &num_pad, &heuristic).unwrap();
        // path_to_buttons(path, &mut steps);
        let paths = find_paths(start, to, width, height, &num_pad).unwrap();
        if paths.len() != 1 {
            if steps.is_empty() {
                for i in 0..paths.len() {
                    let mut v = Vec::new();
                    path_to_buttons(&paths[i].nodes, &mut v);
                    steps.push(v);
                }
            } else {
                let another = steps.clone();
                let mut next_steps: Vec<Vec<char>> = Vec::with_capacity(steps.len() * paths.len());
                for i in 0..paths.len() {
                    for mut branch in another.clone() {
                        path_to_buttons(&paths[i].nodes, &mut branch);
                        next_steps.push(branch);
                    }
                }
                steps = next_steps;
            }
        } else {
            if steps.is_empty() {
                steps.push(Vec::new());
            }
            for i in 0..steps.len() {
                path_to_buttons(&paths[0].nodes, &mut steps[i]);
            }
        }

        start = to;
    }


    let mut steps: Vec<(Vec<char>, usize)> = steps.into_iter()
        .map(|v| {
            let mut distance = 0;
            let mut position = dir_button_position('A');
            for c in &v {
                let next = dir_button_position(*c);
                distance += position.distance(next);
                position = next;
            }
            // println!("{:?}: {}", &v, distance);
            (v, distance)
        })
        .collect();

    steps.sort_by(|a, b| a.1.cmp(&b.1));
    while steps.last().unwrap().1 > steps.first().unwrap().1 {
        steps.remove(steps.len() - 1);
    }

    steps.iter().map(|x| x.0.iter().collect()).collect()
}

fn dir_button_position(c: char) -> Vector {
    match c {
        '^' => Vector { x: 1, y: 0 },
        'A' => Vector { x: 2, y: 0 },
        '<' => Vector { x: 0, y: 1 },
        'v' => Vector { x: 1, y: 1 },
        '>' => Vector { x: 2, y: 1 },
        _ => { panic!() }
    }
}

fn dir_position_to_button(c: Vector) -> char {
    match (c.x, c.y) {
        (1, 0) => '^',
        (2, 0) => 'A',
        (0, 1) => '<',
        (1, 1) => 'v',
        (2, 1) => '>',
        _ => { panic!() }
    }
}

#[allow(unused)]
fn num_position_to_button(c: Vector) -> char {
    match (c.x, c.y) {
        (0, 0) => '7',
        (1, 0) => '8',
        (2, 0) => '9',
        (0, 1) => '4',
        (1, 1) => '5',
        (2, 1) => '6',
        (0, 2) => '1',
        (1, 2) => '2',
        (2, 2) => '3',
        // (0, 3) => '#',
        (1, 3) => '0',
        (2, 3) => 'A',
        x => {
            panic!("unknown coordinate: {x:?}")
        }
    }
}

#[allow(unused)]
fn execute_dir(code: &str) -> String {
    let mut chars = Vec::<char>::new();
    let mut position = dir_button_position('A');
    for (i,c) in code.chars().enumerate() {
        match c {
            'A' => {
                chars.push(dir_position_to_button(position));
            }
            '>' => {
                position.x += 1;
            }
            '<' => {
                position.x -= 1;
            }
            '^' => {
                position.y -= 1;
            }
            'v' => {
                position.y += 1;
            }
            _ => { panic!() }
        }
        if position.x == 0 && position.y == 0 {
            panic!()
        }
        if position.x < 0 || position.x > 2 {
            panic!()
        }
        if position.y < 0 || position.y > 1 {
            panic!()
        }
    }
    chars.iter().collect()
}

#[allow(unused)]
fn execute_num(code: &str) -> String {
    let mut chars = Vec::<char>::new();
    let mut position = Vector { x: 2, y: 3 };
    for c in code.chars() {
        match c {
            'A' => {
                chars.push(num_position_to_button(position));
            }
            '>' => {
                position.x += 1;
            }
            '<' => {
                position.x -= 1;
            }
            '^' => {
                position.y -= 1;
            }
            'v' => {
                position.y += 1;
            }
            _ => { panic!() }
        }
        if position.x == 0 && position.y == 3 {
            panic!()
        }
        if position.x < 0 || position.x > 2 {
            panic!()
        }
        if position.y < 0 || position.y > 3 {
            panic!()
        }
    }
    chars.iter().collect()
}

fn find_dir_pad_paths(code: &str) -> Vec<String> {
    let dir_pad = [
        '#', '^', 'A',
        '<', 'v', '>',
    ];
    let width = 3;
    let height = 2;
    let mut char_positions: [i8; 128] = [-1; 128];
    for i in 0..dir_pad.len() {
        char_positions[dir_pad[i] as usize] = i as i8;
    }

    let start = char_positions['A' as usize] as usize;
    let mut start = Vector { x: (start % width) as i32, y: (start / width) as i32 };


    let mut steps: Vec<Vec<char>> = Vec::new();

    for c in code.chars() {
        let to = char_positions[c as usize] as usize;
        let to = Vector { x: (to % width) as i32, y: (to / width) as i32 };

        let paths = find_paths(start, to, width, height, &dir_pad).unwrap();
        if paths.len() != 1 {
            if steps.is_empty() {
                for i in 0..paths.len() {
                    let mut v = Vec::new();
                    path_to_buttons(&paths[i].nodes, &mut v);
                    steps.push(v);
                }
            } else {
                let another = steps.clone();
                let mut next_steps: Vec<Vec<char>> = Vec::with_capacity(steps.len() * paths.len());
                for i in 0..paths.len() {
                    for mut branch in another.clone() {
                        path_to_buttons(&paths[i].nodes, &mut branch);
                        next_steps.push(branch);
                    }
                }
                steps = next_steps;
            }
        } else {
            if steps.is_empty() {
                steps.push(Vec::new());
            }
            for i in 0..steps.len() {
                path_to_buttons(&paths[0].nodes, &mut steps[i]);
            }
        }

        start = to;
    }


    let mut steps: Vec<(Vec<char>, usize)> = steps.into_iter()
        .map(|v| {
            let mut distance = 0;
            let mut position = dir_button_position('A');
            for c in &v {
                let next = dir_button_position(*c);
                distance += position.distance(next);
                position = next;
            }
            // println!("{}: {}", &v.iter().collect::<String>(), distance);
            (v, distance)
        })
        .collect();

    steps.sort_by(|a, b| a.1.cmp(&b.1));

    while steps.last().unwrap().1 > steps.first().unwrap().1 {
        steps.remove(steps.len() - 1);
    }

    steps.iter().map(|x| x.0.iter().collect()).collect()
}

fn find_dir_pad_path(code: &str) -> String {
    let dir_pad = [
        '#', '^', 'A',
        '<', 'v', '>',
    ];
    let width = 3;
    let height = 2;
    let mut char_positions: [i8; 128] = [-1; 128];
    for i in 0..dir_pad.len() {
        char_positions[dir_pad[i] as usize] = i as i8;
    }

    let start = char_positions['A' as usize] as usize;
    let mut start = Vector { x: (start % width) as i32, y: (start / width) as i32 };


    let mut steps: Vec<char> = Vec::new();

    for c in code.chars() {
        let to = char_positions[c as usize] as usize;
        let to = Vector { x: (to % width) as i32, y: (to / width) as i32 };

        let heuristic = |v: Vector, prev: Vector| -> usize { v.distance(to) + if prev.x != v.x && prev.y != v.y { 10 } else { 0 } };

        let path = find_path(start, to, width, height, &dir_pad, &heuristic).unwrap();
        path_to_buttons(&path, &mut steps);

        start = to;
    }

    steps.iter().collect()
}

fn calculate_min_dir_path<'a>(code: &'a str, depth: usize, cache: &mut HashMap<(&'a str, usize), usize>) -> usize {
    if depth == 0 {
        return code.len();
    }
    if let Some(size) = cache.get(&(code, depth)) {
        return *size;
    }

    let mut current = 'A';
    let mut sum = 0;
    for c in code.chars() {
        match (current, c) {
            ('A','A') => {
                sum += calculate_min_dir_path("A", depth - 1, cache);
            },
            ('A','^') => {
                sum += calculate_min_dir_path("<A", depth - 1, cache);
            },
            ('A','>') => {
                sum += calculate_min_dir_path("vA", depth - 1, cache);
            },
            ('A','v') => {
                // println!("{} v<A {}", depth-1, find_dir_pad_path3("v<A", depth - 1, cache));
                // println!("{} <vA {}", depth-1, find_dir_pad_path3("<vA", depth - 1, cache));
                // sum += find_dir_pad_path3("v<A", depth - 1, cache);
                sum += calculate_min_dir_path("<vA", depth - 1, cache);
            },
            ('A','<') => {
                sum += calculate_min_dir_path("v<<A", depth - 1, cache);
            },

            ('^','A') => {
                sum += calculate_min_dir_path(">A", depth - 1, cache);
            },
            ('^','^') => {
                sum += calculate_min_dir_path("A", depth - 1, cache);
            },
            ('^','>') => {
                // println!("{} v>A {}", depth-1, find_dir_pad_path3("v>A", depth - 1, cache));
                // println!("{} >vA {}", depth-1, find_dir_pad_path3(">vA", depth - 1, cache));
                sum += calculate_min_dir_path("v>A", depth - 1, cache);
            },
            ('^','v') => {
                sum += calculate_min_dir_path("vA", depth - 1, cache);
            },
            ('^','<') => {
                sum += calculate_min_dir_path("v<A", depth - 1, cache);
            },

            ('>','A') => {
                sum += calculate_min_dir_path("^A", depth - 1, cache);
            },
            ('>','^') => {
                // println!("{} ^<A {}", depth-1, find_dir_pad_path3("^<A", depth - 1, cache));
                // println!("{} <^A {}", depth-1, find_dir_pad_path3("<^A", depth - 1, cache));
                sum += calculate_min_dir_path("<^A", depth - 1, cache);
            },
            ('>','>') => {
                sum += calculate_min_dir_path("A", depth - 1, cache);
            },
            ('>','v') => {
                sum += calculate_min_dir_path("<A", depth - 1, cache);
            },
            ('>','<') => {
                sum += calculate_min_dir_path("<<A", depth - 1, cache);
            },

            ('v','A') => {
                // println!("{} ^>A {}", depth-1, find_dir_pad_path3("^>A", depth - 1, cache));
                // println!("{} >^A {}", depth-1, find_dir_pad_path3(">^A", depth - 1, cache));
                sum += calculate_min_dir_path("^>A", depth - 1, cache);
            },
            ('v','^') => {
                sum += calculate_min_dir_path("^A", depth - 1, cache);
            },
            ('v','>') => {
                sum += calculate_min_dir_path(">A", depth - 1, cache);
            },
            ('v','v') => {
                sum += calculate_min_dir_path("A", depth - 1, cache);
            },
            ('v','<') => {
                sum += calculate_min_dir_path("<A", depth - 1, cache);
            },

            ('<','A') => {
                sum += calculate_min_dir_path(">>^A", depth - 1, cache);
            },
            ('<','^') => {
                sum += calculate_min_dir_path(">^A", depth - 1, cache);
            },
            ('<','>') => {
                sum += calculate_min_dir_path(">>A", depth - 1, cache);
            },
            ('<','v') => {
                sum += calculate_min_dir_path(">A", depth - 1, cache);
            },
            ('<','<') => {
                sum += calculate_min_dir_path("A", depth - 1, cache);
            },
            x => { panic!("unknown combination {x:?}") }
        }
        current = c;
    }
    cache.insert((code, depth), sum);
    sum
}

fn path_to_buttons(path: &Vec<Vector>, steps: &mut Vec<char>) {
    for w in path.windows(2) {
        if w[0].x == w[1].x {
            if w[1].y < w[0].y {
                steps.push('^');
            } else {
                steps.push('v');
            }
        } else {
            if w[1].x < w[0].x {
                steps.push('<');
            } else {
                steps.push('>');
            }
        }
    }
    steps.push('A');
}

fn find_min_path(code: &str) -> usize {
    let result = find_num_pad_path(code);
    let mut size = usize::MAX;
    for x in result {
        // println!("{}", x);
        let mut result = find_dir_pad_paths(x.as_str());
        result.sort_by(|a, b| a.len().cmp(&b.len()));
        while result.last().unwrap().len() > result.first().unwrap().len() {
            result.remove(result.len() - 1);
        }
        // result.iter().for_each(|v| println!("{}", v));
        result.iter().for_each(|v| {
            let v = find_dir_pad_path(v.as_str());
            // println!("{}", v);
            if v.len() < size {
                size = v.len();
            }
        });
    }
    size
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_path() {
        assert_eq!(find_min_path("029A"), 68);
        assert_eq!(find_min_path("980A"), 60);
        assert_eq!(find_min_path("179A"), 68);
        assert_eq!(find_min_path("456A"), 64);
        assert_eq!(find_min_path("379A"), 64);
    }

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                029A
                980A
                179A
                456A
                379A
            ",
        );
        assert_eq!(result, "126384")
    }

    #[test]
    fn test_validate() {
        assert_eq!(execute_num("<A^A>^^AvvvA"), "029A");
        assert_eq!(execute_dir("v<<A>>^A<A>AvA<^AA>A<vAAA>^A"), "<A^A>^^AvvvA");
        assert_eq!(execute_dir("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"), "v<<A>>^A<A>AvA<^AA>A<vAAA>^A");
    }
}

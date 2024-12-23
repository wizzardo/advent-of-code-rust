use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

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

    pub fn get_rotations_count_from(&self, position: Vector, direction: Vector) -> usize {
        if self.x == position.x {
            if self.y < position.y {
                if direction.y == 0 {
                    1
                } else if direction.y < 0 {
                    0
                } else {
                    2
                }
            } else {
                if direction.y == 0 {
                    1
                } else if direction.y > 0 {
                    0
                } else {
                    2
                }
            }
        } else {
            if self.x < position.x {
                if direction.x == 0 {
                    1
                } else if direction.x < 0 {
                    0
                } else {
                    2
                }
            } else {
                if direction.x == 0 {
                    1
                } else if direction.x > 0 {
                    0
                } else {
                    2
                }
            }
        }
    }

    pub fn get_direction_from(&self, position: Vector) -> Vector {
        if self.x == position.x {
            if self.y < position.y {
                Vector { x: 0, y: -1 }
            } else {
                Vector { x: 0, y: 1 }
            }
        } else {
            if self.x < position.x {
                Vector { x: -1, y: 0 }
            } else {
                Vector { x: 1, y: 0 }
            }
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct NodeScore {
    score: usize,
    position: Vector,
    direction: Vector,
}

impl PartialOrd<Self> for NodeScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NodeScore {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

struct Path {
    nodes: Vec<Vector>,
}
impl Path {
    fn iter(&self) -> PathIter {
        PathIter { path: self, i: 0, current: None }
    }
}

struct PathIter<'a> {
    path: &'a Path,
    i: usize,
    current: Option<Vector>,
}

impl Iterator for PathIter<'_> {
    type Item = Vector;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_none() {
            self.current = self.path.nodes.get(self.i).map(|v| *v);
            if self.current.is_none() {
                return None;
            }
            return self.current;
        }

        let to = match self.path.nodes.get(self.i + 1) {
            None => { return None; }
            Some(to) => { to }
        };

        if let Some(c) = self.current.as_mut() {
            let d = to.get_direction_from(*c);
            c.x += d.x;
            c.y += d.y;

            if c == to {
                self.i += 1;
            }

            Some(*c)
        } else {
            panic!()
        }
    }
}

pub fn calculate1(input: &str) -> String {
    let mut data: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| line.chars().collect())
        .collect();

    let width = data[0].len();
    let height = data.len();

    if data[height - 2][1] != 'S' {
        panic!()
    }
    if data[1][width - 2] != 'E' {
        panic!()
    }
    data[height - 2][1] = '.';
    data[1][width - 2] = '.';

    // let data_ref = &data;
    let is_passable: Box<dyn Fn(Vector) -> bool> = Box::new(move |p: Vector| {
        p.x >= 0 && p.y >= 0 && p.x < width as i32 && p.y < height as i32 && data[p.y as usize][p.x as usize] == '.'
    });
    let count_connections: Box<dyn Fn(Vector, &Box<dyn Fn(Vector) -> bool>) -> usize> = Box::new(|p: Vector, is_passable: &Box<dyn Fn(Vector) -> bool>| {
        Vector::all_directions().iter().map(|it| Vector { x: p.x + it.x, y: p.y + it.y }).filter(|it| is_passable(*it)).count()
    });

    let mut nodes: HashMap<Vector, HashSet<Vector>> = HashMap::new();
    fn populate_nodes(
        position: Vector,
        nodes: &mut HashMap<Vector, HashSet<Vector>>,
        is_passable: &Box<dyn Fn(Vector) -> bool>,
        count_connections: &Box<dyn Fn(Vector, &Box<dyn Fn(Vector) -> bool>) -> usize>,
        mandatory_nodes: &[Vector],
    ) {
        for d in Vector::all_directions() {
            let mut p = position.clone();
            loop {
                p.x += d.x;
                p.y += d.y;
                if !is_passable(p) {
                    break;
                }
                if count_connections(p, is_passable) > 2 {
                    if nodes.entry(p).or_insert(HashSet::new()).insert(position) {
                        populate_nodes(p, nodes, is_passable, count_connections, mandatory_nodes);
                    }
                    break;
                }
                if count_connections(p, is_passable) == 2 && !is_passable(Vector { x: p.x + d.x, y: p.y + d.y }) {
                    if nodes.entry(p).or_insert(HashSet::new()).insert(position) {
                        populate_nodes(p, nodes, is_passable, count_connections, mandatory_nodes);
                    }
                    break;
                }
                if mandatory_nodes.contains(&p) {
                    if nodes.entry(p).or_insert(HashSet::new()).insert(position) {
                        populate_nodes(p, nodes, is_passable, count_connections, mandatory_nodes);
                    }
                    break;
                }
            }
        }
    }

    populate_nodes(
        Vector { x: 1, y: height as i32 - 2 },
        &mut nodes,
        &is_passable,
        &count_connections,
        &[
            Vector { x: 1, y: height as i32 - 2 },
            Vector { x: width as i32 - 2, y: 1 }
        ]
    );


    fn find_path(from: Vector, to: Vector, direction: Vector, nodes: &HashMap<Vector, HashSet<Vector>>) -> usize {
        let mut min: HashMap<Vector, (usize, Vector, Vector)> = HashMap::new();
        let mut queue: BinaryHeap<NodeScore> = BinaryHeap::new();

        min.insert(from, (0, from, direction));

        queue.push(NodeScore { score: 0, position: from, direction });
        let mut min_score: usize = usize::MAX;

        loop {
            let pop = queue.pop();
            if pop.is_none() {
                break;
            }

            let current = pop.unwrap();
            if current.score > min_score {
                continue;
            }

            let connections = match nodes.get(&current.position) {
                None => {
                    panic!()
                }
                Some(c) => {c}
            };
            for c in connections {
                let next_score = current.score + c.get_rotations_count_from(current.position, current.direction) * 1000 + (c.x - current.position.x).abs() as usize + (c.y - current.position.y).abs() as usize;
                if next_score > min_score {
                    continue
                }
                if *c == to && next_score < min_score {
                    min_score = next_score;
                }

                let next_direction = c.get_direction_from(current.position);

                let old = min.get(c);
                match old {
                    None => {
                        min.insert(*c, (next_score, current.position, next_direction));
                        queue.push(NodeScore { score: next_score, position: *c, direction: next_direction });
                    }
                    Some(old) => {
                        if old.0 > next_score {
                            min.insert(*c, (next_score, current.position, next_direction));
                            queue.push(NodeScore { score: next_score, position: *c, direction: next_direction });
                        }
                    }
                }
            }
        }

        min_score
    }

    let min_score = find_path(Vector { x: 1, y: height as i32 - 2 }, Vector { x: width as i32 - 2, y: 1 }, Vector { x: 1, y: 0 }, &nodes);

    let result = min_score;
    format!("{result}")
}

pub fn calculate2(input: &str) -> String {
    let mut data: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| line.chars().collect())
        .collect();

    let width = data[0].len();
    let height = data.len();

    if data[height - 2][1] != 'S' {
        panic!()
    }
    if data[1][width - 2] != 'E' {
        panic!()
    }
    data[height - 2][1] = '.';
    data[1][width - 2] = '.';

    let data_ref = data.clone();
    let is_passable: Box<dyn Fn(Vector) -> bool> = Box::new(move |p: Vector| {
        p.x >= 0 && p.y >= 0 && p.x < width as i32 && p.y < height as i32 && data_ref[p.y as usize][p.x as usize] == '.'
    });
    let count_connections: Box<dyn Fn(Vector, &Box<dyn Fn(Vector) -> bool>) -> usize> = Box::new(|p: Vector, is_passable: &Box<dyn Fn(Vector) -> bool>| {
        Vector::all_directions().iter().map(|it| Vector { x: p.x + it.x, y: p.y + it.y }).filter(|it| is_passable(*it)).count()
    });

    let mut nodes: HashMap<Vector, HashSet<Vector>> = HashMap::new();
    fn populate_nodes(
        position: Vector,
        nodes: &mut HashMap<Vector, HashSet<Vector>>,
        is_passable: &Box<dyn Fn(Vector) -> bool>,
        count_connections: &Box<dyn Fn(Vector, &Box<dyn Fn(Vector) -> bool>) -> usize>,
        mandatory_nodes: &[Vector],
    ) {
        for d in Vector::all_directions() {
            let mut p = position.clone();
            loop {
                p.x += d.x;
                p.y += d.y;
                if !is_passable(p) {
                    break;
                }
                if count_connections(p, is_passable) > 2 {
                    if nodes.entry(p).or_insert(HashSet::new()).insert(position) {
                        populate_nodes(p, nodes, is_passable, count_connections, mandatory_nodes);
                    }
                    break;
                }
                if count_connections(p, is_passable) == 2 && !is_passable(Vector { x: p.x + d.x, y: p.y + d.y }) {
                    if nodes.entry(p).or_insert(HashSet::new()).insert(position) {
                        populate_nodes(p, nodes, is_passable, count_connections, mandatory_nodes);
                    }
                    break;
                }
                if mandatory_nodes.contains(&p) {
                    if nodes.entry(p).or_insert(HashSet::new()).insert(position) {
                        populate_nodes(p, nodes, is_passable, count_connections, mandatory_nodes);
                    }
                    break;
                }
            }
        }
    }

    populate_nodes(
        Vector { x: 1, y: height as i32 - 2 },
        &mut nodes,
        &is_passable,
        &count_connections,
        &[
            Vector { x: 1, y: height as i32 - 2 },
            Vector { x: width as i32 - 2, y: 1 }
        ],
    );

    fn find_paths(from: Vector, to: Vector, direction: Vector, nodes: &HashMap<Vector, HashSet<Vector>>) -> Option<Vec<Path>> {
        let mut min: HashMap<(Vector, Vector), Vec<(usize, Vector, Vector)>> = HashMap::new();
        let mut queue: BinaryHeap<NodeScore> = BinaryHeap::new();

        min.insert((from, direction), vec![(0, from, direction)]);

        queue.push(NodeScore { score: 0, position: from, direction });
        let mut min_score: usize = usize::MAX;

        loop {
            let pop = queue.pop();
            if pop.is_none() {
                break;
            }

            let current = pop.unwrap();
            if current.score > min_score {
                continue;
            }

            let connections = match nodes.get(&current.position) {
                None => {
                    panic!()
                }
                Some(c) => {c}
            };
            for c in connections {
                let next_score = current.score + c.get_rotations_count_from(current.position, current.direction) * 1000 + (c.x - current.position.x).abs() as usize + (c.y - current.position.y).abs() as usize;
                if next_score > min_score {
                    continue
                }
                if *c == to && next_score < min_score {
                    min_score = next_score;
                }

                let next_direction = c.get_direction_from(current.position);

                let old = min.get_mut(&(*c, next_direction));
                match old {
                    None => {
                        min.insert((*c, next_direction), vec![(next_score, current.position, current.direction)]);
                        queue.push(NodeScore { score: next_score, position: *c, direction: next_direction });
                    }
                    Some(old) => {
                        if old[0].0 > next_score {
                            min.insert((*c, next_direction), vec![(next_score, current.position, current.direction)]);
                            queue.push(NodeScore { score: next_score, position: *c, direction: next_direction });
                        } else if old[0].0 == next_score {
                            if !old.contains(&(next_score, current.position, next_direction)) {
                                old.push((next_score, current.position, next_direction))
                            }
                        }
                    }
                }
            }
        }

        let mut paths: Vec<Vec<(Vector, Vector)>> = Vec::new();
        for d in Vector::all_directions() {
            if min.contains_key(&(to, d)) {
                paths.push(vec![(to, d)]);
            }
        }
        loop {
            if paths.iter().all(|path| path.last().unwrap().0 == from) {
                break
            }
            for i in 0..paths.len() {
                if paths[i].last().unwrap().0 == from {
                    continue
                }
                let position = paths[i].last().unwrap();
                let options = min.get(position).unwrap();
                if options.len() > 1 {
                    for j in 1..options.len() {
                        let mut another = paths[i].clone();
                        another.push((options[j].1, options[j].2));
                        paths.push(another);
                    }
                    paths[i].push((options[0].1,options[0].2));
                } else {
                    paths[i].push((options[0].1,options[0].2));
                }
            }
        }

        Some(paths.into_iter().map(|nodes| Path { nodes: nodes.into_iter().map(|(it, _)| it).collect() }).collect())
    }

    let paths = find_paths(Vector { x: 1, y: height as i32 - 2 }, Vector { x: width as i32 - 2, y: 1 }, Vector { x: 1, y: 0 }, &nodes);
    let paths = paths.unwrap();

    let mut unique_tiles = HashSet::new();
    for path in paths {
        // println!("{:?}", &path.nodes);
        for tile in path.iter() {
            unique_tiles.insert(tile);
        }
    }


    let result = unique_tiles.len();

    // for y in 0..height {
    //     for x in 0..width {
    //         if unique_tiles.contains(&Vector { x: x as i32, y: y as i32 }) {
    //             print!("O");
    //         } else {
    //             print!("{}", data[y][x]);
    //         }
    //     }
    //     println!("");
    // }
    // print!("");

    format!("{result}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                ###############
                #.......#....E#
                #.#.###.#.###.#
                #.....#.#...#.#
                #.###.#####.#.#
                #.#.#.......#.#
                #.#.#####.###.#
                #...........#.#
                ###.#.#####.#.#
                #...#.....#.#.#
                #.#.#.###.#.#.#
                #.....#...#.#.#
                #.###.#.#.#.#.#
                #S..#.....#...#
                ###############
            ",
        );
        assert_eq!(result, "7036")
    }

    #[test]
    fn test_1_2() {
        let result = calculate1(
            "
                #################
                #...#...#...#..E#
                #.#.#.#.#.#.#.#.#
                #.#.#.#...#...#.#
                #.#.#.#.###.#.#.#
                #...#.#.#.....#.#
                #.#.#.#.#.#####.#
                #.#...#.#.#.....#
                #.#.#####.#.###.#
                #.#.#.......#...#
                #.#.###.#####.###
                #.#.#...#.....#.#
                #.#.#.#####.###.#
                #.#.#.........#.#
                #.#.#.#########.#
                #S#.............#
                #################
            ",
        );
        assert_eq!(result, "11048")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                ###############
                #.......#....E#
                #.#.###.#.###.#
                #.....#.#...#.#
                #.###.#####.#.#
                #.#.#.......#.#
                #.#.#####.###.#
                #...........#.#
                ###.#.#####.#.#
                #...#.....#.#.#
                #.#.#.###.#.#.#
                #.....#...#.#.#
                #.###.#.#.#.#.#
                #S..#.....#...#
                ###############
            ",
        );
        assert_eq!(result, "45")
    }
}

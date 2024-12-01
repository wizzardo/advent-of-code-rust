use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::hash_map::{Entry, Keys};
use std::collections::{BinaryHeap, BTreeMap, HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::ops::Not;
use itertools::Itertools;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // self.x.hash(state);
        // self.y.hash(state);
        (self.x + self.y * 100).hash(state);
    }
}

impl HashCode for Point {
    fn hash(&self) -> usize {
        self.x + self.y * 100
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Char(char);

impl HashCode for Char {
    fn hash(&self) -> usize {
        self.0 as usize
    }
}

impl Point {
    fn distance(&self, other: &Point) -> u32 {
        (((self.x as i32) - (other.x as i32)).abs() + ((self.y as i32) - (other.y as i32)).abs()) as u32
    }

    fn neighbors(&self) -> impl Iterator<Item=Point> + '_ {
        let mut index = 0;
        std::iter::from_fn(move || {
            let offsets: [(i32, i32); 4] = [
                (-1, 0),
                (1, 0),
                (0, -1),
                (0, 1),
            ];

            loop {
                if index < offsets.len() {
                    let (dx, dy) = offsets[index];
                    index += 1;

                    if self.x == 0 && dx == -1 {
                        continue;
                    }
                    if self.y == 0 && dy == -1 {
                        continue;
                    }

                    return Some(Point {
                        x: ((self.x as i32) + dx) as usize,
                        y: ((self.y as i32) + dy) as usize,
                    });
                } else {
                    return None;
                }
            }
        })
    }
}

struct Graph {
    // by_point: HashMap<Point, Node>,
    by_point: SimpleHashMap<Point, Node>,
}

impl Graph {
    fn get(&self, key: &Point) -> Option<&Node> {
        self.by_point.get(key)
    }
}

struct Node {
    char: char,
    coordinates: Point,
    links: RefCell<Vec<NodeLink>>,
}

#[derive(Copy, Clone)]
struct NodeLink {
    from: Point,
    to: Point,
    char: char,
    distance: u32,
    door: Option<char>,
}

impl Graph {
    fn link<D, C>(&mut self, from: &Point, to: &Point, distance: u32, is_door: D, to_char: C) -> bool
        where D: Fn(&Point) -> Option<char>,
              C: Fn(&Point) -> char,
    {
        if !self.by_point.contains_key(from) {
            self.by_point.insert(from.clone(), Node {
                char: to_char(&from),
                coordinates: from.clone(),
                links: RefCell::new(Vec::new()),
            });
        }

        if !self.by_point.contains_key(to) {
            self.by_point.insert(to.clone(), Node {
                char: to_char(&to),
                coordinates: to.clone(),
                links: RefCell::new(Vec::new()),
            });
        }


        let from: &Node = self.by_point.get(from).unwrap();
        let to: &Node = self.by_point.get(to).unwrap();

        if from.links.borrow().iter().any(|link| link.to == to.coordinates) {
            return false;
        }

        if to.links.borrow().iter().any(|link| link.to == from.coordinates) {
            return false;
        }

        from.links.borrow_mut().push(NodeLink {
            from: from.coordinates,
            to: to.coordinates,
            char: to_char(&to.coordinates),
            distance,
            door: is_door(&to.coordinates),
        });
        to.links.borrow_mut().push(NodeLink {
            from: to.coordinates,
            to: from.coordinates,
            char: to_char(&from.coordinates),
            distance,
            door: is_door(&from.coordinates),
        });

        true
    }
}

pub trait HashCode {
    fn hash(&self) -> usize;
}

struct SimpleHashMap<K, V> {
    data: Vec<Option<Vec<(K, V)>>>,
    size: usize,
}

impl<K: PartialEq + HashCode, V> SimpleHashMap<K, V> {
    pub fn new(capacity: usize) -> Self {
        let mut data = Vec::with_capacity(capacity);
        data.resize_with(capacity, || None);
        Self { data: data, size: 0 }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    #[inline(never)]
    pub fn clear(&mut self) {
        self.size = 0;
        for x in self.data.iter_mut() {
            match x {
                None => {}
                Some(bucket) => {
                    bucket.clear()
                }
            }
        }
    }

    #[inline(never)]
    pub fn get(&self, key: &K) -> Option<&V> {
        let i = key.hash() % self.data.capacity();
        match &self.data[i] {
            None => { None }
            Some(bucket) => {
                Self::iterate_bucket(key, bucket)
            }
        }
    }

    pub fn keys(&self) -> impl Iterator<Item=&K> + '_ {
        let mut index = 0;
        let mut bucket_index = 0;
        let mut finds = 0;
        std::iter::from_fn(move || {
            if finds == self.size {
                return None;
            }

            while index < self.data.capacity() {
                match &self.data[index] {
                    None => { index += 1 }
                    Some(bucket) => {
                        match bucket.get(bucket_index) {
                            None => {
                                bucket_index = 0;
                                index += 1;
                            }
                            Some(kv) => {
                                bucket_index += 1;
                                finds += 1;
                                return Some(&kv.0);
                            }
                        }
                    }
                }
            }
            None
        })
    }

    pub fn iter(&self) -> impl Iterator<Item=&(K, V)> + '_ {
        let mut index = 0;
        let mut bucket_index = 0;
        let mut finds = 0;
        std::iter::from_fn(move || {
            if finds == self.size {
                return None;
            }

            while index < self.data.capacity() {
                match &self.data[index] {
                    None => { index += 1 }
                    Some(bucket) => {
                        match bucket.get(bucket_index) {
                            None => {
                                bucket_index = 0;
                                index += 1;
                            }
                            Some(kv) => {
                                bucket_index += 1;
                                finds += 1;
                                // return Some((&kv.0, &kv.1));
                                return Some(kv);
                            }
                        }
                    }
                }
            }
            None
        })
    }


    #[inline(never)]
    fn iterate_bucket<'a>(key: &K, bucket: &'a Vec<(K, V)>) -> Option<&'a V> {
        // if bucket.len() > 3 {
        //     println!()
        // }
        bucket.iter().find(|(k, v)| { key == k }).and_then(|(_, v)| { Some(v) })
        // for (k, v) in bucket {
        //     if key == k {
        //         return Some(v);
        //     }
        // }
        // None
    }

    #[inline(never)]
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let i = key.hash() % self.data.capacity();
        match &mut self.data[i] {
            None => {
                let mut bucket = vec![];
                bucket.push((key, value));
                self.data[i] = Some(bucket);
                self.size += 1;
                None
            }
            Some(bucket) => {
                let option = bucket.iter().find_position(|(k, v)| key == *k);
                if let Some((j, prev)) = option {
                    let (_, v) = bucket.remove(j);
                    bucket.push((key, value));
                    return Some(v);
                }
                bucket.push((key, value));
                self.size += 1;
                None
            }
        }
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
}

pub fn calculate1(input: &str) -> String {
    let map: Vec<Vec<char>> = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            line.chars().collect()
        })
        .collect();

    let mut keys: [bool; 26] = [false; 26];
    let mut doors: [bool; 26] = [false; 26];
    let mut coords: HashMap<char, Point> = HashMap::new();
    // coords.keys()
    let mut coords: SimpleHashMap<Char, Point> = SimpleHashMap::new(64);

    let mut current_x = 0;
    let mut current_y = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            match map[y][x] {
                '#' => {}
                '.' => {}
                '@' => {
                    current_x = x;
                    current_y = y;
                }
                c => {
                    // coords.insert(c, Point { x, y });
                    coords.insert(Char(c), Point { x, y });
                }
            }
        }
    }

    // let mut cache: HashMap<CacheKey2, Option<(u32, Vec<char>)>> = HashMap::new();
    // for (x) in cache.iter() {}
    // let mut cache: HashMap<CacheKey3, Option<(u32, Vec<char>)>> = HashMap::new();
    // let mut cache: SimpleHashMap<CacheKey3, Option<(u32, Vec<char>)>> = SimpleHashMap::new(65536);
    // let mut cache: HashMap<CacheKey3, Option<(u32, Vec<char>)>> = HashMap::with_capacity(65536);

    // let steps = collect_all_keys(keys.clone(), doors.clone(), Point { x: current_x, y: current_y }, &map, &coords, &mut cache);
    // return steps.unwrap().0.to_string();

    // let mut start_node = Node {
    //     char: '@',
    //     coordinates: Point { x: current_x, y: current_y },
    //     links: RefCell::new(vec![]),
    // };

    let mut graph: Graph = Graph {
        // by_point: HashMap::new(),
        // by_point: SimpleHashMap::new(1024),
        by_point: SimpleHashMap::new(53),
    };
    // graph.insert(start_node.coordinates, start_node);

    // let ref_cell = RefCell::new(graph);
    // create_graph(Point { x: current_x, y: current_y }, &mut graph, &map, &coords);
    // create_graph2(Point { x: current_x, y: current_y }, &mut graph, &map, &coords, true);
    create_graph3(Point { x: current_x, y: current_y }, &mut graph, &map, &coords);

    // for (p,n) in graph.by_point.iter() {
    //     println!("{} {}: {} -> {}", p.x, p.y, n.char, n.links.borrow().len());
    // }

    // let n = 32;
    let n = graph.by_point.len();
    let mut came_from: SimpleHashMap<Point, NodeLink> = SimpleHashMap::new(n);
    let mut g_score: SimpleHashMap<Point, u32> = SimpleHashMap::new(n);
    let mut f_score: SimpleHashMap<Point, u32> = SimpleHashMap::new(n);
    let mut path: Vec<NodeLink> = vec![];
    let mut open_set: Vec<Point> = vec![];
    let mut counter: Vec<u32> = vec![0];

    // let steps = collect_all_keys_graph(
    //     0,
    //     Point { x: current_x, y: current_y },
    //     &graph,
    //     &coords,
    //     &mut cache,
    //     &mut came_from,
    //     &mut g_score,
    //     &mut f_score,
    //     &mut path,
    //     &mut open_set,
    //     &mut counter,
    // );
    // println!("counter: {:?}", counter);
    // return steps.unwrap().0.to_string();

    // let reachable_nodes = search_for_keys_dijkstra(
    //     Point { x: current_x, y: current_y },
    //     &graph,
    //     0,
    //     0,
    // );
    // let reachable_nodes_keys: Vec<&Char> = reachable_nodes.keys().collect();
    let steps = collect_keys_with_bh(
        Point { x: current_x, y: current_y },
        &graph,
        coords.keys().filter(|c| { c.0.is_ascii_lowercase() }).count(),
    );

    // let steps = search_for_keys_dijkstra(
    //     Point { x: current_x, y: current_y },
    //     &graph,
    //     coords.keys().filter(|c| { c.0.is_ascii_lowercase() }).count(),
    // );
    // return steps.to_string();

    // println!("cache.len(): {}", cache.len());
    return steps.to_string();
}

#[derive(PartialEq, Eq)]
struct SearchState {
    position: Point,
    keys: u32,
    found_keys: usize,
    distance: u32,
    char: char,
}


impl PartialOrd<Self> for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
            .then(self.found_keys.cmp(&other.found_keys))
    }
}

#[inline(never)]
fn collect_keys_with_bh(start: Point, graph: &Graph, keys_count: usize) -> u32 {
    let mut queue: BinaryHeap<SearchState> = BinaryHeap::new();
    queue.push(SearchState {
        position: start,
        keys: 0,
        distance: 0,
        found_keys: 0,
        char: '@',
    });

    let mut queued: HashMap<(char, u32), (u32, usize)> = HashMap::new();

    let mut counter = 0;

    while queue.is_empty().not() {
        let state = queue.pop().unwrap();
        if state.found_keys == keys_count {
            return state.distance;
        }

        let reachable_nodes = search_for_keys_dijkstra(&state.position, graph, state.keys, state.distance);

        counter += 1;
        for (_, v) in reachable_nodes.iter() {
            let mut next_keys = state.keys;
            let mut found_keys = state.found_keys;

            if v.char.is_ascii_lowercase() {
                let key = ((v.char as u8) - ('a' as u8)) as usize;
                if (state.keys >> key) & 1 == 1 {
                    continue;
                }

                next_keys = next_keys | (1 << key);
                found_keys += 1;
            } else {
                continue;
            }

            if let Some(other) = queued.get(&(v.char, next_keys)) {
                if other.0 <= v.cost {
                    continue;
                }
            }

            queue.push(SearchState {
                position: v.point,
                keys: next_keys,
                distance: v.cost,
                found_keys: found_keys,
                char: v.char,
            });
            queued.insert((v.char, next_keys), (v.cost, found_keys));
        }
    }
    return u32::MAX;
}


#[derive(PartialEq, Eq)]
struct SearchStateMultiple {
    positions: Vec<Point>,
    keys: u32,
    found_keys: usize,
    distance: u32,
    char: char,
}


impl PartialOrd<Self> for SearchStateMultiple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchStateMultiple {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
            .then(self.found_keys.cmp(&other.found_keys))
    }
}


#[inline(never)]
fn collect_keys_with_bh_multiple_starts(starts: Vec<Point>, graph: &Graph, keys_count: usize) -> u32 {
    let mut queue: BinaryHeap<SearchStateMultiple> = BinaryHeap::new();
    queue.push(SearchStateMultiple {
        positions: starts.clone(),
        keys: 0,
        distance: 0,
        found_keys: 0,
        char: '@',
    });

    let mut queued: HashMap<(char, u32), (u32, usize)> = HashMap::new();

    let mut counter = 0;

    while queue.is_empty().not() {
        let state = queue.pop().unwrap();
        if state.found_keys == keys_count {
            return state.distance;
        }
        for i in 0..state.positions.len() {
            let position = state.positions[i];
            let reachable_nodes = search_for_keys_dijkstra(&position, graph, state.keys, state.distance);
            counter += 1;
            for (_, v) in reachable_nodes.iter() {
                let mut next_keys = state.keys;
                let mut found_keys = state.found_keys;

                if v.char.is_ascii_lowercase() {
                    let key = ((v.char as u8) - ('a' as u8)) as usize;
                    if (state.keys >> key) & 1 == 1 {
                        continue;
                    }

                    next_keys = next_keys | (1 << key);
                    found_keys += 1;
                } else {
                    continue;
                }

                if let Some(other) = queued.get(&(v.char, next_keys)) {
                    if other.0 <= v.cost {
                        continue;
                    }
                }

                let mut next_positions = state.positions.clone();
                next_positions[i] = v.point;

                queue.push(SearchStateMultiple {
                    positions: next_positions,
                    keys: next_keys,
                    distance: v.cost,
                    found_keys: found_keys,
                    char: v.char,
                });
                queued.insert((v.char, next_keys), (v.cost, found_keys));
            }
        }
    }
    return u32::MAX;
}

#[inline(never)]
fn create_graph(start: Point, graph: &mut Graph, map: &Vec<Vec<char>>, coords: &SimpleHashMap<Char, Point>) {
    let is_passable = |p: &Point| {
        match map[p.y][p.x] {
            '#' => { false }
            _ => { true }
        }
    };
    for x in start.neighbors() {
        if !is_passable(&x) {
            continue;
        }

        let (to, distance) = find_next_node(
            &x,
            &start,
            is_passable,
            |p| {
                match map[p.y][p.x] {
                    '#' => { false }
                    '.' => { false }
                    '@' => { true }
                    _ => { true }
                }
            },
        );

        // if next_node.0 == x {
        //     continue;
        // }

        // let door = match map[to.y][to.x] {
        //     '#' => { None }
        //     '.' => { None }
        //     '@' => { None }
        //     c => {
        //         match c.is_ascii_uppercase() {
        //             true => { Some(c) }
        //             false => { None }
        //         }
        //     }
        // };

        // let distance = if to == x {
        //     distance
        // } else {
        //     distance + 1
        // };

        let is_door = |p: &Point| {
            match map[p.y][p.x] {
                '#' => { None }
                '.' => { None }
                '@' => { None }
                c => {
                    match c.is_ascii_uppercase() {
                        true => { Some(c) }
                        false => { None }
                    }
                }
            }
        };
        if graph.link(&start, &to, distance, is_door, |p| map[p.y][p.x]) {
            create_graph(to, graph, map, coords);
        }
    }
}

#[inline(never)]
fn create_graph3(start: Point, graph: &mut Graph, map: &Vec<Vec<char>>, coords: &SimpleHashMap<Char, Point>) {
    for (char, steps, point) in reachable_nodes(start, map) {
        graph.link(
            &start,
            &point,
            steps as u32,
            |p| match map[p.y][p.x].is_ascii_uppercase() {
                true => { Some(map[p.y][p.x]) }
                false => { None }
            },
            |p| map[p.y][p.x],
        );
    }
    for x in coords.keys() {
        let from = coords.get(x).unwrap();
        for (char, steps, point) in reachable_nodes(*from, map) {
            graph.link(
                &from,
                &point,
                steps as u32,
                |p| match map[p.y][p.x].is_ascii_uppercase() {
                    true => { Some(map[p.y][p.x]) }
                    false => { None }
                },
                |p| map[p.y][p.x],
            );
        }
    }
}

fn reachable_nodes(start: Point, map: &Vec<Vec<char>>) -> Vec<(char, usize, Point)> {
    let mut reachable: Vec<(char, usize, Point)> = vec![];
    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<(Point, usize)> = VecDeque::new();

    queue.push_back((start, 0));

    while !queue.is_empty() {
        let (current, steps) = queue.pop_back().unwrap();
        visited.insert(current);

        for p in current.neighbors() {
            if visited.contains(&p) {
                continue;
            }

            match map[p.y][p.x] {
                '.' => {
                    queue.push_back((p, steps + 1))
                }
                '@' => {
                    queue.push_back((p, steps + 1))
                }
                '#' => {}
                c => {
                    reachable.push((c, steps + 1, p))
                }
            }
        }
    }

    reachable
}

#[inline(never)]
fn create_graph2(start: Point, graph: &mut Graph, map: &Vec<Vec<char>>, coords: &SimpleHashMap<Char, Point>, recursive: bool) {
    for x in coords.keys() {
        let goal = coords.get(x).unwrap();
        if start == *goal {
            continue;
        }
        if let Some(node) = graph.get(goal) {
            // if node.links.borrow().iter().any(|x| x.from == start){
            //     println!();
            //     continue
            // }
            if node.links.borrow().iter().any(|x| x.to == start) {
                // println!();
                continue;
            }
        }


        let path = a_star(
            &start,
            goal,
            |a, b| { a.distance(b) },
            |p: &Point| {
                match map[p.y][p.x] {
                    '.' => { true }
                    '@' => { true }
                    '#' => { false }
                    c => { c == x.0 }
                }
            },
        );
        if path.is_none() {
            continue;
        }

        let path = path.unwrap();

        let steps = path.len() - 1;
        let to = &path[steps];

        let is_door = |p: &Point| {
            match map[p.y][p.x] {
                '#' => { None }
                '.' => { None }
                '@' => { None }
                c => {
                    match c.is_ascii_uppercase() {
                        true => { Some(c) }
                        false => { None }
                    }
                }
            }
        };
        graph.link(&start, to, steps as u32, is_door, |p| map[p.y][p.x]);
    }
    if recursive {
        for x in coords.keys() {
            let point = coords.get(x).unwrap();
            create_graph2(*point, graph, map, coords, false);
        }
    }
}

fn find_next_node<P, E>(start: &Point, from: &Point, is_passable: P, is_node: E) -> (Point, u32)
    where
        P: Fn(&Point) -> bool,
        E: Fn(&Point) -> bool,
{
    let mut prev = *from;
    let mut next = *start;
    let mut current = *start;
    let mut distance: u32 = 0;

    loop {
        let mut count = 0;
        distance += 1;

        if is_node(&current) {
            return (current, distance);
        }

        for x in current.neighbors() {
            if x == prev || !is_passable(&x) {
                continue;
            }

            next = x;

            count += 1;
            if count > 1 {
                return (current, distance);
            }
        }
        if count == 0 {
            return (current, distance - 1);
        } else {
            prev = current;
            current = next;
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
struct CacheKey {
    position: Point,
    keys: [bool; 26],
}

#[derive(Eq, PartialEq, Hash)]
struct CacheKey2 {
    position: Point,
    keys: u32,
}

#[derive(Eq, PartialEq, Hash)]
struct CacheKey3 {
    x: usize,
    y: usize,
    keys: u32,
}

impl HashCode for CacheKey3 {
    fn hash(&self) -> usize {
        // let mut hash = self.x + self.y * 100;
        // hash = hash * 31 + (self.keys as usize);
        // hash
        let mut hash = self.keys as usize;
        hash = hash * 31 + self.x + self.y * 100;
        hash
    }
}

fn collect_all_keys(
    keys: [bool; 26],
    doors: [bool; 26],
    start: Point,
    map: &Vec<Vec<char>>,
    coords: &HashMap<char, Point>,
    cache: &mut HashMap<CacheKey, Option<(u32, Vec<char>)>>,
) -> Option<(u32, Vec<char>)> {
    let mut min_steps = u32::MAX;
    let mut have_all_keys = true;
    let mut min_path: Vec<char> = vec![];

    for i in 0..keys.len() {
        if keys[i] {
            continue;
        }

        let key = ((i as u8) + ('a' as u8)) as char;
        let option = coords.get(&(key));
        if option.is_none() {
            break;
        }
        have_all_keys = false;

        let dest = option.unwrap();

        let path = a_star(
            &start,
            &dest,
            |a, b| a.distance(b),
            |p| {
                match map[p.y][p.x] {
                    '#' => { false }
                    '.' => { true }
                    '@' => { true }
                    c => {
                        if c.is_ascii_lowercase() {
                            true
                        } else {
                            let key = ((c.to_ascii_lowercase() as u8) - ('a' as u8)) as usize;
                            keys[key]
                        }
                    }
                }
            },
        );

        if path.is_none() {
            continue;
        }

        let path = path.unwrap();
        let mut steps: u32 = (path.len() - 1) as u32;
        let mut next_keys = keys.clone();
        next_keys[i] = true;

        let cache_key = CacheKey {
            position: path[path.len() - 1],
            keys: next_keys.clone(),
        };
        let cached_value = cache.get(&cache_key);

        let mut o: &Option<(u32, Vec<char>)> = match cached_value {
            None => {
                let o = collect_all_keys(next_keys, doors, path[path.len() - 1], &map, &coords, cache);
                cache.insert(cache_key, o);
                cache.get(&CacheKey {
                    position: path[path.len() - 1],
                    keys: next_keys.clone(),
                }).unwrap()
            }
            Some(v) => { v }
        };
        match o {
            None => { continue; }
            Some(n) => {
                if steps + n.0 < min_steps {
                    min_steps = steps + n.0;
                    min_path = n.1.clone();
                    min_path.insert(0, key);

                    // if min_path.len() == 8 {
                    //     println!("{} {:?}", min_steps, min_path);
                    // }
                }
            }
        }
    }

    if have_all_keys {
        return Some((0, vec![]));
    }

    return match min_steps {
        u32::MAX => { None }
        n => { Some((n, min_path)) }
    };
}

fn collect_all_keys_graph(
    keys: u32,
    start: Point,
    graph: &Graph,
    // coords: &HashMap<char, Point>,
    coords: &SimpleHashMap<Char, Point>,
    // cache: &mut HashMap<CacheKey2, Option<(u32, Vec<char>)>>,
    // cache: &mut HashMap<CacheKey3, Option<(u32, Vec<char>)>>,
    cache: &mut SimpleHashMap<CacheKey3, Option<(u32, Vec<char>)>>,
    came_from: &mut SimpleHashMap<Point, NodeLink>,
    g_score: &mut SimpleHashMap<Point, u32>,
    f_score: &mut SimpleHashMap<Point, u32>,
    path: &mut Vec<NodeLink>,
    open_set: &mut Vec<Point>,
    counter: &mut Vec<u32>,
) -> Option<(u32, Vec<char>)> {
    let mut min_steps = u32::MAX;
    let mut have_all_keys = true;
    let mut min_path: Vec<char> = vec![];

    for i in 0..32 {
        if (keys >> i) & 1 == 1 {
            continue;
        }

        let key = ((i as u8) + ('a' as u8)) as char;
        // let option = coords.get(&(key));
        let option = coords.get(&Char(key));
        if option.is_none() {
            break;
        }
        have_all_keys = false;

        let dest = option.unwrap();
        let dest_node = graph.get(dest).unwrap();
        let start_node = graph.get(&start).unwrap();

        counter[0] += 1;
        a_star_graph_with_shared_caches(
            start_node,
            dest_node,
            |a, b| a.distance(&b),
            |p| {
                match p.door {
                    None => { true }
                    Some(door) => {
                        let key = ((door.to_ascii_lowercase() as u8) - ('a' as u8)) as usize;
                        (keys >> key) & 1 == 1
                    }
                }
            },
            &graph,
            came_from,
            g_score,
            f_score,
            path,
            open_set,
        );

        came_from.clear();
        g_score.clear();
        f_score.clear();

        if path.is_empty() {
            continue;
        }

        let mut steps: u32 = path.iter().map(|x| x.distance).sum();
        let mut next_keys = keys;
        next_keys = next_keys | (1 << i);

        // let cache_key = CacheKey2 {
        //     position: path[path.len() - 1].to,
        //     keys: next_keys,
        // };

        let point = path[path.len() - 1].to;
        let cache_key = CacheKey3 {
            x: point.x,
            y: point.y,
            keys: next_keys,
        };
        let cached_value = cache.get(&cache_key);

        // let o = collect_all_keys_graph(next_keys, path[path.len() - 1].to, graph, &coords, cache);

        path.clear();
        let mut o: &Option<(u32, Vec<char>)> = match cached_value {
            None => {
                let o = collect_all_keys_graph(next_keys, point, graph, &coords, cache, came_from, g_score, f_score, path, open_set, counter);
                cache.insert(cache_key, o);
                // cache.get(&CacheKey2 {
                //     position: path[path.len() - 1].to,
                //     keys: next_keys,
                // }).unwrap()
                cache.get(&CacheKey3 {
                    x: point.x,
                    y: point.y,
                    keys: next_keys,
                }).unwrap()
            }
            Some(v) => { v }
        };
        match o {
            None => { continue; }
            Some(n) => {
                if steps + n.0 < min_steps {
                    min_steps = steps + n.0;
                    min_path = n.1.clone();
                    min_path.insert(0, key);

                    // if min_path.len() == 8 {
                    //     println!("{} {:?}", min_steps, min_path);
                    // }
                }
            }
        }
    }

    if have_all_keys {
        return Some((0, vec![]));
    }

    return match min_steps {
        u32::MAX => { None }
        n => { Some((n, min_path)) }
    };
}

fn search_for_keys(start: Point, graph: &Graph, keys_count: usize) -> u32 {
    let mut keys: u32 = 0;
    let mut doors: u32 = 0;
    let mut found_keys = 0;
    let mut distance: u32 = 0;

    struct Cost {
        char: Char,
        cost: u32,
    }

    let mut distances: SimpleHashMap<Char, Cost> = SimpleHashMap::new(128);
    let mut current: &Node = graph.get(&start).unwrap();

    while found_keys != keys_count {
        let mut min_cost: u32 = u32::MAX;
        let mut min_node_index: usize = usize::MAX;

        for (i, link) in current.links.borrow().iter().enumerate() {
            if link.char.is_ascii_uppercase() {
                let door = ((link.char as u8) - ('A' as u8)) as usize;
                if (doors >> door) & 1 == 1 {
                    continue;
                }
                if (keys >> door) & 1 == 0 {
                    continue;
                }
            } else if link.char == '@' {
                continue;
            } else {
                let key = ((link.char as u8) - ('a' as u8)) as usize;
                if (keys >> key) & 1 == 1 {
                    continue;
                }
            }

            let char = Char(link.char);
            let cost = link.distance + distance;

            if cost < min_cost {
                min_cost = cost;
                min_node_index = i;
            }

            match distances.get(&char) {
                None => {
                    distances.insert(char, Cost {
                        char: Char(link.char),
                        cost: cost,
                    });
                }
                Some(c) => {
                    if c.cost > link.distance + distance {
                        distances.insert(char, Cost {
                            char: Char(link.char),
                            cost: cost,
                        });
                    } else {
                        if c.cost < min_cost {
                            min_cost = c.cost;
                            min_node_index = i;
                        }
                    }
                }
            }
        }

        let next = &current.links.borrow()[min_node_index];
        distance = min_cost;
        if next.char.is_ascii_lowercase() {
            let key = ((next.char as u8) - ('a' as u8)) as usize;
            keys = keys | (1 << key);
            found_keys += 1;
        } else {
            let door = ((next.char as u8) - ('A' as u8)) as usize;
            doors = doors | (1 << door);
        }
        current = graph.get(&next.to).unwrap();
    }

    distance
}

#[derive(PartialEq, Eq)]
struct Cost {
    char: char,
    from: char,
    cost: u32,
    point: Point,
}


impl PartialOrd<Self> for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}


#[inline(never)]
fn search_for_keys_dijkstra(start: &Point, graph: &Graph, keys: u32, starting_cost: u32) -> HashMap<Char, Cost> {
    let mut visited_keys: u32 = 0;
    let mut visited_doors: u32 = 0;

    // let mut distances: SimpleHashMap<Char, Cost> = SimpleHashMap::new(16);
    let mut distances: HashMap<Char, Cost> = HashMap::new();
    let mut current: &Node = graph.get(start).unwrap();
    let mut queue: BinaryHeap<Cost> = BinaryHeap::with_capacity(16);
    queue.push(Cost { char: current.char, from: '@', cost: starting_cost, point: *start });

    while !queue.is_empty() {
        let current_cost = queue.pop().unwrap();
        let distance: u32 = current_cost.cost;
        current = graph.get(&current_cost.point).unwrap();

        match current.char {
            '@' => {}
            c => {
                if c.is_ascii_lowercase() {
                    let key = ((c as u8) - ('a' as u8)) as usize;
                    if (visited_keys >> key) & 1 == 1 {
                        continue;
                    }
                    visited_keys = visited_keys | (1 << key);
                } else {
                    let key = ((c as u8) - ('A' as u8)) as usize;
                    visited_doors = visited_doors | (1 << key);
                }
            }
        }

        for (i, link) in current.links.borrow().iter().enumerate() {
            if link.char.is_ascii_uppercase() {
                let door = ((link.char as u8) - ('A' as u8)) as usize;
                if (visited_doors >> door) & 1 == 1 {
                    continue;
                }
                if (keys >> door) & 1 == 0 {
                    continue;
                }
            } else if link.char == '@' {
                continue;
            } else {
                let key = ((link.char as u8) - ('a' as u8)) as usize;
                if (visited_keys >> key) & 1 == 1 {
                    continue;
                }
                if (keys >> key) & 1 == 0 {
                    visited_keys = visited_keys | (1 << key);
                }
            }

            let char = Char(link.char);
            let cost = link.distance + distance;

            match distances.get(&char) {
                None => {
                    distances.insert(char, Cost {
                        from: current.char,
                        char: link.char,
                        cost: cost,
                        point: link.to,
                    });
                    queue.push(Cost {
                        from: current.char,
                        char: link.char,
                        cost: cost,
                        point: link.to,
                    });
                }
                Some(c) => {
                    if c.cost > cost {
                        let char_to_remove: char = c.char;
                        distances.insert(char, Cost {
                            from: current.char,
                            char: link.char,
                            cost: cost,
                            point: link.to,
                        });
                        queue.retain(|x| char_to_remove != link.char);
                        queue.push(Cost {
                            from: current.char,
                            char: link.char,
                            cost: cost,
                            point: link.to,
                        });
                    }
                }
            }
        }
    }

    distances
}

fn reconstruct_path(came_from: &HashMap<Point, Point>, current: &Point) -> Vec<Point> {
    let mut path: Vec<Point> = vec![];
    path.push(*current);

    let mut c = current;
    loop {
        match came_from.get(c) {
            None => { break; }
            Some(p) => {
                path.push(*p);
                c = p;
            }
        }
    }

    path.reverse();
    return path;
}

#[inline(never)]
fn a_star<H, P>(start: &Point, goal: &Point, h: H, is_passable: P) -> Option<Vec<Point>>
    where
        H: Fn(&Point, &Point) -> u32,
        P: Fn(&Point) -> bool,
{
    if start.distance(goal) == 1 {
        return Some(vec![*start, *goal]);
    }

    let mut open_set: Vec<Point> = vec![];
    open_set.push(*start);

    let mut came_from: HashMap<Point, Point> = HashMap::new();
    let mut g_score: HashMap<Point, u32> = HashMap::new();
    let mut f_score: HashMap<Point, u32> = HashMap::new();

    g_score.insert(*start, 0);
    g_score.insert(*start, h(start, goal));

    while !open_set.is_empty() {
        // let (i, point) = open_set.iter().enumerate().min_by(|(_, a), (_, b)| h(a, goal).cmp(&h(b, goal))).unwrap();
        let current = open_set.remove(open_set.len() - 1);
        if current.eq(goal) {
            return Some(reconstruct_path(&came_from, &current));
        }

        for neighbor in current.neighbors() {
            if !is_passable(&neighbor) {
                continue;
            }

            let tentative_g_score = g_score.get(&current).unwrap() + 1; // + d(current, neighbor), but d returns 1
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, tentative_g_score + h(&neighbor, goal));

                if !open_set.contains(&neighbor) {
                    open_set.push(neighbor);
                    open_set.sort_by(|a, b| h(b, &goal).cmp(&h(a, &goal)));
                }
            }
        };
    }

    None
}


fn reconstruct_path_graph(came_from: &HashMap<Point, NodeLink>, current: &Point) -> Vec<NodeLink> {
    let mut path: Vec<NodeLink> = vec![];
    // path.push(*current);

    let mut c = current;
    loop {
        match came_from.get(c) {
            None => { break; }
            Some(link) => {
                path.push(*link);
                c = &link.from;
            }
        }
    }

    path.reverse();
    return path;
}

#[inline(never)]
fn reconstruct_path_graph2(came_from: &SimpleHashMap<Point, NodeLink>, current: &Point, path: &mut Vec<NodeLink>) {
    let mut c = current;
    loop {
        match came_from.get(c) {
            None => { break; }
            Some(link) => {
                path.push(*link);
                c = &link.from;
            }
        }
    }

    path.reverse();
}

fn a_star_graph<H, P>(start: &Node, goal: &Node, h: H, is_passable: P, graph: &Graph) -> Option<Vec<NodeLink>>
    where
        H: Fn(&Point, &Point) -> u32,
        P: Fn(&NodeLink) -> bool,
{
    let n = 8;
    let mut came_from: SimpleHashMap<Point, NodeLink> = SimpleHashMap::new(n);
    let mut g_score: SimpleHashMap<Point, u32> = SimpleHashMap::new(n);
    let mut f_score: SimpleHashMap<Point, u32> = SimpleHashMap::new(n);
    let mut result: Vec<NodeLink> = vec![];
    let mut open_set: Vec<Point> = vec![];
    a_star_graph_with_shared_caches(start, goal, h, is_passable, graph, &mut came_from, &mut g_score, &mut f_score, &mut result, &mut open_set);
    match result.is_empty() {
        true => { None }
        false => { Some(result) }
    }
}

#[inline(never)]
fn a_star_graph_with_shared_caches<H, P>(
    start: &Node,
    goal: &Node,
    h: H,
    is_passable: P,
    graph: &Graph,
    came_from: &mut SimpleHashMap<Point, NodeLink>,
    g_score: &mut SimpleHashMap<Point, u32>,
    f_score: &mut SimpleHashMap<Point, u32>,
    collect_into: &mut Vec<NodeLink>,
    open_set: &mut Vec<Point>,
)
    where
        H: Fn(&Point, &Point) -> u32,
        P: Fn(&NodeLink) -> bool,
{
    open_set.push(start.coordinates);

    // let mut came_from: HashMap<Point, NodeLink> = HashMap::new();
    // let mut g_score: HashMap<Point, u32> = HashMap::new();
    // let mut f_score: HashMap<Point, u32> = HashMap::new();
    // let n = 8;
    // let mut came_from: SimpleHashMap<Point, NodeLink> = SimpleHashMap::new(n);
    // let mut g_score: SimpleHashMap<Point, u32> = SimpleHashMap::new(n);
    // let mut f_score: SimpleHashMap<Point, u32> = SimpleHashMap::new(n);

    g_score.insert(start.coordinates, 0);
    g_score.insert(start.coordinates, h(&start.coordinates, &goal.coordinates));

    while !open_set.is_empty() {
        let current = open_set.remove(open_set.len() - 1);
        if current.eq(&goal.coordinates) {
            // return Some(reconstruct_path_graph(&came_from, &current));
            reconstruct_path_graph2(&came_from, &current, collect_into);
            break;
        }

        let node = graph.get(&current).unwrap();

        for neighbor in node.links.borrow().iter() {
            if !is_passable(neighbor) {
                continue;
            }

            // let tentative_g_score = g_score.get(&current).unwrap() + 1; // + d(current, neighbor), but d returns 1
            let tentative_g_score = g_score.get(&current).unwrap() + neighbor.distance;
            if tentative_g_score < *g_score.get(&neighbor.to).unwrap_or(&u32::MAX) {
                came_from.insert(neighbor.to, *neighbor);
                g_score.insert(neighbor.to, tentative_g_score);

                let distance = h(&neighbor.to, &goal.coordinates);
                f_score.insert(neighbor.to, tentative_g_score + distance);

                if !open_set.contains(&neighbor.to) {
                    let result = open_set.binary_search_by(|x| distance.cmp(&h(x, &goal.coordinates)));
                    match result {
                        Ok(i) => { open_set.insert(i, neighbor.to) }
                        Err(i) => { open_set.insert(i, neighbor.to) }
                    }

                    // open_set.push(neighbor.to);
                    // open_set.sort_by(|a, b| h(b, &goal.coordinates).cmp(&h(a, &goal.coordinates)));
                }
            }
        };
    }

    open_set.clear();
}

fn a_star_graph_with_shared_caches_t<H, P>(
    start: &Node,
    goal: &Node,
    h: H,
    is_passable: P,
    graph: &Graph,
    came_from: &mut SimpleHashMap<Point, NodeLink>,
    g_score: &mut SimpleHashMap<Point, u32>,
    f_score: &mut SimpleHashMap<Point, u32>,
    collect_into: &mut Vec<NodeLink>,
)
    where
        H: Fn(&Point, &Point) -> u32,
        P: Fn(&NodeLink) -> bool,
{
    let mut open_set: BTreeMap<u32, Vec<Point>> = BTreeMap::new();
    open_set.insert(0, vec![start.coordinates]);

    g_score.insert(start.coordinates, 0);
    g_score.insert(start.coordinates, h(&start.coordinates, &goal.coordinates));

    while !open_set.is_empty() {
        let mut entry = open_set.first_entry().unwrap();
        let l = entry.get_mut();
        let current = l[l.len() - 1];

        if current.eq(&goal.coordinates) {
            // return Some(reconstruct_path_graph(&came_from, &current));
            reconstruct_path_graph2(&came_from, &current, collect_into);
            return;
        }

        if l.len() == 1 {
            entry.remove();
        } else {
            l.remove(l.len() - 1);
        }

        let node = graph.get(&current).unwrap();

        for neighbor in node.links.borrow().iter() {
            if !is_passable(neighbor) {
                continue;
            }

            let tentative_g_score = g_score.get(&current).unwrap() + 1; // + d(current, neighbor), but d returns 1
            if tentative_g_score < *g_score.get(&neighbor.to).unwrap_or(&u32::MAX) {
                came_from.insert(neighbor.to, *neighbor);
                g_score.insert(neighbor.to, tentative_g_score);

                let f = tentative_g_score + h(&neighbor.to, &goal.coordinates);
                f_score.insert(neighbor.to, f);

                open_set.entry(f)
                    .and_modify(|list| { list.push(neighbor.to) })
                    .or_insert_with(|| vec![neighbor.to]);
            }
        };
    }
}

struct OpenSetEntry {
    score: u32,
    point: Point,
}

impl Eq for OpenSetEntry {}

impl PartialEq<Self> for OpenSetEntry {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl PartialOrd<Self> for OpenSetEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Some(self.score.cmp(&other.score))
        Some(other.score.cmp(&self.score))
    }
}

impl Ord for OpenSetEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // self.score.cmp(&other.score)
        other.score.cmp(&self.score)
    }
}

fn a_star_graph_with_shared_caches_pq<H, P>(
    start: &Node,
    goal: &Node,
    h: H,
    is_passable: P,
    graph: &Graph,
    came_from: &mut SimpleHashMap<Point, NodeLink>,
    g_score: &mut SimpleHashMap<Point, u32>,
    f_score: &mut SimpleHashMap<Point, u32>,
    collect_into: &mut Vec<NodeLink>,
)
    where
        H: Fn(&Point, &Point) -> u32,
        P: Fn(&NodeLink) -> bool,
{
    let mut open_set: BinaryHeap<OpenSetEntry> = BinaryHeap::new();
    open_set.push(OpenSetEntry { score: 0, point: start.coordinates });

    g_score.insert(start.coordinates, 0);
    g_score.insert(start.coordinates, h(&start.coordinates, &goal.coordinates));

    while !open_set.is_empty() {
        let current = open_set.pop().unwrap().point;

        if current.eq(&goal.coordinates) {
            // return Some(reconstruct_path_graph(&came_from, &current));
            reconstruct_path_graph2(&came_from, &current, collect_into);
            return;
        }

        let node = graph.get(&current).unwrap();

        for neighbor in node.links.borrow().iter() {
            if !is_passable(neighbor) {
                continue;
            }

            let tentative_g_score = g_score.get(&current).unwrap() + 1; // + d(current, neighbor), but d returns 1
            if tentative_g_score < *g_score.get(&neighbor.to).unwrap_or(&u32::MAX) {
                came_from.insert(neighbor.to, *neighbor);
                g_score.insert(neighbor.to, tentative_g_score);

                let f = tentative_g_score + h(&neighbor.to, &goal.coordinates);
                f_score.insert(neighbor.to, f);

                open_set.push(OpenSetEntry { score: f, point: neighbor.to });
            }
        };
    }
}


pub fn calculate2(input: &str) -> String {
    let mut map: Vec<Vec<char>> = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            line.chars().collect()
        })
        .collect();


    let mut starts: Vec<Point> = vec![];

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '@' {
                starts.push(Point { x, y })
            }
        }
    }
    if starts.len() == 1 {
        let center = "
            @#@
            ###
            @#@
        ";

        let replacement: Vec<&str> = center.lines()
            .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
            .filter(|it| it.len() != 0)
            .map(|line| {
                line.trim()
            })
            .collect();

        'outer: for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == '@' {
                    for i in 0..replacement.len() {
                        for j in 0..replacement.len() {
                            let char = replacement[i].chars().nth(j).unwrap();
                            map[y - replacement.len() / 2 + i][x - replacement.len() / 2 + j] = char
                        }
                    }
                    break 'outer;
                }
            }
        }

        // for y in 0..map.len() {
        //     for x in 0..map[y].len() {
        //         print!("{}", map[y][x]);
        //     }
        //     println!()
        // }

        starts.clear();

        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == '@' {
                    starts.push(Point { x, y })
                }
            }
        }
    }
    // println!("{:?}", starts);

    let mut coords: SimpleHashMap<Char, Point> = SimpleHashMap::new(64);

    let mut current_x = 0;
    let mut current_y = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            match map[y][x] {
                '#' => {}
                '.' => {}
                '@' => {
                    current_x = x;
                    current_y = y;
                }
                c => {
                    // coords.insert(c, Point { x, y });
                    coords.insert(Char(c), Point { x, y });
                }
            }
        }
    }


    let mut graph: Graph = Graph {
        by_point: SimpleHashMap::new(56),
    };

    for x in &starts {
        create_graph3(*x, &mut graph, &map, &coords);
    }

    let steps = collect_keys_with_bh_multiple_starts(
        starts,
        &graph,
        coords.keys().filter(|c| { c.0.is_ascii_lowercase() }).count(),
    );

    return steps.to_string();
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1_1() {
        let result = calculate1("
                ########################
                #f.D.E.e.C.b.A.@.a.B.c.#
                ######################.#
                #d.....................#
                ########################
            "
        );
        assert_eq!(result, "86")
    }

    #[test]
    fn test_1_2() {
        let result = calculate1("
                ########################
                #...............b.C.D.f#
                #.######################
                #.....@.a.B.c.d.A.e.F.g#
                ########################
            "
        );
        assert_eq!(result, "132")
    }

    #[test]
    fn test_1_3() {
        let result = calculate1("
                #################
                #i.G..c...e..H.p#
                ########.########
                #j.A..b...f..D.o#
                ########@########
                #k.E..a...g..B.n#
                ########.########
                #l.F..d...h..C.m#
                #################
            "
        );
        assert_eq!(result, "136")
    }

    #[test]
    fn test_1_4() {
        let result = calculate1("
                ########################
                #@..............ac.GI.b#
                ###d#e#f################
                ###A#B#C################
                ###g#h#i################
                ########################
            "
        );
        assert_eq!(result, "81")
    }


    #[test]
    fn test_2_1() {
        let result = calculate2("
                #######
                #a.#Cd#
                ##...##
                ##.@.##
                ##...##
                #cB#Ab#
                #######
            "
        );
        assert_eq!(result, "8")
    }

    #[test]
    fn test_2_2() {
        let result = calculate2("
                ###############
                #d.ABC.#.....a#
                ######@#@######
                ###############
                ######@#@######
                #b.....#.....c#
                ###############
            "
        );
        assert_eq!(result, "24")
    }

    #[test]
    fn test_2_3() {
        let result = calculate2("
                #############
                #DcBa.#.GhKl#
                #.###@#@#I###
                #e#d#####j#k#
                ###C#@#@###J#
                #fEbA.#.FgHi#
                #############
            "
        );
        assert_eq!(result, "32")
    }

    #[test]
    fn test_2_4() {
        let result = calculate2("
                #############
                #g#f.D#..h#l#
                #F###e#E###.#
                #dCba@#@BcIJ#
                #############
                #nK.L@#@G...#
                #M###N#H###.#
                #o#m..#i#jk.#
                #############
            "
        );
        assert_eq!(result, "72")
    }
}
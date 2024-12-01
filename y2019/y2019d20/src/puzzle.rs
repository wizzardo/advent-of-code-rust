use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use itertools::Itertools;
use crate::puzzle::PortalType::{INNER, OUTER};

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub(crate) fn neighbours(&self) -> [Point; 4] {
        [
            Point { x: self.x, y: self.y - 1 },
            Point { x: self.x + 1, y: self.y },
            Point { x: self.x, y: self.y + 1 },
            Point { x: self.x - 1, y: self.y },
        ]
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct PortalName(char, char);

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum PortalType {
    INNER,
    OUTER,
}

pub fn calculate1(input: &str) -> String {
    let mut map: Vec<&str> = input.lines()
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        // .enumerate()
        // .map(|(i, line)| {
        //     let vec: Vec<char> = line.chars().collect();
        //     vec
        // })
        .collect();

    let offset = 2;
    let width = map[offset].rfind('#').unwrap() + 1 - offset;
    let height = map.len() - 2 * offset;

    let mut donutWidth = usize::MAX;

    'outer: for y in 0..height {
        for x in 0..width / 2 {
            if map[y + offset][x + offset..x + 1 + offset].chars().nth(0).unwrap() == ' ' {
                donutWidth = x;
                break 'outer;
            }
        }
    }


    let mut portals: HashMap<PortalName, (PortalName, Vec<(Point, PortalType)>)> = HashMap::new();
    for y in offset..map.len() - offset {
        let borders = [0, width + offset];
        parse_h_portals(&map, offset, &mut portals, y, borders, width, height);
    }
    for y in offset + donutWidth..map.len() - offset - donutWidth {
        let borders = [offset + width - donutWidth - offset, offset + donutWidth];
        parse_h_portals(&map, offset, &mut portals, y, borders, width, height);
    }

    for x in offset..width + offset {
        let borders = [0, height + offset];
        parse_v_portals(&map, offset, &mut portals, x, borders, width, height);
    }
    for x in offset + donutWidth..width + offset - donutWidth {
        let borders = [offset + height - donutWidth - offset, offset + donutWidth];
        parse_v_portals(&map, offset, &mut portals, x, borders, width, height);
    }

    let mut portals_by_coordinates: HashMap<Point, PortalName> = HashMap::new();
    portals.values().for_each(|(name, points)| {
        points.iter().for_each(|(p, _)| {
            portals_by_coordinates.insert(*p, *name);
        })
    });

    let mut graph: HashMap<PortalName, Node> = HashMap::new();
    build_graph(&mut graph, &portals, &portals_by_coordinates, &map, width, height, offset);


    let result = find_path(PortalName('A', 'A'), PortalName('Z', 'Z'), &graph);

    return result.to_string();
}

#[derive(Debug)]
struct DijkstraCost {
    node: PortalName,
    cost: u32,
}

impl Eq for DijkstraCost {}

impl PartialEq<Self> for DijkstraCost {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd<Self> for DijkstraCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl Ord for DijkstraCost {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn find_path(from: PortalName, to: PortalName, graph: &HashMap<PortalName, Node>) -> u32 {
    let mut visited: HashSet<PortalName> = HashSet::new();
    let mut queue: BinaryHeap<DijkstraCost> = BinaryHeap::new();


    let mut current: PortalName = from;
    queue.push(DijkstraCost { cost: 0, node: current });

    while let Some(cost) = queue.pop() {
        if !visited.insert(cost.node) {
            continue;
        }

        if cost.node.eq(&to) {
            return cost.cost - 1;
        }

        let node = graph.get(&cost.node).unwrap();

        node.links.iter().for_each(|link| {
            queue.push(DijkstraCost { cost: cost.cost + 1 + link.distance, node: link.to });
        });
    }


    return u32::MAX;
}

#[derive(Debug)]
struct DijkstraCost2 {
    node: PortalName,
    cost: u32,
    level: u32,
}

impl Eq for DijkstraCost2 {}

impl PartialEq<Self> for DijkstraCost2 {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd<Self> for DijkstraCost2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl Ord for DijkstraCost2 {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn find_path2(from: PortalName, to: PortalName, graph: &HashMap<PortalName, Node>) -> u32 {
    let mut queue: BinaryHeap<DijkstraCost2> = BinaryHeap::new();
    let mut visited: HashSet<(PortalName, u32)> = HashSet::new();

    {
        let mut current: PortalName = from;
        queue.push(DijkstraCost2 { cost: 0, node: current, level: 0 });
    }

    while let Some(cost) = queue.pop() {
        if !visited.insert((cost.node, cost.level)) {
            continue;
        }

        if cost.node.eq(&to) && cost.level == 1 {
            return cost.cost - 1;
        }

        let node = graph.get(&cost.node).unwrap();

        if node.portal_type == OUTER && cost.level == 1 {
            continue;
        }

        node.links.iter().for_each(|link| {
            queue.push(DijkstraCost2 {
                cost: cost.cost + 1 + link.distance,
                node: link.to,
                level: if node.portal_type == INNER { cost.level + 1 } else { cost.level - 1 },
            });
        });
    }


    return u32::MAX;
}

fn build_graph(
    graph: &mut HashMap<PortalName, Node>,
    portals: &HashMap<PortalName, (PortalName, Vec<(Point, PortalType)>)>,
    portals_by_coordinates: &HashMap<Point, PortalName>,
    map: &Vec<&str>,
    width: usize,
    height: usize,
    offset: usize,
) {
    let mut queue: VecDeque<(Point, u32, PortalName)> = VecDeque::new();
    let mut visited: HashSet<(PortalName, Point)> = HashSet::new();

    {
        let (position, _) = *portals.get(&PortalName('A', 'A')).unwrap().1.first().unwrap();
        queue.push_back((position, 0, PortalName('A', 'A')));
    }

    while let Some((p, distance, from)) = queue.pop_front() {
        if !visited.insert((from, p)) {
            continue;
        }

        if let Some(portal) = portals_by_coordinates.get(&p) {
            if !portal.eq(&from) {
                let (_, points) = portals.get(&portal).unwrap();
                graph.entry(from)
                    .and_modify(|n| {
                        n.links.push(NodeLink { from: from, to: *portal, distance: distance })
                    })
                    .or_insert_with(|| {
                        let portal_type = points.iter()
                            .find(|(point, _)| {
                                point.eq(&p)
                            }).unwrap().1;
                        Node {
                            name: from,
                            links: vec![
                                NodeLink { from: from, to: *portal, distance: distance }
                            ],
                            portal_type: portal_type,
                        }
                    });

                points.iter()
                    .for_each(|(p, _)| {
                        queue.push_back((*p, 0, *portal));
                    })
            }
        }

        for p in p.neighbours() {
            if p.x < 0 || p.y < 0 || p.x > width as i32 || p.y > height as i32 {
                continue;
            }
            let row = map[offset + (p.y as usize)];
            let column = &row[(p.x as usize) + offset..(p.x as usize) + 1 + offset];
            let c: char = column.chars().nth(0).unwrap();
            if c != '.' {
                continue;
            }
            queue.push_back((p, distance + 1, from));
        }
    }
}


struct Node {
    name: PortalName,
    links: Vec<NodeLink>,
    portal_type: PortalType,
}

#[derive(Copy, Clone)]
struct NodeLink {
    from: PortalName,
    to: PortalName,
    distance: u32,
}


fn parse_v_portals(
    map: &Vec<&str>,
    offset: usize,
    portals: &mut HashMap<PortalName, (PortalName, Vec<(Point, PortalType)>)>,
    x: usize,
    borders: [usize; 2],
    width: usize,
    height: usize,
) {
    for y in borders {
        if x >= map[y].len() {
            continue;
        }
        let substr = &map[y][x..x + 1];
        match substr.chars().nth(0) {
            None => {}
            Some(c) => {
                match c {
                    ' ' => {}
                    c1 => {
                        let c2 = *(&map[y + 1][x..x + 1].chars().nth(0).unwrap());
                        if c2 == ' ' {
                            continue;
                        }
                        let portal = PortalName(c1, c2);
                        let entry = portals.entry(portal);
                        let y = (if y == borders[0] { y } else { y - offset - 1 }) as i32;
                        let x = (x - offset) as i32;
                        let portal_type: PortalType = if x == 0 || y == 0 || x as usize == width - 1 || y as usize == height - 1 { OUTER } else { INNER };
                        entry
                            .and_modify(|(k, points)| {
                                points.push((Point { x: x, y: y }, portal_type))
                            })
                            .or_insert((portal, vec![(Point { x: x, y: y }, portal_type)]));
                    }
                }
            }
        }
    }
}

fn parse_h_portals(
    map: &Vec<&str>,
    offset: usize,
    portals: &mut HashMap<PortalName, (PortalName, Vec<(Point, PortalType)>)>,
    y: usize,
    borders: [usize; 2],
    width: usize,
    height: usize,
) {
    for x in borders {
        if x >= map[y].len() {
            continue;
        }

        let substr = &map[y][x..x + offset];
        match substr.chars().nth(0) {
            None => {}
            Some(c) => {
                match c {
                    ' ' => {}
                    _ => {
                        let portal = PortalName(
                            substr.chars().nth(0).unwrap(),
                            substr.chars().nth(1).unwrap(),
                        );
                        let entry = portals.entry(portal);
                        let x = (if x == borders[0] { x } else { x - offset - 1 }) as i32;
                        let y = (y - offset) as i32;
                        let portal_type: PortalType = if x == 0 || y == 0 || x as usize == width - 1 || y as usize == height - 1 { OUTER } else { INNER };
                        entry
                            .and_modify(|(k, points)| {
                                points.push((Point { x: x as i32, y: y }, portal_type))
                            })
                            .or_insert((portal, vec![(Point { x: x as i32, y: y }, portal_type)]));
                    }
                }
            }
        }
    }
}


pub fn calculate2(input: &str) -> String {
    let mut map: Vec<&str> = input.lines()
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        // .enumerate()
        // .map(|(i, line)| {
        //     let vec: Vec<char> = line.chars().collect();
        //     vec
        // })
        .collect();

    let offset = 2;
    let width = map[offset].rfind('#').unwrap() + 1 - offset;
    let height = map.len() - 2 * offset;

    let mut donutWidth = usize::MAX;

    'outer: for y in 0..height {
        for x in 0..width / 2 {
            if map[y + offset][x + offset..x + 1 + offset].chars().nth(0).unwrap() == ' ' {
                donutWidth = x;
                break 'outer;
            }
        }
    }


    let mut portals: HashMap<PortalName, (PortalName, Vec<(Point, PortalType)>)> = HashMap::new();
    for y in offset..map.len() - offset {
        let borders = [0, width + offset];
        parse_h_portals(&map, offset, &mut portals, y, borders, width, height);
    }
    for y in offset + donutWidth..map.len() - offset - donutWidth {
        let borders = [offset + width - donutWidth - offset, offset + donutWidth];
        parse_h_portals(&map, offset, &mut portals, y, borders, width, height);
    }

    for x in offset..width + offset {
        let borders = [0, height + offset];
        parse_v_portals(&map, offset, &mut portals, x, borders, width, height);
    }
    for x in offset + donutWidth..width + offset - donutWidth {
        let borders = [offset + height - donutWidth - offset, offset + donutWidth];
        parse_v_portals(&map, offset, &mut portals, x, borders, width, height);
    }

    let mut portals_by_coordinates: HashMap<Point, PortalName> = HashMap::new();
    portals.values().for_each(|(name, points)| {
        points.iter().for_each(|(p, _)| {
            portals_by_coordinates.insert(*p, *name);
        })
    });

    let mut graph: HashMap<PortalName, Node> = HashMap::new();
    build_graph(&mut graph, &portals, &portals_by_coordinates, &map, width, height, offset);


    let result = find_path2(PortalName('A', 'A'), PortalName('Z', 'Z'), &graph);

    return result.to_string();
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1_1() {
        let result = calculate1("
         A
         A
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z
             Z
"
        );
        assert_eq!(result, "23")
    }

    #[test]
    fn test_1_2() {
        let result = calculate1("
                   A
                   A
  #################.#############
  #.#...#...................#.#.#
  #.#.#.###.###.###.#########.#.#
  #.#.#.......#...#.....#.#.#...#
  #.#########.###.#####.#.#.###.#
  #.............#.#.....#.......#
  ###.###########.###.#####.#.#.#
  #.....#        A   C    #.#.#.#
  #######        S   P    #####.#
  #.#...#                 #......VT
  #.#.#.#                 #.#####
  #...#.#               YN....#.#
  #.###.#                 #####.#
DI....#.#                 #.....#
  #####.#                 #.###.#
ZZ......#               QG....#..AS
  ###.###                 #######
JO..#.#.#                 #.....#
  #.#.#.#                 ###.#.#
  #...#..DI             BU....#..LF
  #####.#                 #.#####
YN......#               VT..#....QG
  #.###.#                 #.###.#
  #.#...#                 #.....#
  ###.###    J L     J    #.#.###
  #.....#    O F     P    #.#...#
  #.###.#####.#.#####.#####.###.#
  #...#.#.#...#.....#.....#.#...#
  #.#####.###.###.#.#.#########.#
  #...#.#.....#...#.#.#.#.....#.#
  #.###.#####.###.###.#.#.#######
  #.#.........#...#.............#
  #########.###.###.#############
           B   J   C
           U   P   P
"
        );
        assert_eq!(result, "58")
    }

    #[test]
    fn test_2_1() {
        let result = calculate2("
         A
         A
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z
             Z
"
        );
        assert_eq!(result, "26")
    }

    #[test]
    fn test_2_2() {
        let result = calculate2("
             Z L X W       C
             Z P Q B       K
  ###########.#.#.#.#######.###############
  #...#.......#.#.......#.#.......#.#.#...#
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###
  #.#...#.#.#...#.#.#...#...#...#.#.......#
  #.###.#######.###.###.#.###.###.#.#######
  #...#.......#.#...#...#.............#...#
  #.#########.#######.#.#######.#######.###
  #...#.#    F       R I       Z    #.#.#.#
  #.###.#    D       E C       H    #.#.#.#
  #.#...#                           #...#.#
  #.###.#                           #.###.#
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#
CJ......#                           #.....#
  #######                           #######
  #.#....CK                         #......IC
  #.###.#                           #.###.#
  #.....#                           #...#.#
  ###.###                           #.#.#.#
XF....#.#                         RF..#.#.#
  #####.#                           #######
  #......CJ                       NM..#...#
  ###.#.#                           #.###.#
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#
  #.....#        F   Q       P      #.#.#.#
  ###.###########.###.#######.#########.###
  #.....#...#.....#.......#...#.....#.#...#
  #####.#.###.#######.#######.###.###.#.#.#
  #.......#.......#.#.#.#.#...#...#...#.#.#
  #####.###.#####.#.#.#.#.###.###.#.###.###
  #.......#.....#.#...#...............#...#
  #############.#.#.###.###################
               A O F   N
               A A D   M
"
        );
        assert_eq!(result, "396")
    }
}
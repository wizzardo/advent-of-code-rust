use itertools::Itertools;

#[derive(Debug)]
struct LineSegment {
    from_x: i32,
    from_y: i32,
    to_x: i32,
    to_y: i32,
}


#[derive(PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl LineSegment {
    fn length(&self) -> u32 {
        ((self.from_x - self.to_x).abs() + (self.from_y - self.to_y).abs()) as u32
    }

    fn intersects(&self, other: &LineSegment) -> Option<Point> {
        if self.from_x == self.to_x && other.from_x == other.to_x {
            return None;
        }
        if self.from_x == self.to_x {
            let min = other.from_x.min(other.to_x);
            let max = other.from_x.max(other.to_x);
            if min < self.from_x && max > self.to_x {
                let min = self.from_y.min(self.to_y);
                let max = self.from_y.max(self.to_y);
                if min < other.from_y && max > other.to_y {
                    return Some(Point { x: self.to_x, y: other.to_y });
                }
            }
        }

        if other.from_x == other.to_x {
            let min = self.from_x.min(self.to_x);
            let max = self.from_x.max(self.to_x);
            if min < other.from_x && max > other.to_x {
                let min = other.from_y.min(other.to_y);
                let max = other.from_y.max(other.to_y);
                if min < self.from_y && max > self.to_y {
                    return Some(Point { x: other.to_x, y: self.to_y });
                }
            }
        }

        None
    }
}

impl Point {
    fn distance(&self, x: i32, y: i32) -> u32 {
        ((self.x - x).abs() + (self.y - y).abs()) as u32
    }
}

pub fn calculate1(input: &str) -> String {
    let vec: Vec<Vec<LineSegment>> = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let mut x = 0;
            let mut y = 0;
            line.split(",")
                .map(|it| {
                    let length = it[1..].parse::<i32>().unwrap();
                    let direction = it.chars().nth(0).unwrap();
                    // println!("{} {}", direction, length);

                    let segment = match direction {
                        'R' => { LineSegment { from_x: x, from_y: y, to_x: x + length, to_y: y } }
                        'L' => { LineSegment { from_x: x, from_y: y, to_x: x - length, to_y: y } }
                        'U' => { LineSegment { from_x: x, from_y: y, to_x: x, to_y: y + length } }
                        'D' => { LineSegment { from_x: x, from_y: y, to_x: x, to_y: y - length } }
                        d => panic!("unknown direction {}", d)
                    };
                    x = segment.to_x;
                    y = segment.to_y;
                    segment
                })
                .collect()
        })
        // .inspect(|line| { dbg!(line); })
        .collect();

    // println!("{:?}", vec.get(0).unwrap());
    // println!("{:?}", vec.get(1).unwrap());

    let a = vec.get(0).unwrap();
    let b = vec.get(1).unwrap();
    let intersections: Vec<i32> = b.iter()
        .filter_map(|it| {
            let vec = a.iter().filter_map(|other| it.intersects(other)).collect_vec();
            if vec.is_empty() {
                return None;
            } else {
                return Some(vec);
            }
        })
        .flatten()
        .map(|p| p.x.abs() + p.y.abs())
        .sorted_by(|a, b| {
            Ord::cmp(&a, &b)
        })
        .collect();

    // println!("{:?}", intersections);

    return intersections.get(0).unwrap().to_string();
}

pub fn calculate2(input: &str) -> String {
    let vec: Vec<Vec<LineSegment>> = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let mut x = 0;
            let mut y = 0;
            line.split(",")
                .map(|it| {
                    let length = it[1..].parse::<i32>().unwrap();
                    let direction = it.chars().nth(0).unwrap();
                    // println!("{} {}", direction, length);

                    let segment = match direction {
                        'R' => { LineSegment { from_x: x, from_y: y, to_x: x + length, to_y: y } }
                        'L' => { LineSegment { from_x: x, from_y: y, to_x: x - length, to_y: y } }
                        'U' => { LineSegment { from_x: x, from_y: y, to_x: x, to_y: y + length } }
                        'D' => { LineSegment { from_x: x, from_y: y, to_x: x, to_y: y - length } }
                        d => panic!("unknown direction {}", d)
                    };
                    x = segment.to_x;
                    y = segment.to_y;
                    segment
                })
                .collect()
        })
        // .inspect(|line| { dbg!(line); })
        .collect();

    // println!("{:?}", vec.get(0).unwrap());
    // println!("{:?}", vec.get(1).unwrap());

    let a = vec.get(0).unwrap();
    let b = vec.get(1).unwrap();
    let intersections: Vec<u32> = b.iter()
        .enumerate()
        .filter_map(|(bi, it)| {
            let vec: Vec<u32> = a.iter()
                .enumerate()
                .filter_map(|(ai, other)| {
                    let option = it.intersects(other);
                    if option.is_some() {
                        let point = option.unwrap();
                        return Some(
                            count_steps(a, ai)
                                + count_steps(b, bi)
                                + point.distance(it.from_x, it.from_y)
                                + point.distance(other.from_x, other.from_y,
                            ));
                    }
                    None
                })
                .collect();


            return if vec.is_empty() {
                None
            } else {
                Some(vec)
            };
        })
        .flatten()
        .sorted_by(|a, b| {
            Ord::cmp(&a, &b)
        })
        .collect();

    // println!("{:?}", intersections);

    return intersections.get(0).unwrap().to_string();
}

fn count_steps(vec: &Vec<LineSegment>, until: usize) -> u32 {
    let x = vec.as_slice();

    let mut steps = 0;

    for i in (0..until).rev() {
        let intersection = &x[0..i].iter()
            .enumerate()
            .find_map(|(i, it)| it.intersects(&x[i]).map(|it| (i, it)));

        if intersection.is_some() {
            println!("{:?}", intersection);
            panic!()
        } else {
            steps += x[i].length()
        }
    }

    // println!("{:?}", sub);

    steps
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1("
                R75,D30,R83,U83,L12,D49,R71,U7,L72
                U62,R66,U55,R34,D71,R55,D58,R83
            "
        );
        assert_eq!(result, "159")
    }

    #[test]
    fn test_1_2() {
        let result = calculate1("
                R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
                U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
            "
        );
        assert_eq!(result, "135")
    }

    #[test]
    fn test_2_1() {
        let result = calculate2("
                R75,D30,R83,U83,L12,D49,R71,U7,L72
                U62,R66,U55,R34,D71,R55,D58,R83
            "
        );
        assert_eq!(result, "610")
    }

    #[test]
    fn test_2_2() {
        let result = calculate2("
                R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
                U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
            "
        );
        assert_eq!(result, "410")
    }
    #[test]
    fn test_2_3() {
        let result = calculate2("
                R8,U5,L5,D3
                U7,R6,D4,L4
            "
        );
        assert_eq!(result, "30")
    }

    #[test]
    fn intersect_1() {
        let a = LineSegment { from_x: 0, from_y: 0, to_x: 5, to_y: 0 };
        let b = LineSegment { from_x: 2, from_y: 2, to_x: 2, to_y: -2 };
        let option = a.intersects(&b);
        assert_eq!(option.is_some(), true);
        assert_eq!(option.unwrap(), Point { x: 2, y: 0 });

        let option = b.intersects(&a);
        assert_eq!(option.is_some(), true);
        assert_eq!(option.unwrap(), Point { x: 2, y: 0 });
    }
}
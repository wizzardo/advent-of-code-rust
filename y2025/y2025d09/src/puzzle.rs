use std::collections::HashMap;

pub fn calculate1(input: &str) -> String {
    let coords: Vec<(u64, u64)> = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap())
        })
        .collect();
    let mut max = 0;
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let a = coords[i];
            let b = coords[j];
            let v = (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1);
            if v > max {
                max = v;
            }
        }
    }
    let result = max;
    format!("{result}")
}

pub fn calculate2(input: &str) -> String {
    let coords: Vec<(u64, u64)> = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap())
        })
        .collect();

    let mut verticals: HashMap<u64, Vec<(u64, u64)>> = HashMap::new();
    let mut horizontals: HashMap<u64, Vec<(u64, u64)>> = HashMap::new();
    let mut vertical_segments: Vec<((u64, u64), (u64, u64))> = vec![];
    let mut horizontal_segments: Vec<((u64, u64), (u64, u64))> = vec![];

    let mut width = 0;
    let mut height = 0;

    for i in 0..coords.len() {
        let a = coords[i];
        let b = coords[if i + 1 >= coords.len() { 0 } else { i + 1 }];

        if a.0 > width {
            width = a.0;
        }
        if a.1 > height {
            height = a.1;
        }

        if a.0 == b.0 {
            verticals.entry(a.0).or_insert_with(Vec::new).push((a.1.min(b.1), a.1.max(b.1)));
            vertical_segments.push(((a.0, a.1.min(b.1)), (b.0, a.1.max(b.1))));
        } else {
            horizontals.entry(a.1).or_insert_with(Vec::new).push((a.0.min(b.0), a.0.max(b.0)));
            horizontal_segments.push(((a.0.min(b.0), a.1), (a.0.max(b.0), a.1)));
        }
    }
    width += 2;
    height += 2;

    fn is_inside(
        x: u64,
        y: u64,
        width: u64,
        height: u64,
        horizontals: &HashMap<u64, Vec<(u64, u64)>>,
        verticals: &HashMap<u64, Vec<(u64, u64)>>,
    ) -> bool {
        if let Some(horizontals) = horizontals.get(&y) {
            if horizontals.iter().any(|range| x >= range.0 && x <= range.1) {
                return true;
            }
        }
        if let Some(verticals) = verticals.get(&x) {
            if verticals.iter().any(|range| y >= range.0 && y <= range.1) {
                return true;
            }
        }

        let mut count = 0;
        for i in x..width {
            if let Some(verticals) = verticals.get(&i) {
                if verticals.iter().any(|range| y >= range.0 && y < range.1) {
                    count += 1;
                }
            }
        }

        let result = count % 2 == 1;
        result
    }

    fn intersets(segment: ((u64, u64), (u64, u64)), segments: &Vec<((u64, u64), (u64, u64))>) -> bool {
        if segment.0.0 == segment.1.0 {
            let x = segment.0.0;
            let y = (segment.0.1, segment.1.1);
            segments.iter().any(|h| h.0.0 < x && x < h.1.0 && y.0 < h.0.1 && h.0.1 < y.1)
        } else {
            let y = segment.0.1;
            let x = (segment.0.0, segment.1.0);
            segments.iter().any(|v| v.0.1 < y && y < v.1.1 && x.0 < v.0.0 && v.1.0 < x.1)
        }
    }

    fn is_valid(
        a: (u64, u64),
        b: (u64, u64),
        width: u64,
        height: u64,
        horizontals: &HashMap<u64, Vec<(u64, u64)>>,
        verticals: &HashMap<u64, Vec<(u64, u64)>>,
        horizontal_segments: &Vec<((u64, u64), (u64, u64))>,
        vertical_segments: &Vec<((u64, u64), (u64, u64))>,
    ) -> bool {
        // println!("is_valid {:?} {:?}", a, b);
        let min = (a.0.min(b.0), a.1.min(b.1));
        let max = (a.0.max(b.0), a.1.max(b.1));

        if intersets(((min.0, min.1), (min.0, max.1)), horizontal_segments) {
            return false;
        }
        if intersets(((max.0, min.1), (max.0, max.1)), horizontal_segments) {
            return false;
        }
        if intersets(((min.0, min.1), (max.0, min.1)), vertical_segments) {
            return false;
        }
        if intersets(((min.0, max.1), (max.0, max.1)), vertical_segments) {
            return false;
        }

        if !is_inside(min.0, min.1, width, height, horizontals, verticals) {
            return false;
        }
        if !is_inside(max.0, max.1, width, height, horizontals, verticals) {
            return false;
        }
        if !is_inside(min.0, max.1, width, height, horizontals, verticals) {
            return false;
        }
        if !is_inside(max.0, min.1, width, height, horizontals, verticals) {
            return false;
        }
        true
    }

    // if true {
    //     println!(" <svg width=\"800\" height=\"800\" viewBox=\"0 0 {width} {height}\" xmlns=\"http://www.w3.org/2000/svg\"><path d=\"");
    //     for (i, (x, y)) in coords.iter().enumerate() {
    //         if i == 0 {
    //             println!("M {} {}", x, y);
    //         } else {
    //             println!("L {} {}", x, y);
    //         }
    //     }
    //     println!("Z");
    //     println!("\" fill=\"black\" stroke=\"black\" stroke-width=\"0.1\"></path></svg>");
    // // <circle cx=\"5398\" cy=\"67501\" r=\"1000\" fill=\"red\"></circle>
    // // <circle cx=\"94737\" cy=\"50273\" r=\"1000\" fill=\"red\"></circle>
    //     return "test".to_string();
    // }

    let mut max = 0;
    let mut max_rect = ((0, 0), (0, 0));
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let a = coords[i];
            let b = coords[j];
            let v = (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1);
            if v > max {
                println!("testing {}/{}   {v} {a:?} {b:?}", (i * coords.len()) + (j + 1), coords.len() * coords.len());
                if is_valid(a, b, width, height, &horizontals, &verticals, &horizontal_segments, &vertical_segments) {
                    max = v;
                    max_rect = (a, b);
                }
            }
        }
    }
    let result = max;
    println!("{result} at {max_rect:?}");
    format!("{result}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                7,1
                11,1
                11,7
                9,7
                9,5
                2,5
                2,3
                7,3
            ",
        );
        assert_eq!(result, "50")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                7,1
                11,1
                11,7
                9,7
                9,5
                2,5
                2,3
                7,3
            ",
        );
        assert_eq!(result, "24")
    }
}

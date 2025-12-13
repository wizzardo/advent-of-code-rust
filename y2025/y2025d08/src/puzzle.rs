use std::collections::HashSet;

pub fn calculate1(input: &str) -> String {
    let boxes: Vec<(u64, u64, u64)> = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let mut coords = line.split(',').map(|num| num.parse::<u64>().unwrap());
            (coords.next().unwrap(), coords.next().unwrap(), coords.next().unwrap())
        })
        .collect();

    let target_number_of_connections = if boxes.len() < 100 { 10 } else { 1000 };
    let mut circuits: Vec<Vec<(u64, u64, u64)>> = vec![];
    for (x, y, z) in boxes {
        circuits.push(vec![(x, y, z)]);
    }

    #[derive(Debug)]
    struct Pair {
        distance: u64,
        circuit_a: usize,
        circuit_index_a: usize,
        circuit_b: usize,
        circuit_index_b: usize,
    }

    let mut min_connections: HashSet<((u64, u64, u64), (u64, u64, u64))> = HashSet::new();

    for _ in 0..target_number_of_connections {
        let mut min: Option<Pair> = None;
        for i in 0..circuits.len() {
            for j in 0..circuits[i].len() {
                let a = &circuits[i][j];

                for ii in i..circuits.len() {
                    for jj in 0..circuits[ii].len() {
                        if i == ii && j == jj {
                            continue;
                        }

                        let b = &circuits[ii][jj];

                        let distance = a.0.abs_diff(b.0).pow(2) + a.1.abs_diff(b.1).pow(2) + a.2.abs_diff(b.2).pow(2);
                        match &min {
                            None => {
                                if i != ii || !min_connections.contains(&(*a, *b)) {
                                    min = Some(Pair { distance, circuit_a: i, circuit_index_a: j, circuit_b: ii, circuit_index_b: jj })
                                }
                                // min = Some(Pair { distance, circuit_a: i, circuit_index_a: j, circuit_b: ii, circuit_index_b: jj })
                            }
                            Some(p) => {
                                if distance < p.distance && (i != ii || !min_connections.contains(&(*a, *b))) {
                                    min = Some(Pair { distance, circuit_a: i, circuit_index_a: j, circuit_b: ii, circuit_index_b: jj })
                                }
                            }
                        }
                    }
                }
            }
        }

        if let Some(p) = min {
            // println!("min: {:?} {:?} {:?} {:?}", p.circuit_a, p.circuit_b, circuits[p.circuit_a][p.circuit_index_a], circuits[p.circuit_b][p.circuit_index_b]);
            min_connections.insert((circuits[p.circuit_a][p.circuit_index_a], circuits[p.circuit_b][p.circuit_index_b]));
            min_connections.insert((circuits[p.circuit_b][p.circuit_index_b], circuits[p.circuit_a][p.circuit_index_a]));
            if p.circuit_a == p.circuit_b {
                continue;
            }

            let mut b = circuits.swap_remove(p.circuit_b);
            circuits[p.circuit_a].append(&mut b)
        }
    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));

    // for x in &circuits {
    //     println!("{:?}", x);
    // }

    let result = circuits[0].len() * circuits[1].len() * circuits[2].len();

    format!("{result}")
}

pub fn calculate2(input: &str) -> String {
    let boxes: Vec<(u64, u64, u64)> = input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let mut coords = line.split(',').map(|num| num.parse::<u64>().unwrap());
            (coords.next().unwrap(), coords.next().unwrap(), coords.next().unwrap())
        })
        .collect();

    let mut circuits: Vec<Vec<(u64, u64, u64)>> = vec![];
    for (x, y, z) in boxes {
        circuits.push(vec![(x, y, z)]);
    }

    #[derive(Debug)]
    struct Pair {
        distance: u64,
        circuit_a: usize,
        circuit_index_a: usize,
        circuit_b: usize,
        circuit_index_b: usize,
    }

    let result;

    loop {
        let mut min: Option<Pair> = None;
        for i in 0..circuits.len() {
            for j in 0..circuits[i].len() {
                let a = &circuits[i][j];

                for ii in i + 1..circuits.len() {
                    for jj in 0..circuits[ii].len() {
                        if i == ii && j == jj {
                            continue;
                        }

                        let b = &circuits[ii][jj];

                        let distance = a.0.abs_diff(b.0).pow(2) + a.1.abs_diff(b.1).pow(2) + a.2.abs_diff(b.2).pow(2);
                        match &min {
                            None => {
                                min = Some(Pair { distance, circuit_a: i, circuit_index_a: j, circuit_b: ii, circuit_index_b: jj })
                            }
                            Some(p) => {
                                if distance < p.distance {
                                    min = Some(Pair { distance, circuit_a: i, circuit_index_a: j, circuit_b: ii, circuit_index_b: jj })
                                }
                            }
                        }
                    }
                }
            }
        }

        if let Some(p) = min {
            // println!("min: {:?} {:?} {:?} {:?}", p.circuit_a, p.circuit_b, circuits[p.circuit_a][p.circuit_index_a], circuits[p.circuit_b][p.circuit_index_b]);

            if circuits.len() == 2 {
                result = circuits[p.circuit_a][p.circuit_index_a].0 * circuits[p.circuit_b][p.circuit_index_b].0;
                break;
            }

            let mut b = circuits.swap_remove(p.circuit_b);
            circuits[p.circuit_a].append(&mut b)
        }
    }


    format!("{result}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                162,817,812
                57,618,57
                906,360,560
                592,479,940
                352,342,300
                466,668,158
                542,29,236
                431,825,988
                739,650,466
                52,470,668
                216,146,977
                819,987,18
                117,168,530
                805,96,715
                346,949,466
                970,615,88
                941,993,340
                862,61,35
                984,92,344
                425,690,689
            ",
        );
        assert_eq!(result, "40")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                162,817,812
                57,618,57
                906,360,560
                592,479,940
                352,342,300
                466,668,158
                542,29,236
                431,825,988
                739,650,466
                52,470,668
                216,146,977
                819,987,18
                117,168,530
                805,96,715
                346,949,466
                970,615,88
                941,993,340
                862,61,35
                984,92,344
                425,690,689
            ",
        );
        assert_eq!(result, "25272")
    }
}

use std::collections::{HashMap, HashSet};

pub fn calculate1(input: &str) -> String {
    let mut nodes: HashMap<&str, Vec<&str>> = HashMap::new();
    input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            let (l, r) = line.split_once('-').unwrap();
            nodes.entry(l).or_insert_with(Vec::new).push(r);
            nodes.entry(r).or_insert_with(Vec::new).push(l);
        });

    let groups: HashSet<[&str; 3]> = nodes.iter()
        .filter(|(k, _)| k.starts_with("t"))
        .map(|(name, connections)| {
            let mut groups = Vec::with_capacity(0);
            for i in 0..connections.len() {
                for j in 1..connections.len() {
                    let l = nodes.get(connections[i]).unwrap();
                    let r = nodes.get(connections[j]).unwrap();
                    if l.contains(&connections[j]) && r.contains(&connections[i]) {
                        let mut group = [name, connections[i], connections[j]];
                        group.sort();
                        groups.push(group);
                    }
                }
            }
            groups
        })
        .filter(|it| it.len() != 0)
        .flatten()
        // .inspect(|it| {println!("{:?}", it)})
        .collect();

    let result = groups.len();
    format!("{result}")
}

pub fn calculate2(input: &str) -> String {
    let mut nodes: HashMap<&str, Vec<&str>> = HashMap::new();
    input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            let (l, r) = line.split_once('-').unwrap();
            nodes.entry(l).or_insert_with(Vec::new).push(r);
            nodes.entry(r).or_insert_with(Vec::new).push(l);
        });

    let mut groups: Vec<Vec<&str>> = Vec::new();
    for (name, connections) in &nodes {
        for i in 0..connections.len() {
            let mut group = connections.clone();
            group.push(name);
            for j in 0..connections.len() {
                if i == j {
                    continue;
                }
                let x = connections[j];
                let other = nodes.get(x).unwrap();

                for i in (0..group.len()).rev() {
                    let b = group[i];
                    if b == x {
                        continue;
                    }
                    if !other.contains(&b) {
                        group.remove(i);
                    }
                }
            }
            groups.push(group);
        }
    }
    groups.sort_by(|a, b| b.len().cmp(&a.len()));
    groups[0].sort();

    let result = groups[0].join(",");
    format!("{result}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                kh-tc
                qp-kh
                de-cg
                ka-co
                yn-aq
                qp-ub
                cg-tb
                vc-aq
                tb-ka
                wh-tc
                yn-cg
                kh-ub
                ta-co
                de-co
                tc-td
                tb-wq
                wh-td
                ta-ka
                td-qp
                aq-cg
                wq-ub
                ub-vc
                de-ta
                wq-aq
                wq-vc
                wh-yn
                ka-de
                kh-ta
                co-tc
                wh-qp
                tb-vc
                td-yn
            ",
        );
        assert_eq!(result, "7")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                kh-tc
                qp-kh
                de-cg
                ka-co
                yn-aq
                qp-ub
                cg-tb
                vc-aq
                tb-ka
                wh-tc
                yn-cg
                kh-ub
                ta-co
                de-co
                tc-td
                tb-wq
                wh-td
                ta-ka
                td-qp
                aq-cg
                wq-ub
                ub-vc
                de-ta
                wq-aq
                wq-vc
                wh-yn
                ka-de
                kh-ta
                co-tc
                wh-qp
                tb-vc
                td-yn
            ",
        );
        assert_eq!(result, "co,de,ka,ta")
    }
}

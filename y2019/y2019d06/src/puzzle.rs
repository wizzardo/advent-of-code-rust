use std::collections::HashMap;

pub fn calculate1(input: &str) -> String {
    let mut orbits: HashMap<&str, &str> = HashMap::new();

    input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            let split: Vec<&str> = line.split(")").collect();
            orbits.insert(split[1], split[0]);
        });

    let sum: u32 = orbits.keys()
        .map(|key| {
            let mut depth = 0;
            let mut parent: &str = key;
            loop {
                match orbits.get(parent) {
                    None => { break; }
                    Some(p) => {
                        depth += 1;
                        parent = p;
                    }
                }
            }
            depth
        })
        .sum();


    return sum.to_string();
}


pub fn calculate2(input: &str) -> String {
    let mut orbits: HashMap<&str, &str> = HashMap::new();

    input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            let split: Vec<&str> = line.split(")").collect();
            orbits.insert(split[1], split[0]);
        });


    let mut my_orbits: HashMap<&str, u32> = HashMap::new();

    let mut parent: &str = "YOU";
    let mut depth: u32 = 0;
    loop {
        match orbits.get(parent) {
            None => { break; }
            Some(p) => {
                my_orbits.insert(p, depth);
                depth += 1;
                parent = p;

            }
        }
    }

    let mut parent: &str = "SAN";
    let mut depth: u32 = 0;
    loop {
        match orbits.get(parent) {
            None => { break; }
            Some(p) => {
                if let Some(d) = my_orbits.get(p) {
                    depth += d;
                    break
                }
                depth += 1;
                parent = p;
            }
        }
    }


    return depth.to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1("
                COM)B
                B)C
                C)D
                D)E
                E)F
                B)G
                G)H
                D)I
                E)J
                J)K
                K)L
            "
        );
        assert_eq!(result, "42")
    }

    #[test]
    fn test_2() {
        let result = calculate2("
                COM)B
                B)C
                C)D
                D)E
                E)F
                B)G
                G)H
                D)I
                E)J
                J)K
                K)L
                K)YOU
                I)SAN
            "
        );
        assert_eq!(result, "4")
    }
}
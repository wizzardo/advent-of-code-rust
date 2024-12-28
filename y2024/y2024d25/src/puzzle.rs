
pub fn calculate1(input: &str) -> String {
    let mut key: [u8; 5] = [0; 5];
    let mut lock: [u8; 5] = [0; 5];
    let mut is_lock = true;
    let mut locks:Vec<[u8; 5]> = Vec::new();
    let mut keys:Vec<[u8; 5]> = Vec::new();
    input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .enumerate()
        .for_each(|(i, line)| {
            let l = i % 7;
            if l == 0 {
                is_lock = line.eq("#####");
            } else if l < 6 {
                line.chars().enumerate().for_each(|(i, c)| {
                    if c == '#' {
                        if is_lock {
                            lock[i] += 1;
                        } else {
                            key[i] += 1;
                        }
                    }
                });
            } else {
                if is_lock {
                    locks.push(lock);
                    lock.fill(0);
                } else {
                    keys.push(key);
                    key.fill(0);
                }
            }
        });

    let mut result = 0;
    for i in 0..locks.len() {
        let lock = &locks[i];
        'outer: for j in 0..keys.len() {
            let key = &keys[j];
            for k in 0..5 {
                if lock[k] + key[k] > 5 {
                    continue 'outer;
                }
            }

            result += 1;
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
                #####
                .####
                .####
                .####
                .#.#.
                .#...
                .....

                #####
                ##.##
                .#.##
                ...##
                ...#.
                ...#.
                .....

                .....
                #....
                #....
                #...#
                #.#.#
                #.###
                #####

                .....
                .....
                #.#..
                ###..
                ###.#
                ###.#
                #####

                .....
                .....
                .....
                #....
                #.#..
                #.#.#
                #####
            ",
        );
        assert_eq!(result, "3")
    }
}

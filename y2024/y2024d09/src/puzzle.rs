pub fn calculate1(input: &str) -> String {
    let mut data: Vec<i32> = Vec::with_capacity(input.len() * 9);
    input
        .trim_ascii()
        .chars()
        .enumerate()
        .for_each(|(index, c)| {
            let size = (c as usize) - ('0' as usize);
            if index % 2 != 0 {
                for _ in 0..size {
                    data.push(-1);
                }
            } else {
                for _ in 0..size {
                    data.push((index as i32) / 2);
                }
            }
        });

    let mut l = 0;
    let mut r = data.len() - 1;
    // for i in 0..data.len() {
    //     let v = data[i];
    //     print!("{}", v);
    // }
    // println!("");

    loop {
        while data[r] == -1 {
            r -= 1;
        }
        while data[l] != -1 {
            l += 1;
        }
        if l >= r {
            break;
        }
        data.swap(l, r);
    }

    let mut result = 0;
    for i in 0..data.len() {
        let v = data[i];
        if v == -1 {
            break;
        }
        // print!("{}", v);
        result += i as i64 * v as i64;
    }
    // println!("");
    format!("{result}")
}

pub fn calculate2(input: &str) -> String {
    let mut data: Vec<i32> = Vec::with_capacity(input.len() * 9);
    input
        .trim_ascii()
        .chars()
        .enumerate()
        .for_each(|(index, c)| {
            let size = (c as usize) - ('0' as usize);
            if index % 2 != 0 {
                for _ in 0..size {
                    data.push(-1);
                }
            } else {
                for _ in 0..size {
                    data.push((index as i32) / 2);
                }
            }
        });

    let mut r = data.len() - 1;
    // for i in 0..data.len() {
    //     let v = data[i];
    //     if v == -1 {
    //         print!(".");
    //     } else {
    //         print!("{}", v);
    //     }
    // }
    // println!("");

    let mut l = 0;
    loop {
        while r > 0 && data[r] == -1 {
            r -= 1;
        }
        if r <= 0 {
            break;
        }

        let mut len = 0;
        let index = data[r];
        while r > 0 && data[r] == index {
            len += 1;
            r -= 1;
        }

        while data[l] != -1 {
            l += 1;
        }

        let mut i = l;
        'outer: while i < r {
            if data[i] == -1 {
                for j in 0..len {
                    if data[i + j] != -1 {
                        i += j;
                        continue 'outer;
                    }
                }
                for j in 0..len {
                    data.swap(i + j, r + 1 + j);
                }
                break
            } else {
                i += 1;
            }
        }
    }

    let mut result = 0;
    for i in 0..data.len() {
        let v = data[i];
        // if v == -1 {
        //     print!(".");
        // } else {
        //     print!("{}", v);
        // }
        if v == -1 {
            continue;
        }
        result += i as i64 * v as i64;
    }
    // println!("");
    format!("{result}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                2333133121414131402
            ",
        );
        assert_eq!(result, "1928")
    }

    #[test]
    fn test_2() {
        let result = calculate2(
            "
                2333133121414131402
            ",
        );
        assert_eq!(result, "2858")
    }
}

use itertools::Itertools;


pub fn calculate1(input: &str) -> String {
    let mut arr: Vec<i32> = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let vec: Vec<i32> = line.chars().map(|x| (x as i32) - ('0' as i32)).collect();
            vec
        }).last().unwrap();

    let mut next = arr.clone();
    let data = arr.as_mut_slice();

    let pattern: [i32; 4] = [0, 1, 0, -1];
    for _ in 0..100 {
        for j in 0..data.len() {
            let mut sum = 0;
            for i in 0..data.len() {
                let p = pattern[((i + 1) / (j + 1)) % pattern.len()];
                // print!("{}*{} + ", data[i], p);
                sum += data[i] * p;
            }
            // println!(" = {}", sum);
            next[j] = sum.abs() % 10;
        }

        data.copy_from_slice(next.as_slice());
        // println!("{:?}", data);
    }

    let result = data[0..8].iter().map(|it| it.to_string()).join("");
    return result;
}


pub fn calculate2(input: &str) -> String {
    let arr: Vec<u8> = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let vec: Vec<u8> = line.chars().map(|x| (x as u8) - ('0' as u8)).collect();
            vec
        }).last().unwrap();

    let mut real_data: Vec<u8> = Vec::with_capacity(arr.len() * 10000);
    real_data.resize(arr.len() * 10000, 0);

    let mut next = real_data.clone();
    let data = real_data.as_mut_slice();

    for i in 0..10000 {
        let to = &mut data[i * arr.len()..(i + 1) * arr.len()];
        to.copy_from_slice(arr.as_slice())
    }

    let mut offset: usize = 0;
    for i in 0..7 {
        offset *= 10;
        offset += data[i] as usize;
    }

    if data.len() - offset > offset {
        panic!()
    }

    for _ in 0..100 {
        let mut sum: i32 = data[offset..data.len()].iter().map(|x| *x as i32).sum();
        for j in offset..data.len() {
            next[j] = (sum % 10) as u8;
            sum -= data[j] as i32;
        }

        // data.copy_from_slice(next.as_slice());
        data[offset..].copy_from_slice(&next.as_slice()[offset..]);
    }

    let sub = &data[offset..offset + 8];
    let result = sub.iter().map(|it| it.to_string()).join("");
    return result;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1_1() {
        let result = calculate1("
                80871224585914546619083218645595
            "
        );
        assert_eq!(result, "24176176")
    }

    #[test]
    fn test_1_2() {
        let result = calculate1("
                19617804207202209144916044189917
            "
        );
        assert_eq!(result, "73745418")
    }

    #[test]
    fn test_1_3() {
        let result = calculate1("
                69317163492948606335995924319873
            "
        );
        assert_eq!(result, "52432133")
    }

    #[test]
    fn test_2_1() {
        let result = calculate2("
                03036732577212944063491565474664
            "
        );
        assert_eq!(result, "84462026")
    }
}
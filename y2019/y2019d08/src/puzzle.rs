pub fn calculate1(input: &str) -> String {
    let result = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let width = 25;
            let height = 6;
            let pixels_count = width * height;
            let layers_count = line.len() / pixels_count;

            let mut min_zeroes = pixels_count + 1;
            let mut result = 0;

            for l in 0..layers_count {
                let layer = &line[pixels_count * l..pixels_count * l + pixels_count];
                let zeroes = layer.chars().filter(|x| *x == '0').count();
                if zeroes < min_zeroes {
                    let ones = layer.chars().filter(|x| *x == '1').count();
                    let twos = layer.chars().filter(|x| *x == '2').count();
                    result = ones * twos;
                    min_zeroes = zeroes;
                }
            }

            result
        }).last().unwrap();


    return result.to_string();
}


pub fn calculate2(input: &str) -> String {
    let width = 25;
    let height = 6;
    let pixels_count = width * height;

    let result = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let layers_count = line.len() / pixels_count;

            let mut image = [3; 150];

            for l in 0..layers_count {
                let layer = &line[pixels_count * l..pixels_count * l + pixels_count];
                // layer.chars().enumerate()
                //     .for_each(|(i, x)| {
                //         if image[i] >= 2 {
                //             image[i] = (x as u32) - ('0' as u32);
                //         }
                //     });

                for (i, x) in layer.chars().enumerate() {
                    if image[i] >= 2 {
                        image[i] = (x as u32) - ('0' as u32);
                    }
                }
            }

            image
        }).last().unwrap();


    for i in 0..height {
        let line = &result[width * i..width * i + width];
        line.iter().for_each(|x| { print!("{}", if *x == 0 { ' ' } else { '1' }); });
        println!("");
    }

    return 0.to_string();
}

#[cfg(test)]
mod test {
    use super::*;

}
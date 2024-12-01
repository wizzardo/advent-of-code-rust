use std::collections::HashMap;

struct Reaction<'a> {
    output: u64,
    inputs: Vec<(&'a str, u64)>,
}

pub fn calculate1(input: &str) -> String {
    let mut reactions: HashMap<&str, Reaction> = HashMap::new();

    input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            let (inputs, output) = line.rsplit_once(" => ").unwrap();
            let (output_amount, output) = output.split_once(" ").unwrap();
            let inputs = inputs.split(", ").map(|it| {
                let (input_amount, input) = it.split_once(" ").unwrap();
                (input, input_amount.parse().unwrap())
            }).collect();

            reactions.insert(output, Reaction {
                output: output_amount.parse().unwrap(),
                inputs: inputs,
            });
        });

    let mut cache: HashMap<&str, u64> = HashMap::with_capacity(reactions.len());

    let ore_count = execute_reaction("FUEL", &reactions, &mut cache);

    return ore_count.to_string();
}


fn execute_reaction<'a>(output: &'a str, reactions: &HashMap<&str, Reaction<'a>>, cache: &mut HashMap<&'a str, u64>) -> u64 {
    if output == "ORE" {
        let was = cache.get(output).unwrap_or(&0);
        cache.insert(output, 1 + was);
        return 1;
    }

    let reaction = reactions.get(output).unwrap();
    let mut ore_count = 0;
    for input in reaction.inputs.iter() {
        let mut has_amount = cache.get(input.0).unwrap_or(&0);

        if has_amount >= &input.1 {
            cache.insert(input.0, has_amount - &input.1);
        } else {
            while has_amount < &input.1 {
                ore_count += execute_reaction(input.0, reactions, cache);
                has_amount = cache.get(input.0).unwrap();
            }
            cache.insert(input.0, has_amount - &input.1);
        }
    }
    let was = cache.get(output).unwrap_or(&0);
    cache.insert(output, reaction.output + was);

    ore_count
}

fn execute_reaction_with_limited_ore<'a>(output: &'a str, reactions: &HashMap<&str, Reaction<'a>>, cache: &mut HashMap<&'a str, u64>) -> u64 {
    if output == "ORE" {
        return 0;
    }

    let reaction = reactions.get(output).unwrap();
    let mut multiplier = 1;
    for input in reaction.inputs.iter() {
        let mut has_amount = *cache.get(input.0).unwrap_or(&0);

        if has_amount >= input.1 {
            cache.insert(input.0, has_amount - &input.1);
        } else {
            while has_amount < input.1 {
                let added = execute_reaction_with_limited_ore(input.0, reactions, cache);
                has_amount = *cache.get(input.0).unwrap();
                if added == 0 {
                    break;
                }
            }

            if has_amount < input.1 {
                multiplier = 0;
            } else {
                cache.insert(input.0, has_amount - &input.1);
            }
        }
    }
    let was = cache.get(output).unwrap_or(&0);
    cache.insert(output, reaction.output * multiplier + was);

    reaction.output * multiplier
}

fn count_reaction_output<'a>(output: &'a str, reactions: &HashMap<&str, Reaction<'a>>, cache: &HashMap<&'a str, u64>) -> u64 {
    let reaction = reactions.get(output).unwrap();
    let mut multiplier = u64::MAX;
    for input in reaction.inputs.iter() {
        let mut has_amount = *cache.get(input.0).unwrap_or(&0);
        if has_amount / &input.1 < multiplier {
            multiplier = has_amount / &input.1;
        }
    }
    reaction.output * multiplier
}


struct ReactionInt {
    output: u64,
    inputs: Vec<(usize, u64)>,
}

pub fn calculate2(input: &str) -> String {
    let mut reactions: Vec<ReactionInt> = vec![];
    let mut components: Vec<&str> = vec![];

    input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            let (inputs, output) = line.rsplit_once(" => ").unwrap();
            let (output_amount, output) = output.split_once(" ").unwrap();
            let inputs = inputs.split(", ").map(|it| {
                let (input_amount, input) = it.split_once(" ").unwrap();
                let index: usize = match components.iter().position(|&it| it == input) {
                    None => {
                        components.push(input);
                        components.len() - 1
                    }
                    Some(i) => { i }
                };
                (index, input_amount.parse().unwrap())
            }).collect();

            let index: usize = match components.iter().position(|&it| it == output) {
                None => {
                    components.push(output);
                    components.len() - 1
                }
                Some(i) => { i }
            };

            if reactions.len() <= index {
                reactions.resize_with(index + 1, || ReactionInt { output: 0, inputs: vec![] })
            }

            reactions[index] = ReactionInt {
                output: output_amount.parse().unwrap(),
                inputs: inputs,
            };
        });

    let fuel_index = components.iter().position(|&it| it == "FUEL").unwrap();
    let ore_index = components.iter().position(|&it| it == "ORE").unwrap();

    reactions[ore_index] = ReactionInt { output: 1, inputs: vec![] };

    let mut cache: Vec<u64> = Vec::with_capacity(components.len());
    cache.resize(components.len(), 0);
    let reactions_slice = reactions.as_slice();
    let cache_mut = cache.as_mut_slice();

    // let ore_for_fuel = execute_reaction_slices(fuel_index, ore_index, &reactions_slice, cache.as_mut_slice());

    // let cache_after_one_fuel = cache.clone();
    let ore_for_fuel = execute_reaction_slices_mult(fuel_index, ore_index, 1, &reactions_slice, cache.as_mut_slice());

    // let mut ore_counter = ore_for_fuel;
    // for _ in 1..100 {
    //     ore_counter+=execute_reaction_slices(fuel_index, ore_index, &reactions_slice, cache.as_mut_slice());
    // }

    // let mut cache: HashMap<&str, u64> = HashMap::with_capacity(reactions.len());
    // let ore_for_fuel = execute_reaction("FUEL", &reactions, &mut cache);
    let mut ore: u64 = 1000000000000;
    let mut ore_reserved: u64 = ore / 100 * 100;
    ore -= ore_reserved;
    let mut fuel = ore / ore_for_fuel + 1;
    let ore_left = (ore % ore_for_fuel) + ore_reserved;

    for i in 0..cache.len() {
        if i != fuel_index {
            cache[i] *= fuel;
        }
    }


    // cache[ore_index] += ore_left;

    // for (_, amount) in cache.iter_mut() {
    //     *amount *= fuel;
    // }
    //
    // let ore_in_cache = *cache.get("ORE").unwrap_or(&0);
    // cache.insert("ORE", ore_in_cache + ore_left);
    // loop {
    //     let added_fuel = execute_reaction_with_limited_ore("FUEL", &reactions, &mut cache);
    //     fuel += added_fuel;
    //     if added_fuel==0{
    //         break
    //     }
    // }

    // let additional_fuel = count_reaction_output("FUEL", &reactions, &cache);

    let mut ore_count: u64 = ore_for_fuel * fuel;
    // loop {
    //     let fuel = *cache.get("FUEL").unwrap_or(&0);
    //     ore_count += execute_reaction("FUEL", &reactions, &mut cache);
    //     if ore_count > 10000000 {
    //         return fuel.to_string()
    //     }
    //     if ore_count > 1000000000000 {
    //         return fuel.to_string()
    //     }
    // }

    let limit: u64 = 1000000000000;
    // let limit: u64 = 1000000000;
    let mut percent_counter = 0;
    // loop {
    //     let additional_ore = execute_reaction_slices(fuel_index, ore_index, &reactions_slice, cache.as_mut_slice());
    //     ore_count += additional_ore;
    //     // if ore_count > 1000000000 {
    //     //     return fuel.to_string();
    //     // }
    //
    //     // if cache.iter().enumerate().all(|(i, v)| *v == 0 || i == fuel_index || i == ore_index) {
    //     //     println!("cache is empty at {}", fuel + 1)
    //     // }
    //     // if cache.iter().enumerate().all(|(i, v)| *v == cache_after_one_fuel[i] || i == fuel_index || i == ore_index) {
    //     //     println!("cache is empty at {}", fuel + 1)
    //     // }
    //
    //     if ore_count > limit {
    //         // println!("{:?}", cache);
    //         return fuel.to_string();
    //     }
    //     // if ore_count > limit / 100 * percent_counter {
    //     //     println!("{}% ", percent_counter);
    //     //     percent_counter += 1;
    //     // }
    //     fuel += 1;
    // }
    loop {
        let step = 100000;
        while limit - ore_count > ore_for_fuel * step * 1 {
            let additional_ore = execute_reaction_slices_mult(fuel_index, ore_index, step, &reactions_slice, cache.as_mut_slice());
            ore_count += additional_ore;

            fuel += step;
        }

        let step = 1000;
        while limit - ore_count > ore_for_fuel * step * 1 {
            let additional_ore = execute_reaction_slices_mult(fuel_index, ore_index, step, &reactions_slice, cache.as_mut_slice());
            ore_count += additional_ore;

            fuel += step;
        }

        let additional_ore = execute_reaction_slices(fuel_index, ore_index, &reactions_slice, cache.as_mut_slice());
        ore_count += additional_ore;

        if ore_count > limit {
            // println!("{:?}", cache);
            return fuel.to_string();
        }
        fuel += 1;
    }

    return 0.to_string();
}

fn execute_reaction_slices<'a>(output: usize, ore_index: usize, reactions: &[ReactionInt], cache: &mut [u64]) -> u64 {
    // if output == "ORE" {
    //     let was = cache.get(output).unwrap_or(&0);
    //     cache.insert(output, 1 + was);
    //     return 1;
    // }
    if output == ore_index {
        cache[output] += 1;
        return 1;
    }

    let reaction = &reactions[output];
    let mut ore_count = 0;
    for input in reaction.inputs.iter() {
        let mut has_amount = cache[input.0];

        if has_amount >= input.1 {
            cache[input.0] = has_amount - input.1;
        } else {
            while has_amount < input.1 {
                if input.0 == ore_index {
                    let produced = input.1 - has_amount;
                    cache[input.0] += produced;
                    ore_count += produced;
                } else {
                    ore_count += execute_reaction_slices(input.0, ore_index, reactions, cache);
                }
                has_amount = cache[input.0];
            }
            cache[input.0] = has_amount - input.1;
        }
    }
    let was = cache[output];
    cache[output] = reaction.output + was;

    ore_count
}

fn execute_reaction_slices_mult<'a>(output: usize, ore_index: usize, amount: u64, reactions: &[ReactionInt], cache: &mut [u64]) -> u64 {
    // if output == "ORE" {
    //     let was = cache.get(output).unwrap_or(&0);
    //     cache.insert(output, 1 + was);
    //     return 1;
    // }
    if output == ore_index {
        cache[output] += amount;
        return amount;
    }

    let reaction = &reactions[output];
    let mut multiplier = amount / reaction.output;
    if amount % reaction.output != 0 {
        multiplier += 1;
    }

    let mut ore_count = 0;
    for input in reaction.inputs.iter() {
        let mut has_amount = cache[input.0];

        if has_amount >= input.1 * multiplier {
            cache[input.0] = has_amount - input.1 * multiplier;
        } else {
            if has_amount < input.1 * multiplier {
                ore_count += execute_reaction_slices_mult(input.0, ore_index, input.1 * multiplier, reactions, cache);
                has_amount = cache[input.0];
            }
            cache[input.0] = has_amount - input.1 * multiplier;
        }
    }
    let was = cache[output];
    cache[output] = reaction.output * multiplier + was;

    ore_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1_1() {
        let result = calculate1("
                10 ORE => 10 A
                1 ORE => 1 B
                7 A, 1 B => 1 C
                7 A, 1 C => 1 D
                7 A, 1 D => 1 E
                7 A, 1 E => 1 FUEL
            "
        );
        assert_eq!(result, "31")
    }

    #[test]
    fn test_1_2() {
        let result = calculate1("
                9 ORE => 2 A
                8 ORE => 3 B
                7 ORE => 5 C
                3 A, 4 B => 1 AB
                5 B, 7 C => 1 BC
                4 C, 1 A => 1 CA
                2 AB, 3 BC, 4 CA => 1 FUEL
            "
        );
        assert_eq!(result, "165")
    }

    #[test]
    fn test_1_3() {
        let result = calculate1("
                157 ORE => 5 NZVS
                165 ORE => 6 DCFZ
                44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
                12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
                179 ORE => 7 PSHF
                177 ORE => 5 HKGWZ
                7 DCFZ, 7 PSHF => 2 XJWVT
                165 ORE => 2 GPVTF
                3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
            "
        );
        assert_eq!(result, "13312")
    }

    #[test]
    fn test_1_4() {
        let result = calculate1("
                2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
                17 NVRVD, 3 JNWZP => 8 VPVL
                53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
                22 VJHF, 37 MNCFX => 5 FWMGM
                139 ORE => 4 NVRVD
                144 ORE => 7 JNWZP
                5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
                5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
                145 ORE => 6 MNCFX
                1 NVRVD => 8 CXFTF
                1 VJHF, 6 MNCFX => 4 RFSQX
                176 ORE => 6 VJHF
            "
        );
        assert_eq!(result, "180697")
    }

    #[test]
    fn test_1_5() {
        let result = calculate1("
                171 ORE => 8 CNZTR
                7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
                114 ORE => 4 BHXH
                14 VRPVC => 6 BMBT
                6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
                6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
                15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
                13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
                5 BMBT => 4 WPTQ
                189 ORE => 9 KTJDG
                1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
                12 VRPVC, 27 CNZTR => 2 XDBXC
                15 KTJDG, 12 BHXH => 5 XCVML
                3 BHXH, 2 VRPVC => 7 MZWV
                121 ORE => 7 VRPVC
                7 XCVML => 6 RJRHP
                5 BHXH, 4 VRPVC => 5 LTCX
            "
        );
        assert_eq!(result, "2210736")
    }

    #[test]
    fn test_2_1() {
        let result = calculate2("
                157 ORE => 5 NZVS
                165 ORE => 6 DCFZ
                44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
                12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
                179 ORE => 7 PSHF
                177 ORE => 5 HKGWZ
                7 DCFZ, 7 PSHF => 2 XJWVT
                165 ORE => 2 GPVTF
                3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
            "
        );
        assert_eq!(result, "82892753")
    }
}
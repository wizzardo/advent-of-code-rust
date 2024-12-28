use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
enum Gate {
    AND,
    OR,
    XOR,
}

pub fn calculate1(input: &str) -> String {
    let mut values: HashMap<&str, u8> = HashMap::new();
    let mut gates: Vec<(Gate, (&str, &str, &str))> = Vec::new();
    input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            if line[3..4].eq(":") {
                let (l, r) = line.split_once(": ").unwrap();
                values.insert(l, r.parse::<u8>().unwrap());
            } else {
                let parts = line.split(' ').collect::<Vec<&str>>();
                let gate = match parts[1] {
                    "AND" => { (Gate::AND, (parts[0], parts[2], parts[4])) }
                    "OR" => { (Gate::OR, (parts[0], parts[2], parts[4])) }
                    "XOR" => { (Gate::XOR, (parts[0], parts[2], parts[4])) }
                    _ => { panic!() }
                };
                gates.push(gate);
            }
        });

    process_gates(&mut values, &mut gates);

    let mut result: u64 = 0;
    for i in (0..64).rev() {
        result = result << 1;
        if let Some(v) = &values.get(format!("z{:02}", i).as_str()) {
            result += **v as u64;
        }
    }
    format!("{result}")
}

pub fn calculate2(input: &str) -> String {
    let mut values: HashMap<&str, u8> = HashMap::new();
    let mut gates: Vec<(Gate, (&str, &str, &str))> = Vec::new();
    // let swaps: HashMap<&str, &str> = HashMap::new();
    // swaps.insert("aaa", "eee");
    // swaps.insert("eee", "aaa");
    // swaps.insert("ooo", "z99");
    // swaps.insert("z99", "ooo");
    // swaps.insert("bbb", "ccc");
    // swaps.insert("ccc", "bbb");
    // swaps.insert("aoc", "z24");
    // swaps.insert("z24", "aoc");

    input
        .lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .for_each(|line| {
            if line[3..4].eq(":") {
                let (l, r) = line.split_once(": ").unwrap();
                values.insert(l, r.parse::<u8>().unwrap());
            } else {
                let parts = line.split(' ').collect::<Vec<&str>>();
                let gate = match parts[1] {
                    "AND" => { (Gate::AND, (parts[0], parts[2], parts[4])) }
                    "OR" => { (Gate::OR, (parts[0], parts[2], parts[4])) }
                    "XOR" => { (Gate::XOR, (parts[0], parts[2], parts[4])) }
                    _ => { panic!() }
                };
                gates.push(gate);
            }
        });

    for i in (0..64).rev() {
        if let Some(v) = values.get_mut(format!("x{:02}", i).as_str()) {
            *v=0;
        }
        if let Some(v) = values.get_mut(format!("y{:02}", i).as_str()) {
            *v=0;
        }
    }

    let mut gates_by_out: HashMap<&str, (Gate, (&str, &str, &str))> = HashMap::new();
    for g in &gates {
        gates_by_out.insert(g.1.2, g.clone());
    }

    fn print_dependency(key: &str, gates: &HashMap<&str, (Gate, (&str, &str, &str))>){
        if let Some(g) = gates.get(key) {
            println!("{key}: {:?}", g);
            print_dependency(g.1.0, gates);
            print_dependency(g.1.1, gates);
        }
    }

    fn collect_dependencies<'a>(key: &'a str, gates: &HashMap<&'a str, (Gate, (&'a str, &'a str, &'a str))>, into: &mut Vec<&'a str>) {
        if let Some(g) = gates.get(key) {
            // println!("{key}: {:?}", g);
            into.push(g.1.0);
            into.push(g.1.1);
            collect_dependencies(g.1.0, gates, into);
            collect_dependencies(g.1.1, gates, into);
        }
    }

    let mut fixes: HashMap<&str, &str> = HashMap::new();
    // fixes.insert("bss", "grr"); //16?
    // fixes.insert("grr", "bss");
    // fixes.insert("bss", "svk"); //16?
    // fixes.insert("svk", "bss");
    // fixes.insert("bss", "kcm"); //16?
    // fixes.insert("kcm", "bss");
    // fixes.insert("bss", "fkb"); //16
    // fixes.insert("fkb", "bss");
    fixes.insert("z16", "fkb"); //16
    fixes.insert("fkb", "z16");
    fixes.insert("rrn", "z37"); // 37
    fixes.insert("z37", "rrn");
    // fixes.insert("jrg", "z37"); // 37
    // fixes.insert("z37", "jrg");
    fixes.insert("nnr", "rqf"); // 21
    fixes.insert("rqf", "nnr");
    fixes.insert("rdn", "z31"); // 31?
    fixes.insert("z31", "rdn");

    // print_dependency("z14", &gates_by_out);
    // println!("");
    // print_dependency("z15", &gates_by_out);
    // println!("");
    // print_dependency("z16", &gates_by_out);
    let mut dependencies: Vec<&str> = Vec::new();
    dependencies.extend(gates_by_out.keys());
    // collect_dependencies("z16", &gates_by_out, &mut dependencies);
    // println!("{:?}", dependencies);
    // let mut dependencies_b = Vec::new();
    // collect_dependencies("z15", &gates_by_out, &mut dependencies_b);
    // println!("{:?}", dependencies_b);
    // {
    //     let mut values = values.clone();
    //     let mut gates = gates.clone();
    //     values.insert("x37", 1);
    //     process_gates(&mut values, &mut gates);
    //
    //     let mut result: u64 = 0;
    //     for i in (0..64).rev() {
    //         result = result << 1;
    //         if let Some(v) = values.get(format!("z{:02}", i).as_str()) {
    //             result += *v as u64;
    //         }
    //     }
    //     println!("{:#010b} {:#010b} {:#010b} {:#010b} {:#010b} {:#010b} {result} ", (result >> 40) & 255, (result >> 32) & 255, (result >> 24) & 255, (result >> 16) & 255, (result >> 8) & 255, result & 255);
    // }
    // for i in 0..dependencies.len() {
    //     for j in i..dependencies.len() {
    //     // for j in 0..dependencies_b.len() {
    //         let mut values = values.clone();
    //         let mut gates = gates.clone();
    //         // values.insert("x37", 1);
    //         values.insert("x15", 1);
    //         values.insert("x16", 1);
    //         values.insert("y15", 1);
    //         // values.insert("x21", 1);
    //         // values.insert("y21", 1);
    //         // values.insert("x30", 1);
    //         // values.insert("y30", 1);
    //         // values.insert("y37", 1);
    //         // values.insert("x15", 1);
    //         // values.insert("y15", 1);
    //
    //         // z16<>fkb 0b00000000 0b00000000 0b00000000 0b00000010 0b00000000 0b00000000 131072
    //         // z16<>rcc 0b00000000 0b00000000 0b00000000 0b00000010 0b00000000 0b00000000 131072
    //         // z16<>z17 0b00000000 0b00000000 0b00000000 0b00000010 0b00000000 0b00000000 131072
    //         // grr<>cjt 0b00000000 0b00000000 0b00000000 0b00000010 0b00000000 0b00000000 131072
    //         // fkb<>tnn 0b00000000 0b00000000 0b00000000 0b00000010 0b00000000 0b00000000 131072
    //         // rcc<>tnn 0b00000000 0b00000000 0b00000000 0b00000010 0b00000000 0b00000000 131072
    //         // tnn<>z17 0b00000000 0b00000000 0b00000000 0b00000010 0b00000000 0b00000000 131072
    //
    //
    //         // tnn<>z17 0b00000000 0b00000000 0b00000000 0b00000001 0b00000000 0b00000000 65536
    //         // tnn<>fkb 0b00000000 0b00000000 0b00000000 0b00000001 0b00000000 0b00000000 65536
    //         // svk<>bss 0b00000000 0b00000000 0b00000000 0b00000001 0b00000000 0b00000000 65536
    //         // z17<>bss 0b00000000 0b00000000 0b00000000 0b00000001 0b00000000 0b00000000 65536
    //         // z17<>z16 0b00000000 0b00000000 0b00000000 0b00000001 0b00000000 0b00000000 65536
    //         // kcm<>bss 0b00000000 0b00000000 0b00000000 0b00000001 0b00000000 0b00000000 65536
    //         // fkb<>z16 0b00000000 0b00000000 0b00000000 0b00000001 0b00000000 0b00000000 65536
    //         // bss<>fkb 0b00000000 0b00000000 0b00000000 0b00000001 0b00000000 0b00000000 65536
    //
    //         // svk<>bss 0b00000000 0b00000000 0b00000000 0b00000010 0b00000000 0b00000000 131072
    //         // bss<>cjt 0b00000000 0b00000000 0b00000000 0b00000010 0b00000000 0b00000000 131072
    //         // bss<>grr 0b00000000 0b00000000 0b00000000 0b00000010 0b00000000 0b00000000 131072
    //         // bss<>fkb 0b00000000 0b00000000 0b00000000 0b00000010 0b00000000 0b00000000 131072
    //         // bss<>z17 0b00000000 0b00000000 0b00000000 0b00000010 0b00000000 0b00000000 131072
    //         // bss<>rcc 0b00000000 0b00000000 0b00000000 0b00000010 0b00000000 0b00000000 131072
    //         // bss<>kcm 0b00000000 0b00000000 0b00000000 0b00000010 0b00000000 0b00000000 131072
    //         // fkb<>z16 0b00000000 0b00000000 0b00000000 0b00000010 0b00000000 0b00000000 131072
    //         // z16<>z17 0b00000000 0b00000000 0b00000000 0b00000010 0b00000000 0b00000000 131072
    //         // z16<>rcc 0b00000000 0b00000000 0b00000000 0b00000010 0b00000000 0b00000000 131072
    //
    //
    //         // z21<>jck 0b00000000 0b00000000 0b00000000 0b01000000 0b00000000 0b00000000 4194304
    //         // z21<>z22 0b00000000 0b00000000 0b00000000 0b01000000 0b00000000 0b00000000 4194304
    //         // z21<>jsd 0b00000000 0b00000000 0b00000000 0b01000000 0b00000000 0b00000000 4194304
    //         // z21<>sfw 0b00000000 0b00000000 0b00000000 0b01000000 0b00000000 0b00000000 4194304
    //         // z21<>nnr 0b00000000 0b00000000 0b00000000 0b01000000 0b00000000 0b00000000 4194304
    //         // jck<>rqf 0b00000000 0b00000000 0b00000000 0b01000000 0b00000000 0b00000000 4194304
    //         // rqf<>nnr 0b00000000 0b00000000 0b00000000 0b01000000 0b00000000 0b00000000 4194304
    //
    //         // z32<>z31 0b00000000 0b00000000 0b10000000 0b00000000 0b00000000 0b00000000 2147483648
    //         // z31<>vtb 0b00000000 0b00000000 0b10000000 0b00000000 0b00000000 0b00000000 2147483648
    //         // z31<>rdn 0b00000000 0b00000000 0b10000000 0b00000000 0b00000000 0b00000000 2147483648
    //
    //
    //         // jrg<>z37 0b00000000 0b00100000 0b00000000 0b00000000 0b00000000 0b00000000 137438953472
    //         // gcg<>z37 0b00000000 0b00100000 0b00000000 0b00000000 0b00000000 0b00000000 137438953472
    //         // z37<>rrn 0b00000000 0b00100000 0b00000000 0b00000000 0b00000000 0b00000000 137438953472
    //         // z37<>z38 0b00000000 0b00100000 0b00000000 0b00000000 0b00000000 0b00000000 137438953472
    //
    //         let mut swaps: HashMap<&str, &str> = HashMap::new();
    //         let a = dependencies[i];
    //         // let b = dependencies_b[j];
    //         let b = dependencies[j];
    //         swaps.insert(a, b);
    //         swaps.insert(b, a);
    //
    //         if !process_gates_with_swaps(&mut values, &mut gates, &swaps) {
    //             continue;
    //         }
    //
    //         let mut result: u64 = 0;
    //         for i in (0..64).rev() {
    //             result = result << 1;
    //             if let Some(v) = values.get(format!("z{:02}", i).as_str()) {
    //                 result += *v as u64;
    //             }
    //         }
    //
    //         if result == 131072 { // 16
    //             println!("{a}<>{b} {:#010b} {:#010b} {:#010b} {:#010b} {:#010b} {:#010b} {result} ", (result >> 40) & 255, (result >> 32) & 255, (result >> 24) & 255, (result >> 16) & 255, (result >> 8) & 255, result & 255);
    //         }
    //         // if result == 131072 { // 15
    //         //     println!("{a}<>{b} {:#010b} {:#010b} {:#010b} {:#010b} {:#010b} {:#010b} {result} ", (result >> 40) & 255, (result >> 32) & 255, (result >> 24) & 255, (result >> 16) & 255, (result >> 8) & 255, result & 255);
    //         // }
    //         // if result == 2097152*2 { // 21
    //         //     println!("{a}<>{b} {:#010b} {:#010b} {:#010b} {:#010b} {:#010b} {:#010b} {result} ", (result >> 40) & 255, (result >> 32) & 255, (result >> 24) & 255, (result >> 16) & 255, (result >> 8) & 255, result & 255);
    //         // }
    //         // if result == 1073741824*2 { // 30
    //         //     println!("{a}<>{b} {:#010b} {:#010b} {:#010b} {:#010b} {:#010b} {:#010b} {result} ", (result >> 40) & 255, (result >> 32) & 255, (result >> 24) & 255, (result >> 16) & 255, (result >> 8) & 255, result & 255);
    //         // }
    //         // if result == 137438953472 { // 37
    //         //     println!("{a}<>{b} {:#010b} {:#010b} {:#010b} {:#010b} {:#010b} {:#010b} {result} ", (result >> 40) & 255, (result >> 32) & 255, (result >> 24) & 255, (result >> 16) & 255, (result >> 8) & 255, result & 255);
    //         // }
    //         // if result != 4194304 { // 21
    //         //     println!("{a}<>{b} {:#010b} {:#010b} {:#010b} {:#010b} {:#010b} {:#010b} {result} ", (result >> 40) & 255, (result >> 32) & 255, (result >> 24) & 255, (result >> 16) & 255, (result >> 8) & 255, result & 255);
    //         // }
    //         // if result != 4294967296 { // 31
    //         //     println!("{a}<>{b} {:#010b} {:#010b} {:#010b} {:#010b} {:#010b} {:#010b} {result} ", (result >> 40) & 255, (result >> 32) & 255, (result >> 24) & 255, (result >> 16) & 255, (result >> 8) & 255, result & 255);
    //         // }
    //         // if result != 274877906944 { // 37
    //         //     println!("{a}<>{b} {:#010b} {:#010b} {:#010b} {:#010b} {:#010b} {:#010b} {result} ", (result >> 40) & 255, (result >> 32) & 255, (result >> 24) & 255, (result >> 16) & 255, (result >> 8) & 255, result & 255);
    //         // }
    //     }
    // }

    for i in 0..45 {
    // for i in 0..1 {
        let mut values= values.clone();
        let mut gates= gates.clone();

        // values.insert("x01",1); // 2 0b00000000 0b00000010
        // values.insert("x02",1); // 4 0b00000000 0b00000100
        // values.insert("x03",1); // 4 0b00000000 0b00000100
        // let key = format!("y{:02}", i);
        // values.insert(key.as_str(), 1);
        let key = format!("x{:02}", i);
        values.insert(key.as_str(), 1);
        // values.insert("x37", 1);
        // values.insert("x31", 1);
        // values.insert("x32", 1);
        // values.insert("x33", 1);
        // let key = format!("x{:02}", i+0);
        // values.insert(key.as_str(), 1);
        // let key = format!("x{:02}", i+1);
        // values.insert(key.as_str(), 1);
        // let key = format!("x{:02}", i+2);
        // values.insert(key.as_str(), 1);
        // let key = format!("x{:02}", i+3);
        // values.insert(key.as_str(), 1);
        let key = format!("y{:02}", i+0);
        values.insert(key.as_str(), 1);
        let key = format!("y{:02}", i+1);
        values.insert(key.as_str(), 1);
        // let key = format!("y{:02}", i+2);
        // values.insert(key.as_str(), 1);
        // let key = format!("y{:02}", i+3);
        // values.insert(key.as_str(), 1);
        // let key = format!("x{:02}", i*2);
        // values.insert(key.as_str(), 1);
        // let key = format!("y{:02}", i*2+1);
        // values.insert(key.as_str(), 1);

        // for j in 8..24 { //0b00000000 0b00000000 0b00000001 0b00100000 0b11111111 0b00000000 18939648
        //     if let Some(v) = values.get_mut(format!("x{:02}", j).as_str()) {
        //         *v=1;
        //     }
        // }

        // for j in 0..22 {
        //     // if j == 16 || j == 21 || j == 31 || j == 37 { //0b00001111 0b11011111 0b01111111 0b11011110 0b11111111 0b11111111 17452597444607
        //     //     continue
        //     // }
        //     if let Some(v) = values.get_mut(format!("x{:02}", j*2).as_str()) {
        //         *v=1;
        //     }
        //     // if let Some(v) = values.get_mut(format!("y{:02}", j*2+1).as_str()) {
        //     //     *v=1;
        //     // }
        // }

        process_gates_with_swaps(&mut values, &mut gates, &fixes);
        // process_gates(&mut values, &mut gates);

        let mut result: u64 = 0;
        for i in (0..64).rev() {
            result = result << 1;
            if let Some(v) = values.get(format!("z{:02}", i).as_str()) {
                result += *v as u64;
            }
        }
        // x
        // 16 broken -> shows values as 17
        // 21 broken -> shows values as 22
        // 31 broken -> shows values as 32
        // 37 broken -> shows values as 38
        // y
        // 16 broken -> shows values as 17
        // 21 broken -> shows values as 22
        // 31 broken -> shows values as 32
        // 37 broken -> shows values as 38
        // println!("{:#010b} {:#010b} {:#010b} {:#010b} {:#010b} {:#010b} {result} ", (result >> 40) & 255, (result >> 32) & 255, (result >> 24) & 255, (result >> 16) & 255, (result >> 8) & 255, result & 255);
        // println!("{key} {:#010b} {:#010b} {:#010b} {:#010b} {:#010b} {:#010b} {result} ", (result >> 40) & 255, (result >> 32) & 255, (result >> 24) & 255, (result >> 16) & 255, (result >> 8) & 255, result & 255);
        println!("{key} {:#048b} {result} ", result);
        // println!("{:#048b} {result} ", result);
        // println!("{:#010b} {:#010b} {:#010b} {:#010b} {:#010b} {:#010b} {result} ", (result >> 40) & 255, (result >> 32) & 255, (result >> 24) & 255, (result >> 16) & 255, (result >> 8) & 255, result & 255);

        // y13 0b00000000 0b00000000 0b00000000 0b00000000 0b00110000 0b00000000 12288
        // y15 0b00000000 0b00000000 0b00000000 0b00000000 0b11000000 0b00000000 49152
        //?y17 0b00000000 0b00000000 0b00000000 0b00000100 0b00000000 0b00000000 262144
        // y19 0b00000000 0b00000000 0b00000000 0b00001100 0b00000000 0b00000000 786432
        //?y21 0b00000000 0b00000000 0b00000000 0b01010000 0b00000000 0b00000000 5242880
        // y23 0b00000000 0b00000000 0b00000000 0b11000000 0b00000000 0b00000000 12582912
        // y29 0b00000000 0b00000000 0b00110000 0b00000000 0b00000000 0b00000000 805306368
        //?y31 0b00000000 0b00000001 0b01000000 0b00000000 0b00000000 0b00000000 5368709120
        // y33 0b00000000 0b00000011 0b00000000 0b00000000 0b00000000 0b00000000 12884901888
        // y35 0b00000000 0b00001100 0b00000000 0b00000000 0b00000000 0b00000000 51539607552
        //?y37 0b00000000 0b01010000 0b00000000 0b00000000 0b00000000 0b00000000 343597383680
        // y39 0b00000000 0b11000000 0b00000000 0b00000000 0b00000000 0b00000000 824633720832
    }
    let mut result: Vec<&str> = Vec::new();
    result.extend(fixes.keys());
    result.sort();
    let result = result.join(",");
    format!("{result}")
}

fn process_gates<'a>(values: &mut HashMap<&'a str, u8>, gates: &'a mut Vec<(Gate, (&str, &str, &str))>) {
    while !gates.is_empty() {
        for i in (0..gates.len()).rev() {
            let (gate, args) = gates[i];
            let (l, r, out) = args;

            let l = values.get(l);
            if l.is_none() {
                continue
            }
            let r = values.get(r);
            if r.is_none() {
                continue
            }
            let l = *l.unwrap();
            let r = *r.unwrap();

            match gate {
                Gate::AND => {
                    values.insert(out, l & r);
                }
                Gate::OR => {
                    values.insert(out, l | r);
                }
                Gate::XOR => {
                    values.insert(out, l ^ r);
                }
            }
            gates.remove(i);
        }
    }
}

fn process_gates_with_swaps<'a>(values: &mut HashMap<&'a str, u8>, gates: &'a mut Vec<(Gate, (&str, &str, &str))>, swaps: &HashMap<&'a str, &'a str>) -> bool {
    while !gates.is_empty() {
        let mut modified = false;
        for i in (0..gates.len()).rev() {
            let (gate, args) = gates[i];
            let (l, r, out) = args;
            let out = swaps.get(out).unwrap_or(&out);

            let l = values.get(l);
            if l.is_none() {
                continue;
            }
            let r = values.get(r);
            if r.is_none() {
                continue;
            }
            let l = *l.unwrap();
            let r = *r.unwrap();

            match gate {
                Gate::AND => {
                    values.insert(out, l & r);
                }
                Gate::OR => {
                    values.insert(out, l | r);
                }
                Gate::XOR => {
                    values.insert(out, l ^ r);
                }
            }
            gates.remove(i);
            modified = true;
        }
        if !modified {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1(
            "
                x00: 1
                x01: 0
                x02: 1
                x03: 1
                x04: 0
                y00: 1
                y01: 1
                y02: 1
                y03: 1
                y04: 1

                ntg XOR fgs -> mjb
                y02 OR x01 -> tnw
                kwq OR kpj -> z05
                x00 OR x03 -> fst
                tgd XOR rvg -> z01
                vdt OR tnw -> bfw
                bfw AND frj -> z10
                ffh OR nrd -> bqk
                y00 AND y03 -> djm
                y03 OR y00 -> psh
                bqk OR frj -> z08
                tnw OR fst -> frj
                gnj AND tgd -> z11
                bfw XOR mjb -> z00
                x03 OR x00 -> vdt
                gnj AND wpb -> z02
                x04 AND y00 -> kjc
                djm OR pbm -> qhw
                nrd AND vdt -> hwm
                kjc AND fst -> rvg
                y04 OR y02 -> fgs
                y01 AND x02 -> pbm
                ntg OR kjc -> kwq
                psh XOR fgs -> tgd
                qhw XOR tgd -> z09
                pbm OR djm -> kpj
                x03 XOR y03 -> ffh
                x00 XOR y04 -> ntg
                bfw OR bqk -> z06
                nrd XOR fgs -> wpb
                frj XOR qhw -> z04
                bqk OR frj -> z07
                y03 OR x01 -> nrd
                hwm AND bqk -> z03
                tgd XOR rvg -> z12
                tnw OR pbm -> gnj
            ",
        );
        assert_eq!(result, "2024")
    }
}

use y2019d18::puzzle::calculate1;
use y2019d18::puzzle::calculate2;
use y2019d18::external_solution::solve_first;
use y2019d18::external_solution::solve_second;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(sample_count = 100)]
fn part1() {
    calculate1(divan::black_box(include_str!("../src/input1")));
}
#[divan::bench(sample_count = 100)]
fn part1_ex() {
    solve_first(divan::black_box(include_str!("../src/input1")));
}

#[divan::bench]
fn part2() {
    calculate2(divan::black_box(include_str!("../src/input1")));
}

#[divan::bench]
fn part2_ex() {
    solve_second(divan::black_box(include_str!("../src/input1")));
}


// #[divan::bench(sample_count = 10)]
// fn part_1_4() {
//     calculate1(divan::black_box(
//         "
//                 #################
//                 #i.G..c...e..H.p#
//                 ########.########
//                 #j.A..b...f..D.o#
//                 ########@########
//                 #k.E..a...g..B.n#
//                 ########.########
//                 #l.F..d...h..C.m#
//                 #################
//         "
//     ));
// }
#[divan::bench(sample_count = 100)]
fn part_1_3() {
    calculate1(divan::black_box(
        "
                #################
                #i.G..c...e..H.p#
                ########.########
                #j.A..b...f..D.o#
                ########@########
                #k.E..a...g..B.n#
                ########.########
                #l.F..d...h..C.m#
                #################
        "
    ));
}
#[divan::bench(sample_count = 100)]
fn part_1_3_ex() {
    solve_first(divan::black_box(
        "
                #################
                #i.G..c...e..H.p#
                ########.########
                #j.A..b...f..D.o#
                ########@########
                #k.E..a...g..B.n#
                ########.########
                #l.F..d...h..C.m#
                #################
        "
    ));
}
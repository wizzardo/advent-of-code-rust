use y2019d09::puzzle::calculate1;
use y2019d09::puzzle::calculate2;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    calculate1(divan::black_box(include_str!("../src/input1")));
}

#[divan::bench]
fn part2() {
    calculate2(divan::black_box(include_str!("../src/input1")));
}
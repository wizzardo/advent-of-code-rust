use day_02::puzzle::calculate1;
use day_02::puzzle::calculate2;

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
use y2025d04::puzzle::calculate1;
use y2025d04::puzzle::calculate2;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    calculate1(divan::black_box(include_str!("../src/input")));
}

#[divan::bench]
fn part2() {
    calculate2(divan::black_box(include_str!("../src/input")));
}
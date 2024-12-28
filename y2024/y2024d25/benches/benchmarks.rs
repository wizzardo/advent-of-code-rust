use y2024d25::puzzle::calculate1;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    calculate1(divan::black_box(include_str!("../src/input")));
}

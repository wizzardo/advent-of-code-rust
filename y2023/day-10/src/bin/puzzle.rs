use std::thread;
use day_10::puzzle::{calculate1, calculate2};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() {
    #[cfg(feature = "dhat-heap")]
        let _profiler = dhat::Profiler::new_heap();

    #[cfg(not(feature = "dhat-heap"))]
    tracing_subscriber::fmt::init();

    let input = include_str!("../input1");
    println!("{}", calculate1(input));
    // println!("{}", calculate2(input));

    let child = thread::Builder::new()
        .stack_size(16 * 1024 * 1024)
        .spawn(|| println!("{}", calculate2(input)))
        .unwrap();

    child.join().unwrap();
}
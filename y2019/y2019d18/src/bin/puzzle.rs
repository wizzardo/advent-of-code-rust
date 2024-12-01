use y2019d18::puzzle::{calculate1, calculate2};
use y2019d18::external_solution::solve_first;

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
    // println!("{}", calculate1(input));
    println!("{}", calculate2(input));

    // println!("{}", calculate1("
    //             #################
    //             #i.G..c...e..H.p#
    //             ########.########
    //             #j.A..b...f..D.o#
    //             ########@########
    //             #k.E..a...g..B.n#
    //             ########.########
    //             #l.F..d...h..C.m#
    //             #################
    //     "
    // ));

    // println!("{}", calculate1("
    //             ########################
    //             #@..............ac.GI.b#
    //             ###d#e#f################
    //             ###A#B#C################
    //             ###g#h#i################
    //             ########################
    //     "
    // ));
    // println!("{}", solve_first("
    //             ########################
    //             #f.D.E.e.C.b.A.@.a.B.c.#
    //             ######################.#
    //             #d.....................#
    //             ########################
    //     "
    // ));
    // println!("{}", solve_first("
    //             #################
    //             #i.G..c...e..H.p#
    //             ########.########
    //             #j.A..b...f..D.o#
    //             ########@########
    //             #k.E..a...g..B.n#
    //             ########.########
    //             #l.F..d...h..C.m#
    //             #################
    //     "
    // ));
    // println!("{}", solve_first("
    //             ########################
    //             #@..............ac.GI.b#
    //             ###d#e#f################
    //             ###A#B#C################
    //             ###g#h#i################
    //             ########################
    //     "
    // ));
}
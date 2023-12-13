#[cfg(not(target_arch = "wasm32"))]
use solutions::*;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Eq, PartialEq)]
enum Parts {
    One,
    Two,
    Both,
}

#[cfg(not(target_arch = "wasm32"))]
fn print_day(mut day: Box<dyn Solver>, index: usize, parts: Parts) {
    use std::time::Instant;

    let time_start = Instant::now();
    day.parse().unwrap();
    let parse_time = time_start.elapsed();
    println!("day={} parse - {:?}", index, parse_time);

    if parts == Parts::Both || parts == Parts::One {
        let time_start = Instant::now();
        let part_1 = day.part_1().unwrap();
        let part_1_time = time_start.elapsed();
        println!(
            "day={} part={} solution={} - {:?}",
            index, 1, part_1, part_1_time
        );
    }

    if parts == Parts::Both || parts == Parts::Two {
        let time_start = Instant::now();
        let part_2 = day.part_2().unwrap();
        let part_2_time = time_start.elapsed();

        println!(
            "day={} part={} solution={} - {:?}",
            index, 2, part_2, part_2_time
        );
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let day = 10;
    let example = false;
    let mut solver = solvers::Solvers::get(day).unwrap();

    if example {
        solver.setup(example::get(day));
    } else {
        solver.setup(input::get(day));
    }

    print_day(solver, day, Parts::Both);
}

#[cfg(target_arch = "wasm32")]
fn main() {
    wasm_bindgen_futures::spawn_local(web::bind())
}

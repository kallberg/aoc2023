#[cfg(not(target_arch = "wasm32"))]
use solutions::*;

#[cfg(not(target_arch = "wasm32"))]
fn print_day(day: Box<dyn Solver>, index: usize) {
    let time_start = std::time::Instant::now();
    let part_1 = day.part_1().unwrap();
    let part_1_time = time_start.elapsed();
    let time_start = std::time::Instant::now();
    let part_2 = day.part_2().unwrap();
    let part_2_time = time_start.elapsed();

    println!(
        "day={} part={} solution={}, {:?}",
        index, 1, part_1, part_1_time
    );
    println!(
        "day={} part={} solution={}, {:?}",
        index, 2, part_2, part_2_time
    );
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let day = 6;
    let example = false;
    let mut solver = solvers::Solvers::get(day).unwrap();

    if example {
        solver.setup(example::get(day));
    } else {
        solver.setup(input::get(day));
    }

    solver.parse().unwrap();

    print_day(solver, day);
}

#[cfg(target_arch = "wasm32")]
fn main() {
    wasm_bindgen_futures::spawn_local(web::bind())
}

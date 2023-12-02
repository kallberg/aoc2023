#[cfg(not(target_arch = "wasm32"))]
use solutions::*;

#[cfg(not(target_arch = "wasm32"))]
fn print_day(day: Box<dyn Solver>, index: usize) {
    println!(
        "day={} part={} solution={}",
        index,
        1,
        day.part_1().unwrap()
    );
    println!(
        "day={} part={} solution={}",
        index,
        2,
        day.part_2().unwrap()
    );
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let solver = solvers::Solvers::get(2, input::TWO).unwrap();

    print_day(solver, 2);
}

#[cfg(target_arch = "wasm32")]
fn main() {
    wasm_bindgen_futures::spawn_local(web::bind())
}

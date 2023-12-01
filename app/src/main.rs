use solutions::*;
use solutions::solvers::Solvers;
#[cfg(target_arch = "wasm32")]
use web::*;

fn print_day(day: &Box<dyn Solver>, index: usize) {
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
    let solver = Solvers::get(1, input::ONE).unwrap();

    print_day(&solver, 1);
}

#[cfg(target_arch = "wasm32")]
fn main() {

    wasm_bindgen_futures::spawn_local(web::bind())
}

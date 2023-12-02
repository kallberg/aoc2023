use crate::ui::UIRef;
use gloo_events::EventListener;
use solutions::input::get;
use solutions::solvers::Solvers;
use wasm_bindgen::closure::Closure;

pub struct EventProcessor {
    ui_ref: UIRef,
}

impl EventProcessor {
    pub fn new(ui_ref: UIRef) -> Self {
        Self { ui_ref }
    }

    #[cfg(web_sys_unstable_apis)]
    fn part_1_click_handler(ui_ref: &UIRef) {
        let solution = ui_ref.part_1();

        if let Some(clipboard) = web_sys::window().unwrap().navigator().clipboard() {
            let _promise = clipboard.write_text(&solution);
        };
    }

    #[cfg(web_sys_unstable_apis)]
    fn part_2_click_handler(ui_ref: &UIRef) {
        let solution = ui_ref.part_2();

        if let Some(clipboard) = web_sys::window().unwrap().navigator().clipboard() {
            let _promise = clipboard.write_text(&solution);
        };
    }

    fn day_select_handler(ui_ref: &mut UIRef) {
        ui_ref.clear_outputs();
        ui_ref.set_input(get(ui_ref.day()));
    }

    fn solve_handler(ui_ref: &mut UIRef) {
        ui_ref.clear_outputs();

        let input = ui_ref.input();
        let day: usize = ui_ref.day();

        let Some(mut solver) = Solvers::get(day) else {
            return;
        };

        solver.setup(&input);

        let parse_result =
            gloo_console::Timer::scope(&format!("day {} parse", day), || solver.parse());

        if let Err(error) = parse_result {
            ui_ref.fail_status(day, "parse", error);
            return;
        }

        let part_1_result =
            gloo_console::Timer::scope(&format!("day {} solve 1", day), || solver.part_1());

        let part_1 = match part_1_result {
            Ok(result) => result,
            Err(err) => {
                ui_ref.fail_status(day, "solve part 1", err);
                return;
            }
        };

        let part_2_result =
            gloo_console::Timer::scope(&format!("day {} solve 2", day), || solver.part_2());

        let part_2 = match part_2_result {
            Ok(result) => result,
            Err(err) => {
                ui_ref.fail_status(day, "solve part 2", err);
                return;
            }
        };

        ui_ref.set_part_1(&part_1);
        ui_ref.set_part_2(&part_2);
    }

    pub fn register(&mut self) {
        let solve_event_target = self.ui_ref.solve_event_target();

        let mut ui_clone = self.ui_ref.clone();

        let _solve_listener = EventListener::new(&solve_event_target, "click", move |_event| {
            EventProcessor::solve_handler(&mut ui_clone);
        });

        let mut ui_clone = self.ui_ref.clone();

        let _day_listener = EventListener::new(
            self.ui_ref.day_select_event_target(),
            "change",
            move |_event| {
                EventProcessor::day_select_handler(&mut ui_clone);
            },
        );

        let ui_clone = self.ui_ref.clone();

        let _part_1_copy_listener = EventListener::new(
            self.ui_ref.part_1_button_event_target(),
            "click",
            move |_event| {
                EventProcessor::part_1_click_handler(&ui_clone);
            },
        );

        let ui_clone = self.ui_ref.clone();

        let _part_2_copy_listener = EventListener::new(
            self.ui_ref.part_2_button_event_target(),
            "click",
            move |_event| {
                EventProcessor::part_2_click_handler(&ui_clone);
            },
        );

        _part_2_copy_listener.forget();
        _part_1_copy_listener.forget();
        _day_listener.forget();
        _solve_listener.forget();
    }
}

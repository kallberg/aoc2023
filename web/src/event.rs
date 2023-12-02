use crate::ui::UIRef;
use gloo_events::EventListener;
use solutions::input::get;
use solutions::solvers::Solvers;

pub struct EventProcessor {
    ui_ref: UIRef,
}

impl EventProcessor {
    pub fn new(ui_ref: UIRef) -> Self {
        Self { ui_ref }
    }

    fn day_select_handler(ui_ref: &mut UIRef) {
        ui_ref.set_input(get(ui_ref.day()));
    }

    fn solve_handler(ui_ref: &mut UIRef) {
        let input = ui_ref.input();
        let day: usize = ui_ref.day();

        let Some(solver) = Solvers::get(day, &input) else {
            return;
        };

        let part_1_result = gloo_console::Timer::scope("part-1", || solver.part_1());

        let part_1 = match part_1_result {
            Ok(result) => result,
            Err(err) => {
                ui_ref.set_status(&format!("day_1 error={}", err));
                return;
            }
        };

        let part_2_result = gloo_console::Timer::scope("part-2", || solver.part_2());

        let part_2 = match part_2_result {
            Ok(result) => result,
            Err(err) => {
                ui_ref.set_status(&format!("day_1 error={}", err));
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

        _day_listener.forget();
        _solve_listener.forget();
    }
}

use crate::state::State;
use anyhow::Result;
use solutions::input;
use std::fmt::Display;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{
    EventTarget, HtmlButtonElement, HtmlDivElement, HtmlOptionElement, HtmlOptionsCollection,
    HtmlSelectElement, HtmlTextAreaElement, Location,
};

pub const MAX_DAY: usize = 11;

#[derive(Clone)]
pub struct UIRef {
    input: HtmlTextAreaElement,
    part_1: HtmlDivElement,
    part_2: HtmlDivElement,
    solve: HtmlButtonElement,
    status: HtmlDivElement,
    day: HtmlSelectElement,
    part_1_button: HtmlButtonElement,
    part_2_button: HtmlButtonElement,
    previous: HtmlButtonElement,
    next: HtmlButtonElement,
}

impl UIRef {
    pub fn new(state: State) -> Self {
        let input = gloo_utils::document()
            .get_element_by_id("input")
            .unwrap()
            .dyn_into::<HtmlTextAreaElement>()
            .unwrap();

        let part_1 = gloo_utils::document()
            .get_element_by_id("part-1")
            .unwrap()
            .dyn_into::<HtmlDivElement>()
            .unwrap();

        let part_2 = gloo_utils::document()
            .get_element_by_id("part-2")
            .unwrap()
            .dyn_into::<HtmlDivElement>()
            .unwrap();

        let solve = gloo_utils::document()
            .get_element_by_id("solve")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()
            .unwrap();

        let status = gloo_utils::document()
            .get_element_by_id("status")
            .unwrap()
            .dyn_into::<HtmlDivElement>()
            .unwrap();

        let day = gloo_utils::document()
            .get_element_by_id("day")
            .unwrap()
            .dyn_into::<HtmlSelectElement>()
            .unwrap();

        let part_1_button = gloo_utils::document()
            .get_element_by_id("part-1-button")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()
            .unwrap();

        let part_2_button = gloo_utils::document()
            .get_element_by_id("part-2-button")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()
            .unwrap();

        let previous = gloo_utils::document()
            .get_element_by_id("previous")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()
            .unwrap();

        let next = gloo_utils::document()
            .get_element_by_id("next")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()
            .unwrap();

        day.set_selected_index((state.day - 1) as i32);

        Self {
            input,
            part_1,
            part_2,
            solve,
            status,
            day,
            part_1_button,
            part_2_button,
            previous,
            next,
        }
    }

    fn state(&self) -> State {
        State {
            day: self.day() as u8,
        }
    }

    pub fn solve_event_target(&self) -> EventTarget {
        self.solve.dyn_ref::<EventTarget>().unwrap().clone()
    }

    pub fn day_select_event_target(&self) -> &EventTarget {
        self.day.dyn_ref::<EventTarget>().unwrap()
    }

    pub fn part_1_button_event_target(&self) -> &EventTarget {
        self.part_1_button.dyn_ref::<EventTarget>().unwrap()
    }

    pub fn part_2_button_event_target(&self) -> &EventTarget {
        self.part_2_button.dyn_ref::<EventTarget>().unwrap()
    }

    pub fn next_event_target(&self) -> &EventTarget {
        self.next.dyn_ref::<EventTarget>().unwrap()
    }

    pub fn previous_event_target(&self) -> &EventTarget {
        self.previous.dyn_ref::<EventTarget>().unwrap()
    }

    pub fn day(&self) -> usize {
        let options: HtmlOptionsCollection = self.day.options();
        let index = self.day.selected_index();
        let selection = options
            .get_with_index(index as u32)
            .unwrap()
            .dyn_into::<HtmlOptionElement>()
            .unwrap();

        selection.value().parse().unwrap()
    }

    pub fn save_state(&mut self, location: &mut Location) -> Result<()> {
        self.state().write_location(location)
    }

    pub fn set_input(&mut self, input: &str) {
        self.input.set_value(input);
    }

    pub fn input(&self) -> String {
        self.input.value()
    }

    pub fn set_part_1(&mut self, value: &str) {
        self.part_1.set_inner_text(value);
        self.part_1_button.set_disabled(false);
    }

    pub fn set_part_2(&mut self, value: &str) {
        self.part_2.set_inner_text(value);
        self.part_2_button.set_disabled(false)
    }

    pub fn set_status(&mut self, status: &str) {
        self.status.set_inner_text(status);
    }

    pub fn clear_outputs(&mut self) {
        self.set_part_1("");
        self.set_part_2("");
        self.part_1_button.set_disabled(true);
        self.part_2_button.set_disabled(true);
    }

    pub fn fail_status(&mut self, day: usize, expect: &str, error: impl Display) {
        self.set_status(&format!("day {} expect {} error={}", day, expect, error))
    }

    pub fn part_1(&self) -> String {
        self.part_1.inner_text()
    }

    pub fn part_2(&self) -> String {
        self.part_2.inner_text()
    }

    pub fn handle_day_change(&mut self) {
        self.clear_outputs();
        self.set_input(input::get(self.day()));
        let _result = self.save_state(&mut gloo_utils::window().location());

        if self.day() == MAX_DAY {
            self.next.set_disabled(true);
        } else {
            self.next.set_disabled(false);
        }

        if self.day() == 1 {
            self.previous.set_disabled(true);
        } else {
            self.previous.set_disabled(false);
        }
    }

    pub fn next_day(&mut self) {
        let day = self.day();

        if day < MAX_DAY {
            self.day.set_selected_index(day as i32)
        }

        self.handle_day_change();
    }

    pub fn previous_day(&mut self) {
        let day = self.day();

        if day > 1 {
            self.day.set_selected_index(day as i32 - 2)
        }

        self.handle_day_change();
    }
}

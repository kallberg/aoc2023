use crate::state::State;
use std::fmt::Display;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{
    EventTarget, HtmlButtonElement, HtmlDivElement, HtmlOptionElement, HtmlOptionsCollection,
    HtmlSelectElement, HtmlTextAreaElement, Location,
};

#[derive(Clone)]
pub struct UIRef {
    state: State,
    input: HtmlTextAreaElement,
    part_1: HtmlDivElement,
    part_2: HtmlDivElement,
    solve: HtmlButtonElement,
    status: HtmlDivElement,
    day: HtmlSelectElement,
    part_1_button: HtmlButtonElement,
    part_2_button: HtmlButtonElement,
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
            state,
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

    pub fn save_state(&mut self, location: &mut Location) {
        self.state.day = self.day() as u8;
        self.state.write_location(location);
    }

    pub fn set_input(&mut self, input: &str) {
        self.input.set_value(input);
    }

    pub fn input(&self) -> String {
        self.input.value()
    }

    pub fn set_part_1(&mut self, value: &str) {
        self.part_1.set_inner_text(value);
    }

    pub fn set_part_2(&mut self, value: &str) {
        self.part_2.set_inner_text(value);
    }

    pub fn set_status(&mut self, status: &str) {
        self.status.set_inner_text(status);
    }

    pub fn clear_outputs(&mut self) {
        self.set_part_1("");
        self.set_part_2("");
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
}

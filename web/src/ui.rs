use web_sys::wasm_bindgen::JsCast;
use web_sys::{
    EventTarget, HtmlButtonElement, HtmlDivElement, HtmlOptionElement, HtmlOptionsCollection,
    HtmlSelectElement, HtmlTextAreaElement,
};

#[derive(Clone)]
pub struct UIRef {
    input: HtmlTextAreaElement,
    part_1: HtmlDivElement,
    part_2: HtmlDivElement,
    solve: HtmlButtonElement,
    status: HtmlDivElement,
    day: HtmlSelectElement,
}

impl Default for UIRef {
    fn default() -> Self {
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

        Self {
            input,
            part_1,
            part_2,
            solve,
            status,
            day,
        }
    }
}

impl UIRef {
    pub fn solve_event_target(&self) -> EventTarget {
        self.solve.dyn_ref::<EventTarget>().unwrap().clone()
    }

    pub fn day_select_event_target(&self) -> &EventTarget {
        self.day.dyn_ref::<EventTarget>().unwrap()
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
}

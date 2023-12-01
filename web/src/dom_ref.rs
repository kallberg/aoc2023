use web_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, HtmlDivElement, HtmlTextAreaElement};

pub struct DOMRef {
    pub input: HtmlTextAreaElement,
    pub part_1: HtmlDivElement,
    pub part_2: HtmlDivElement,
    pub solve: HtmlButtonElement,
    pub status: HtmlDivElement,
}

impl Default for DOMRef {
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

        let run = gloo_utils::document()
            .get_element_by_id("solve")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()
            .unwrap();

        let status = gloo_utils::document()
            .get_element_by_id("status")
            .unwrap()
            .dyn_into::<HtmlDivElement>()
            .unwrap();

        Self {
            input,
            part_1,
            part_2,
            solve: run,
            status,
        }
    }
}


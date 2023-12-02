#[cfg(target_arch = "wasm32")]
mod dom_ref;

#[cfg(target_arch = "wasm32")]
mod wasm {
    use dom_ref::DOMRef;
    use futures::channel::mpsc;
    use futures::{Stream, StreamExt};
    use gloo_events::EventListener;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use wasm_bindgen::{JsCast, UnwrapThrowExt};
    use web_sys::{EventTarget, HtmlOptionElement, HtmlOptionsCollection};

    use crate::dom_ref;
    use solutions::solvers::Solvers;

    pub struct EventStream {
        receiver: mpsc::UnboundedReceiver<()>,
        _listener: EventListener,
    }

    impl EventStream {
        pub fn new(target: EventTarget, event_type: &str) -> Self {
            let (sender, receiver) = mpsc::unbounded();

            let _listener = EventListener::new(&target, event_type, move |_event| {
                sender.unbounded_send(()).unwrap_throw()
            });

            Self {
                receiver,
                _listener,
            }
        }
    }

    impl Stream for EventStream {
        type Item = ();

        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
            Pin::new(&mut self.receiver).poll_next(cx)
        }
    }

    pub async fn bind() {
        let dom_ref = DOMRef::default();
        let day: usize = 1;

        let options: HtmlOptionsCollection = dom_ref.day.options();
        let index = dom_ref.day.selected_index();

        let selection = options
            .get_with_index(index as u32)
            .unwrap()
            .dyn_into::<HtmlOptionElement>()
            .unwrap();

        let day: usize = selection.value().parse().unwrap();

        let input = solutions::input::get(day);

        dom_ref.input.set_value(&input);

        let mut run_click_handler = EventStream::new(
            dom_ref.solve.dyn_ref::<EventTarget>().unwrap().clone(),
            "click",
        );

        while run_click_handler.next().await.is_some() {
            let input = dom_ref.input.value();

            let Some(solver) = Solvers::get(day, &input) else {
                continue;
            };

            let part_1_result = gloo_console::Timer::scope("part-1", || solver.part_1());

            let part_1 = match part_1_result {
                Ok(result) => result,
                Err(err) => {
                    dom_ref
                        .status
                        .set_inner_text(&format!("day_1 error={}", err));
                    continue;
                }
            };

            let part_2_result = gloo_console::Timer::scope("part-2", || solver.part_2());

            let part_2 = match part_2_result {
                Ok(result) => result,
                Err(err) => {
                    dom_ref
                        .status
                        .set_inner_text(&format!("day_1 error={}", err));
                    continue;
                }
            };

            dom_ref.part_1.set_inner_text(&part_1);
            dom_ref.part_2.set_inner_text(&part_2);
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::bind;

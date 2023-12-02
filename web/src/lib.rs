
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
    use web_sys::EventTarget;

    use solutions::input::ONE;
    use solutions::solvers::Solvers;
    use crate::dom_ref;

    pub struct RunClickHandler {
        receiver: mpsc::UnboundedReceiver<()>,
        _listener: EventListener,
    }

    impl RunClickHandler {
        pub fn new(target: EventTarget) -> Self {
            let (sender, receiver) = mpsc::unbounded();

            let _listener = EventListener::new(&target, "click", move |_event| {
                sender.unbounded_send(()).unwrap_throw()
            });

            Self { receiver, _listener }
        }
    }

    impl Stream for RunClickHandler {
        type Item = ();

        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
            Pin::new(&mut self.receiver).poll_next(cx)
        }
    }

    pub async fn bind() {
        let dom_ref = DOMRef::default();

        dom_ref.input.set_value(&ONE);

        let mut run_click_handler =
            RunClickHandler::new(dom_ref.solve.dyn_ref::<EventTarget>().unwrap().clone());

        while run_click_handler.next().await.is_some() {
            let input = dom_ref.input.value();

            let Some(solver) = Solvers::get(1, &input) else {
                continue;
            };

            let part_1_result = gloo_console::Timer::scope("part-1", || solver.part_1());

            let part_1 = match part_1_result {
                Ok(result) => result,
                Err(err) => {
                    dom_ref.status.set_inner_text(&format!("day_1 error={}", err));
                    continue;
                }
            };

            let part_2_result = gloo_console::Timer::scope("part-2", || solver.part_2());

            let part_2 = match part_2_result {
                Ok(result) => result,
                Err(err) => {
                    dom_ref.status.set_inner_text(&format!("day_1 error={}", err));
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

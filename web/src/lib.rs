#[cfg(target_arch = "wasm32")]
mod ui;

#[cfg(target_arch = "wasm32")]
mod event;

#[cfg(target_arch = "wasm32")]
mod wasm {
    use ui::UIRef;

    use crate::event::EventProcessor;
    use crate::ui;

    pub async fn bind() {
        let mut ui_ref = UIRef::default();

        let day: usize = ui_ref.day();

        let input = solutions::input::get(day);

        ui_ref.set_input(input);

        let mut event_processor = EventProcessor::new(ui_ref);

        event_processor.register();
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::bind;

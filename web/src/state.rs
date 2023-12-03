use anyhow::Result;
use web_sys::{Location, UrlSearchParams};

#[derive(Clone)]
pub struct State {
    pub day: u8,
}

impl Default for State {
    fn default() -> Self {
        Self { day: 3 }
    }
}

impl State {
    pub fn from_location(location: &Location) -> Option<State> {
        let search = location.search().ok()?;
        let params = UrlSearchParams::new_with_str(&search).ok()?;
        let day_str = params.get("day")?;
        let day = day_str.parse().ok()?;

        Some(State { day })
    }

    pub fn write_location(&self, location: &mut Location) {
        let Ok(params) = UrlSearchParams::new() else {
            return;
        };

        params.set("day", &self.day.to_string());

        let search = params.to_string().as_string().unwrap();

        let _result = location.set_search(&search);
    }
}

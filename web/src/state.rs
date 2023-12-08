use anyhow::{bail, Result};
use web_sys::Location;

#[derive(Clone)]
pub struct State {
    pub day: u8,
}

impl Default for State {
    fn default() -> Self {
        Self { day: 7 }
    }
}

impl State {
    pub fn from_location(location: &Location) -> Option<State> {
        let href: String = location.href().ok()?;
        let split = href.split_once('#')?;
        let state_str = split.1;
        let day = state_str.parse::<u8>().ok()?;

        Some(State { day })
    }

    pub fn write_location(&self, location: &mut Location) -> Result<()> {
        let Ok(mut href) = location.href() else {
            bail!("unable to get href")
        };

        if let Some(split) = href.split_once('#') {
            href = format!("{}#{}", split.0, self.day);
        } else {
            href = format!("{}#{}", href, self.day);
        }

        if location.set_href(&href).is_err() {
            bail!("unable to update href")
        }

        Ok(())
    }
}

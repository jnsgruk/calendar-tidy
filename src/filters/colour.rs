use super::*;
use crate::{config::Config, event::Event};
use anyhow::Result;

/// ColourFilter is used for filtering events based on their colour in the user's calendar.
pub struct ColourFilter {
    /// colours is a list of colours to ignore.
    colours: Vec<String>,
}

impl ColourFilter {
    /// build creates a new ColourFilter from the given Config.
    pub fn build(config: &Config) -> Result<Box<Self>> {
        Ok(Box::new(Self {
            colours: config.ignored_colours.clone(),
        }))
    }
}

impl Filter for ColourFilter {
    /// exclude returns true if the event's colour is in the list of ignored colours.
    fn exclude(&self, event: &Event) -> bool {
        self.colours.contains(&event.color)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exclude() {
        let filter = ColourFilter {
            colours: vec!["9".to_string(), "8".to_string()],
        };

        let event = Event {
            color: "9".to_string(),
            ..Default::default()
        };
        assert!(filter.exclude(&event));

        let event = Event {
            color: "".to_string(),
            ..Default::default()
        };
        assert!(!filter.exclude(&event));
    }
}

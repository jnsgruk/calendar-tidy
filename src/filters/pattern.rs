use super::*;
use crate::config::Config;
use crate::event::Event;
use anyhow::Result;
use regex::Regex;

/// PatternFilter is used for filtering events based on their name.
pub struct PatternFilter {
    regexs: Vec<Regex>,
}

impl PatternFilter {
    /// build creates a new PatternFilter from the given Config.
    pub fn build(config: &Config) -> Result<Box<Self>> {
        Ok(Box::new(Self {
            regexs: config.ignored_regex.clone(),
        }))
    }
}

impl Filter for PatternFilter {
    /// exclude returns true if the event's name matches any of the ignored patterns.
    fn exclude(&self, event: &Event) -> bool {
        self.regexs.iter().any(|r| r.is_match(&event.name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::Config, event::Event};

    #[test]
    fn test_multiple_patterns() {
        let config = Config {
            ignored_regex: vec![
                Regex::new(r"temp.*").unwrap(),
                Regex::new(r".*backup").unwrap(),
            ],
            ..Default::default()
        };

        let filter = PatternFilter::build(&config).unwrap();

        let temp_event = Event {
            name: "temp_file.txt".to_string(),
            ..Default::default()
        };

        let backup_event = Event {
            name: "file.backup".to_string(),
            ..Default::default()
        };

        let keep_event = Event {
            name: "important.doc".to_string(),
            ..Default::default()
        };

        assert!(filter.exclude(&temp_event));
        assert!(filter.exclude(&backup_event));
        assert!(!filter.exclude(&keep_event));
    }
}

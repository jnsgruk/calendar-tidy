use super::*;
use crate::event::Event;
use anyhow::Result;

/// AllDayFilter is used for filtering events that last all day.
pub struct AllDayFilter;

impl AllDayFilter {
    /// build creates a new AllDayFilter.
    pub fn build() -> Result<Box<Self>> {
        Ok(Box::new(Self))
    }
}

impl Filter for AllDayFilter {
    /// exclude returns true if the event lasts all day.
    fn exclude(&self, event: &Event) -> bool {
        event.start_time.timestamp() == 0
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};

    use super::*;

    #[test]
    fn test_exclude_all_day_event() {
        let filter = AllDayFilter::build().unwrap();
        let event = Event {
            start_time: DateTime::<Utc>::from_timestamp(0, 0).unwrap().into(),
            ..Default::default()
        };
        assert!(filter.exclude(&event));
    }

    #[test]
    fn test_include_timed_event() {
        let filter = AllDayFilter::build().unwrap();
        let event = Event {
            start_time: DateTime::<Utc>::from_timestamp(1234567890, 0)
                .unwrap()
                .into(),
            ..Default::default()
        };
        assert!(!filter.exclude(&event));
    }
}

mod all_day;
mod colour;
mod pattern;

use all_day::AllDayFilter;
use colour::ColourFilter;
use pattern::PatternFilter;

use crate::{config::Config, event::Event};
use anyhow::Result;

/// default_filters returns a list of the default filters.
pub fn default_filters(config: &Config) -> Result<Vec<Box<dyn Filter>>> {
    let filters: Vec<Box<dyn Filter>> = vec![
        AllDayFilter::build()?,
        ColourFilter::build(config)?,
        PatternFilter::build(config)?,
    ];

    Ok(filters)
}

/// Filter is the interface for filtering events.
pub trait Filter {
    /// exclude returns true if the event should be excluded.
    fn exclude(&self, event: &Event) -> bool;
}

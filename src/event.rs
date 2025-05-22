use chrono::{DateTime, Local};

#[derive(Debug, Clone, Default)]
pub struct Event {
    /// Unique ID of event
    pub id: String,
    /// Start time of the event. For all day events, this is set to the Unix epoch.
    pub start_time: DateTime<Local>,
    /// Name of the event.
    pub name: String,
}

impl Event {
    /// Build a new event in the context of the current configuration.
    pub fn build(id: String, start: DateTime<chrono::Local>, name: String) -> Self {
        Self {
            id,
            start_time: start,
            name,
        }
    }
}

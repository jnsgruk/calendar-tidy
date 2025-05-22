mod google_calendar;
use crate::event::Event;
use anyhow::Result;
pub use google_calendar::GoogleCalendarClient;

pub trait CalendarClient {
    async fn events(&self) -> Result<Vec<Event>>;
    async fn delete_events(&self, event_ids: Vec<String>) -> Result<()>;
}

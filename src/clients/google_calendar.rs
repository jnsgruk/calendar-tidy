use super::*;

use anyhow::{Context, Result};
use chrono::{DateTime, Duration, NaiveDate, NaiveTime, Utc};
use google_calendar3::api::Event as GCalEvent;
use google_calendar3::hyper_rustls::HttpsConnector;
use google_calendar3::hyper_util::client::legacy::connect::HttpConnector;
use google_calendar3::{CalendarHub, hyper_rustls, hyper_util, yup_oauth2};

use crate::config::Config;
use crate::event::Event;

/// GCalHub is a type alias for the Google Calendar API client.
type GCalHub = CalendarHub<HttpsConnector<HttpConnector>>;

/// GoogleCalendarClient is a client for the Google Calendar API.
pub struct GoogleCalendarClient {
    config: Config,
    hub: GCalHub,
}

impl GoogleCalendarClient {
    /// build creates a new GoogleCalendarClient from the given Config.
    pub async fn build(config: &Config) -> Result<Self> {
        let calendar_hub = Self::auth(config).await?;

        let client = Self {
            config: config.to_owned(),
            hub: calendar_hub,
        };

        Ok(client)
    }

    /// auth authenticated with the Google API and returns an authenticated "hub" object".
    async fn auth(config: &Config) -> Result<GCalHub> {
        let secret = yup_oauth2::read_application_secret(config.credentials_path.clone())
            .await
            .with_context(|| {
                format!(
                    "failed to read supplied credentials file: {}",
                    &config.credentials_path.display(),
                )
            })?;

        let token_storage_path = xdg::BaseDirectories::new()?
            .place_config_file("agendrr/token.json")
            .with_context(|| {
                "failed to cache tokens file at path: $XDG_CONFIG_HOME/agendrr/token.json"
                    .to_string()
            })?;

        let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
            secret,
            yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        )
        .persist_tokens_to_disk(token_storage_path)
        .build()
        .await?;

        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build(
                    hyper_rustls::HttpsConnectorBuilder::new()
                        .with_native_roots()?
                        .https_or_http()
                        .enable_http1()
                        .build(),
                );

        let calendar_hub = CalendarHub::new(client, auth);
        Ok(calendar_hub)
    }

    /// build_agenda_event creates an Event from a Google Calendar event.
    fn build_agenda_event(&self, event: GCalEvent) -> Event {
        let id = event.id.unwrap();
        let start = event
            .start
            .and_then(|s| s.date_time)
            .unwrap_or_default()
            .into();
        let summary = event.summary.unwrap_or_default();

        let color = event.color_id.unwrap_or_else(|| "none".to_string());

        let agendrr_event = Event::build(id, start, summary, color);

        agendrr_event
    }
}

impl CalendarClient for GoogleCalendarClient {
    /// events returns the events for the specified day.
    async fn events(&self) -> Result<Vec<Event>> {
        // Compute the start time for the specified day.
        let date = NaiveDate::parse_from_str(&self.config.start_date, "%d/%m/%Y")?;

        let time_min = DateTime::<Utc>::from_naive_utc_and_offset(
            date.and_time(NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap()),
            Utc,
        );

        // Compute the end time for the specified day.
        let time_max = time_min
            .checked_add_signed(Duration::hours(24 * &self.config.days))
            .context("failed to compute end time")?;

        let result = self
            .hub
            .events()
            .list(&self.config.calendar_id)
            .time_min(time_min)
            .time_max(time_max)
            .single_events(true)
            .add_event_types("default")
            .add_event_types("focusTime")
            .order_by("startTime")
            .doit()
            .await;

        // Maps the received events to the internal representation.
        let events = result?
            .1
            .items
            .unwrap_or(vec![])
            .into_iter()
            .map(|e| self.build_agenda_event(e))
            .collect();

        Ok(events)
    }

    async fn delete_events(&self, event_ids: Vec<String>) -> Result<()> {
        for id in event_ids {
            println!("Deleting event with ID: {}", id);
            self.hub
                .events()
                .delete(&self.config.calendar_id, &id)
                .doit()
                .await?;
        }
        Ok(())
    }
}

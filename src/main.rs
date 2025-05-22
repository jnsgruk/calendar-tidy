mod clients;
mod config;
mod event;
mod filters;

use anyhow::Result;
use clap::Parser;
use cli_table::{Cell, Style, Table};
use clients::{CalendarClient, GoogleCalendarClient};
use config::Config;
use event::Event;
use filters::default_filters;
use inquire::Confirm;
use std::process::exit;

/// A command-line utility to bulk delete events from a Google Calendar.
#[derive(Parser)]
#[command(version, about, long_about)]
struct Cli {
    /// Date to starting clearing from in the format DD/MM/YYYY.
    #[arg(short, long)]
    start_date: String,

    /// Number of days to clear from the start date.
    #[arg(short, long, default_value = "5")]
    days: i64,

    /// Path to the credentials file.
    #[arg(long, default_value = "credentials.json")]
    credentials: String,

    /// Path to the configuration file.
    #[arg(short, long, default_value = "calendar-tidy.yaml")]
    config_file: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Set up the application.
    let args = Cli::parse();
    let config = Config::build(args)?;

    // Build and authenticate the Google Calendar client.
    let client = GoogleCalendarClient::build(&config).await?;

    // Use the default filters and handlers to render the events.
    let filters = default_filters(&config)?;

    // Fetch a vector containing filtered events.
    let filtered_events: Vec<Event> = client
        .events()
        .await?
        .clone()
        .into_iter()
        .filter(|e| !filters.iter().any(|f| f.exclude(e)))
        .collect();

    if filtered_events.is_empty() {
        println!("No events to delete!");
        exit(0);
    }

    // Render events into a table and confirm if the user would like to continue to delete.
    render_events_table(&filtered_events);

    match Confirm::new("Delete events?").with_default(false).prompt() {
        Ok(true) => {
            let ids: Vec<String> = filtered_events.into_iter().map(|e| e.id).collect();
            client.delete_events(ids).await?;
        }
        _ => exit(1),
    }

    Ok(())
}

/// Render a set of events into a command-line table.
fn render_events_table(events: &[Event]) {
    // Get a list of events formatted for output in a table.
    let rows: Vec<Vec<String>> = events
        .iter()
        .map(|e| {
            vec![
                format!("{}", e.start_time.format("%d/%m/%Y %H:%M")),
                e.name.clone(),
            ]
        })
        .collect();

    let table = rows
        .table()
        .title(vec![
            "Time".cell().bold(true),
            "Event Name".cell().bold(true),
        ])
        .bold(true);

    println!("{}", table.display().expect("Unable to draw events table"));
}

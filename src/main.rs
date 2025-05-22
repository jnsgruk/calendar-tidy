mod clients;
mod config;
mod event;
mod filters;

use anyhow::Result;
use clap::Parser;
use clients::{CalendarClient, GoogleCalendarClient};
use config::Config;
use event::Event;
use filters::default_filters;
use inquire::Confirm;
use std::process::exit;

/// A command-line utility to generate a markdown summary of events from Google Calendar.
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

    /// Toggle debug output.
    #[arg(long, default_value = "false")]
    debug: bool,
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

    let events = client.events().await?;

    // Fetch a vector containing rendered events.
    let filtered_events: Vec<Event> = events
        .clone()
        .into_iter()
        .filter(|e| !filters.iter().any(|f| f.exclude(&e)))
        .collect();

    // Print the rendered events.
    let event_strings: Vec<String> = filtered_events
        .iter()
        .map(|e| format!("{} {} {}", e.color, e.start_time, e.name))
        .collect();
    println!("{}", event_strings.join("\n"));

    if filtered_events.len() == 0 {
        println!("No events to delete!");
        exit(0);
    }

    confirm_or_exit();
    let ids: Vec<String> = filtered_events.into_iter().map(|e| e.id).collect();
    client.delete_events(ids).await?;

    Ok(())
}

/// Display a confirmation prompt to the user asking whether they'd like to continue.
/// If they select no, or there is an error - exit the program.
fn confirm_or_exit() {
    let ans = Confirm::new("Delete events?").with_default(false).prompt();

    match ans {
        Ok(true) => (),
        Ok(false) => exit(1),
        Err(_) => exit(1),
    }
}

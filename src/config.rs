use anyhow::{Result, bail};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::Cli;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// The date to clean in the format "DD/MM/YYYY".
    #[serde(default)]
    pub start_date: String,

    /// Number of days to clean.
    #[serde(default)]
    pub days: i64,

    /// The path to the calendar credentials file (usually credentials.json).
    #[serde(default)]
    pub credentials_path: PathBuf,

    /// The ID of the calendar to fetch events from.
    pub calendar_id: String,

    /// A list of colours to ignore (where colour is the event colour on the calendar).
    #[serde(default)]
    pub ignored_colours: Vec<String>,

    /// A list of regexes to ignore, matching on event names.
    #[serde(with = "serde_regex")]
    #[serde(default)]
    pub ignored_regex: Vec<Regex>,
}

impl Config {
    /// Construct the configuration from the filesystem and CLI arguments.
    pub fn build(args: Cli) -> Result<Self> {
        // Check if the config file specified exists.
        if !Path::new(&args.config_file).exists() {
            bail!("config file does not exist: {:?}", args.config_file);
        }

        // Load the configuration from the filesystem.
        let mut cfg: Config = confy::load_path(args.config_file)?;
        // Set the runtime offset from the CLI arguments.
        cfg.start_date = args.start_date;
        cfg.days = args.days;

        Ok(Self {
            credentials_path: PathBuf::from(args.credentials),
            start_date: cfg.start_date,
            days: cfg.days,
            calendar_id: cfg.calendar_id,
            ignored_colours: cfg.ignored_colours,
            ignored_regex: cfg.ignored_regex,
        })
    }
}

# calendar-tidy

A utility for bulk deleting events from a Google Calendar within a given date range, according
to some (optional) filters.

All day events are not removed, and events for deletion can be filtered using regular expression
patterns in the configuration file.

## Installation & Usage

```
# Delete 5 days of events using default config/token location.
calendar-tidy --start-date "2025/07/25"

# Delete 2 days of events using default config/token location.
calendar-tidy -s "2025/07/25" -d 2

# Delete events for a single day using specific token/config file.
calendar-tidy \
  --config-file ~/temp/calendar-tidy.yaml \
  --credentials ~/temp/creds.json \
  --start-date "2025/07/25" \
  --days 1
```


## Installation

You can install `calendar-tidy` with Cargo:

```bash
cargo install --git https://github.com/jnsgruk/calendar-tidy
```

## Command Reference

```
A command-line utility to bulk delete events from a Google Calendar.

Usage: calendar-tidy [OPTIONS] --start-date <START_DATE>

Options:
  -s, --start-date <START_DATE>
          Date to starting clearing from in the format YYYY/MM/DD

  -d, --days <DAYS>
          Number of days to clear from the start date

          [default: 5]

      --credentials <CREDENTIALS>
          Path to the credentials file

          [default: credentials.json]

  -c, --config-file <CONFIG_FILE>
          Path to the configuration file

          [default: calendar-tidy.yaml]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Configuration Reference

There is an example configuration file at [./calendar-tidy.example.yaml], the schema is as follows:

```yaml
# (Required) The ID of the calendar in Google Calendar.
calendar-id: joe.bloggs@example.com

# (Optional) A list of regular expressions that match the titles of events you'd like to ignore.
ignored-regex:
  - "^Some Meeting Name$"
  - "^[C|D]EFG"
```

## Credentials

The script will look for a `credentials.json` file in the same directory as
`calendar-tidy`. A credential can be downloaded once a [desktop OAuth app] has
been created for the [Google Calendar API]. Once the tool has successfully
authenticated once, it will create a `~/.config/calendar-tidy/token.json` file,
which will be used on subsequent runs to get access to the API without
reauthorising.

[desktop OAuth app]: https://developers.google.com/workspace/guides/create-credentials#desktop-app
[Google Calendar API]: https://developers.google.com/calendar/api/guides/overview

## Building `calendar-tidy`.

```bash
# With nix
nix run .#calendar-tidy
# With cargo
cargo build
```

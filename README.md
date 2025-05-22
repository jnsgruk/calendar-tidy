# calendar-tidy

```
# Delete 5 days of events
cargo run -- -c ~/data/general/calendar-tidy.yaml --credentials ~/data/general/agendrr-creds.json --start-date "21/07/2025"

# Delete events for a single day
cargo run -- -c ~/data/general/calendar-tidy.yaml --credentials ~/data/general/agendrr-creds.json --start-date "28/07/2025" --days 1
```

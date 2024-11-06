# Ad Event Log Parser

## Project Description
`ad_event_log_parser` is a Rust-based parser for ad event logs, designed to extract insights from click reports containing data such as transaction IDs, country, OS, IP, status, and revenue details.

This parser is intended for ad-tech analysis, providing breakdowns of event statuses, regional distribution, revenue calculations, and error tracking. It will read each line of log data, parse fields based on a predefined grammar, and output results for further processing or storage.

## Parsing Process
The parsing process is based on structured grammar rules, ensuring that each field aligns with the expected format for accurate analysis. For instance, dates must follow a specific pattern (e.g., `YYYY-MM-DD HH:MM:SS`), and statuses should be checked against known values. Each field is tokenized, verified, and then passed into a structured log entry format.

## Example Usage
To run the parser on a log file:

```bash
cargo run -- log_file.csv

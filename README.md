# Rust Bot - Reddit Post Title Analysis

A Reddit bot written in Rust that parses the titles of posts in the **r/all/hot** subreddit and creates a graph of the number of titles that match certain keywords.

## Features

- Fetches post titles from the **r/all/hot** subreddit.
- Parses the post titles to search for predefined keywords.
- Generates a graph showing the count of post titles that match each keyword.

## Requirements

- **Rust**: Ensure you have [Rust](https://www.rust-lang.org/) installed on your system.
- **Python**: Required for graph generation. You can install Python from [python.org](https://www.python.org/).
- **Dependencies**: The bot depends on several libraries for HTTP requests, parsing, and data visualization. These are defined in the `Cargo.toml` and Python requirements file.

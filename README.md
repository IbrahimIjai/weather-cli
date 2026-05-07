# 🌍 weather-cli

A small, fast command-line tool for fetching live weather from anywhere in the world built in Rust with `clap` and the Open-Meteo API.

```
$ weather-cli weather lagos
Weather in Lagos, Nigeria
  Temperature: 29.4°C
```

---

## About the Project

`weather-cli` is a lightweight CLI that takes a city name and returns the current weather for it. Under the hood it does two things: it geocodes the city name into latitude/longitude coordinates, then queries a weather forecast API for the current conditions at those coordinates. The result is a clean, terminal-friendly weather readout with optional details like humidity and wind speed.

It supports both metric and imperial units, handles cities with spaces and accents correctly, and requires no API key to use.

## Motivation

I built this project for two reasons.

The first was to genuinely learn `clap` — Rust's standard CLI argument parser. Most online tutorials for `clap` are several major versions out of date (the Builder API vs Derive API split, deprecated macros, removed methods), and I wanted to work through the modern Derive API end-to-end on something real.

The second was to get hands-on with the Rust HTTP and JSON ecosystem — `reqwest` for making requests, `serde` for deserializing JSON responses into typed Rust structs, and `tokio` for the async runtime that ties it all together. A weather CLI is the perfect scope: small enough to finish in an afternoon, but big enough to hit every interesting concept.

## Demo

![Demo](demo.mp4)

```
$ weather-cli weather "new york" --units imperial --verbose
Weather in New York, United States
  Temperature: 71.2°F
  Humidity:    64%
  Wind:        8.4 mph
```

## How to Test

### Prerequisites

You need the Rust toolchain installed. If you don't have it:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install Locally

Clone the repo and install the binary to your `~/.cargo/bin` directory (which `rustup` adds to your `PATH` automatically):

```bash
git clone https://github.com/your-username/weather-cli.git
cd weather-cli
cargo install --path .
```

After this, `weather-cli` is available globally in your shell.

### Or Just Run It Without Installing

```bash
cargo run -- weather lagos
```

The `--` separates Cargo's own arguments from the arguments passed to your binary.

### Available Commands

**See the top-level help:**

```bash
weather-cli --help
```

**See help for the weather subcommand:**

```bash
weather-cli weather --help
```

**Basic weather lookup (defaults to metric):**

```bash
weather-cli weather lagos
weather-cli weather tokyo
weather-cli weather "são paulo"
```

**Switch to imperial units:**

```bash
weather-cli weather miami --units imperial
weather-cli weather london -u imperial
```

**Get extended details (humidity, wind speed):**

```bash
weather-cli weather nairobi --verbose
weather-cli weather berlin -v
```

**Combine flags:**

```bash
weather-cli weather "new york" -u imperial -v
```

**Check the version:**

```bash
weather-cli --version
```

## Folder Structure

```
weather-cli/
├── Cargo.toml          # Project manifest and dependencies
├── Cargo.lock          # Locked dependency versions
├── README.md           # You're reading it
├── src/
│   └── main.rs         # All the CLI + HTTP logic
└── target/             # Build artifacts (gitignored)
```

For a project this size, everything lives in `main.rs`. As it grows, the natural split would be moving the HTTP/API code into a `weather.rs` module and keeping `main.rs` focused on CLI plumbing.

## Tech Stack

**Language**

- **Rust (2021 edition)** — chosen for its strong type system, fast startup, and small static binaries that are easy to distribute.

**Crates**

- **`clap`** *(with the `derive` feature)* — declarative command-line parsing. Lets you describe your CLI as a struct and have argument parsing, validation, and `--help` output generated for you at compile time.
- **`reqwest`** *(with `json` feature)* — high-level async HTTP client. Wraps the lower-level `hyper` crate with a friendlier API.
- **`serde`** *(with `derive` feature)* — Rust's standard serialization/deserialization framework. Used here to turn JSON responses into typed structs.
- **`serde_json`** — the JSON-specific implementation of serde. Pulled in transitively by `reqwest`'s `json` feature.
- **`tokio`** — async runtime. Powers the `await` points in the HTTP calls.
- **`urlencoding`** — escapes special characters in URL query parameters so city names with spaces and accents don't break the request.

**APIs**

- **Open-Meteo Geocoding API** (`https://geocoding-api.open-meteo.com/v1/search`) — translates a human-readable city name into latitude, longitude, and country.
- **Open-Meteo Forecast API** (`https://api.open-meteo.com/v1/forecast`) — returns current weather for a given coordinate. Free, no API key required, generous rate limits.

## Outro

This started as a way to actually understand `clap` after fighting with outdated tutorials, and turned into a nice end-to-end tour of the Rust CLI/HTTP ecosystem. If you're learning Rust, I'd genuinely recommend building something like this — small enough to finish, big enough to teach you `serde`, `async/await`, `reqwest`, and `clap` all in one go.

Issues, PRs, and ideas for new subcommands (forecasts, alerts, multi-city diffs) are welcome.

---

Built with ☕ and 🦀.
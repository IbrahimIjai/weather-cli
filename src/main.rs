use clap::{Parser, Subcommand, ValueEnum};
use serde::Deserialize;

#[derive(Parser)]
#[command(name = "weather-cli")]
#[command(version, about = "A handy little CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get the current weather for a city
    Weather {
        /// The city to look up (e.g. "Lagos", "New York")
        city: String,

        /// Temperature units
        #[arg(short, long, value_enum, default_value_t = Units::Metric)]
        units: Units,

        /// Show extra detail (humidity, wind, etc.)
        #[arg(short, long)]
        verbose: bool,
    },
}

#[derive(Copy, Clone, ValueEnum)]
enum Units {
    Metric,
    Imperial,
}

#[derive(Deserialize)]
struct GeoResponse {
    results: Option<Vec<GeoResult>>,
}

#[derive(Deserialize)]
struct GeoResult {
    name: String,
    country: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Deserialize)]
struct WeatherResponse {
    current: Current,
}

#[derive(Deserialize)]
struct Current {
    temperature_2m: f64,
    relative_humidity_2m: f64,
    wind_speed_10m: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Weather { city, units, verbose } => {
            get_weather(&city, units, verbose).await?;
        }
    }
    Ok(())
}

async fn get_weather(
    city: &str,
    units: Units,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Geocode: city name -> coordinates
    let geo_url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1",
        urlencoding::encode(city)
    );
    let geo: GeoResponse = reqwest::get(&geo_url).await?.json().await?;

    let place = match geo.results.and_then(|mut r| r.pop()) {
        Some(p) => p,
        None => {
            eprintln!("Could not find city: {city}");
            std::process::exit(1);
        }
    };

    let temp_unit = match units {
        Units::Metric => "celsius",
        Units::Imperial => "fahrenheit",
    };
    let wind_unit = match units {
        Units::Metric => "kmh",
        Units::Imperial => "mph",
    };

    let weather_url = format!(
        "https://api.open-meteo.com/v1/forecast?\
         latitude={}&longitude={}\
         &current=temperature_2m,relative_humidity_2m,wind_speed_10m\
         &temperature_unit={}&wind_speed_unit={}",
        place.latitude, place.longitude, temp_unit, wind_unit
    );
    let weather: WeatherResponse = reqwest::get(&weather_url).await?.json().await?;

    let temp_symbol = if matches!(units, Units::Metric) { "°C" } else { "°F" };
    let wind_symbol = if matches!(units, Units::Metric) { "km/h" } else { "mph" };

    println!("Weather in {}, {}", place.name, place.country);
    println!("  Temperature: {:.1}{}", weather.current.temperature_2m, temp_symbol);

    if verbose {
        println!("  Humidity:    {:.0}%", weather.current.relative_humidity_2m);
        println!("  Wind:        {:.1} {}", weather.current.wind_speed_10m, wind_symbol);
    }

    Ok(())
}
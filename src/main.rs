use clap::Parser;
use dotenv;
use reqwest::{self, Error};

use crate::data::current_weather::CurrentWeather;
use crate::data::forecast_weather::ForecastWeather;

mod data;

const LAT: f32 = 48.8;
const LON: f32 = 2.3;
const OWM_URL: &str = "https://api.openweathermap.org/data/2.5";

#[derive(Parser)]
#[command(name = "weather")]
#[command(about = "Weather in your terminal", long_about = None)]
struct Args {
    // number of days for the forecast
    #[arg(short, default_value_t = 0)]
    days: u8,
}

fn main() {
    dotenv::dotenv().unwrap();

    let mut api_key = None;
    for (key, value) in std::env::vars() {
        if key == "API_KEY" {
            api_key = Some(value);
        }
    }

    if api_key.is_none() {
        panic!("API_KEY is mandatory !")
    }

    let api_key = api_key.unwrap();

    let args = Args::parse();

    let result = match args.days {
        0 => current_weather(LAT, LON, &api_key),
        other => forecast_weather(LAT, LON, &api_key, other),
    };

    if result.is_err() {
        println!("Error while calling Open Weather Map API");
    }
}

fn current_weather(lat: f32, lon: f32, api_key: &str) -> Result<(), Error> {
    let url = format!("{OWM_URL}/weather?lat={lat}&lon={lon}&appid={api_key}&units=metric");
    let current_weather: CurrentWeather = reqwest::blocking::get(url)?.json()?;

    println!(
        "{} celcius - {}",
        current_weather.main.temp, current_weather.weather[0].description
    );

    Ok(())
}

fn forecast_weather(lat: f32, lon: f32, api_key: &str, days: u8) -> Result<(), Error> {
    let cnt = days * 8;
    let url =
        format!("{OWM_URL}/weather?lat={lat}&lon={lon}&appid={api_key}&cnt={cnt}units=metric");
    let forecast_weather: ForecastWeather = reqwest::blocking::get(url)?.json()?;

    Ok(())
}

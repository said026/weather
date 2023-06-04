use clap::Parser;
use dotenv;
use reqwest::{self, Error};

const LAT: f32 = 48.8;
const LON: f32 = 2.3;

#[derive(Parser)]
#[command(name = "weather")]
#[command(about = "Weather in your terminal", long_about = None)]
struct Args {
    // number of days for the forecast
    #[arg(short, default_value_t = 0)]
    days: u8,
}

fn main() -> Result<(), Error> {
    dotenv::dotenv().unwrap();

    let mut api_key = None;
    for (key, value) in std::env::vars() {
        if key == "API_KEY" {
            println!("{value}");
            api_key = Some(value);
        }
    }

    if api_key.is_none() {
        panic!("API_KEY is mandatory !")
    }

    let api_key = api_key.unwrap();

    let url = format!(
        "https://api.openweathermap.org/data/2.5/forecast?lat={LAT}&lon={LON}&appid={api_key}&units=metric"
    );

    println!("{}", url);

    let body = reqwest::blocking::get(url)?.text()?;

    println!("body = {:?}", body);

    //let args = Args::parse();

    //println!("{}", args.days);

    Ok(())
}

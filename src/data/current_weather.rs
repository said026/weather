use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentWeather {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: i64,
    pub wind: Wind,
    #[serde(default)]
    pub rain: Rain,
    pub clouds: Clouds,
    pub dt: i64,
    pub sys: Sys,
    pub timezone: i64,
    pub id: i64,
    pub name: String,
    pub cod: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Weather {
    pub id: i64,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i64,
    pub humidity: i64,
    #[serde(default)]
    pub sea_level: i64,
    #[serde(default)]
    pub grnd_level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wind {
    pub speed: f64,
    pub deg: i64,
    #[serde(default)]
    pub gust: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rain {
    #[serde(rename = "1h")]
    pub n1h: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Clouds {
    pub all: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sys {
    #[serde(default)]
    pub type_field: i64,
    pub id: i64,
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}

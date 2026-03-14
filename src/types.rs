//! All types needed for Accuweather Api
extern crate serde_derive;
extern crate serde_json;
use std::fmt;

/// Type for most of Accuweather forecasts value
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccuweatherMeasurement {
    /// the actual value
    pub value: f32,
    /// the unit
    pub unit: String,
    /// Accuweather internal unit representation
    unit_type: i32,
}

impl AccuweatherMeasurement {
    pub fn print(&self) {
        println!("{} {}", self.value, self.unit);
    }
}

impl fmt::Debug for AccuweatherMeasurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} ({})", self.value, self.unit, self.unit_type)
    }
}

impl fmt::Display for AccuweatherMeasurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}

/// Represntation of Air and Pollen information
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AirAndPollen {
    pub name: String,
    pub value: i32,
    pub category: String,
    #[allow(dead_code)]
    category_value: i32,
    #[serde(default = "air_pollen_default_type")]
    #[allow(dead_code)]
    r#type: String,
}

/// Representation of wind direction
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct WindDirection {
    pub degrees: f32,
    pub localized: String,
    pub english: String,
}

/// Represnetation of Wind in forecasts api
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Wind {
    pub speed: AccuweatherMeasurement,
    pub direction: WindDirection,
}

/// Representation of wind gust in daily forecasts api
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DailyWindGust {
    #[allow(dead_code)]
    speed: AccuweatherMeasurement,
}

/// Representation of temperature in daily forecast
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Temperature {
    pub maximum: AccuweatherMeasurement,
    pub minimum: AccuweatherMeasurement,
}

/// Represention of forecast for a day part (either night or day) in daily forecast api.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DayPartForecast {
    pub cloud_cover: i32,
    pub hours_of_precipitation: f32,
    pub hours_of_rain: f32,
    pub ice: AccuweatherMeasurement,
    pub ice_probability: i32,
    pub icon: i32,
    pub icon_phrase: String,
    pub long_phrase: String,
    pub precipitation_probability: i32,
    pub rain: AccuweatherMeasurement,
    pub rain_probability: i32,
    pub short_phrase: String,
    pub snow: AccuweatherMeasurement,
    pub snow_probability: i32,
    pub thunderstorm_probability: i32,
    pub total_liquid: AccuweatherMeasurement,
    pub wind: Wind,
    pub wind_gust: Wind,
}

/// Representation of degree day summary in daily forecast api.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DegreeDaySummary {
    pub heating: AccuweatherMeasurement,
    pub cooling: AccuweatherMeasurement,
}

/// Representation of Sun information in daily forecast api.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Sun {
    pub rise: Option<String>,
    pub epoch_rise: Option<i64>,
    pub set: Option<String>,
    pub epoch_set: Option<i64>,
}

/// Representation of Moon information in daily forecast api.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Moon {
    pub rise: Option<String>,
    pub epoch_rise: Option<i64>,
    pub set: Option<String>,
    pub epoch_set: Option<i64>,
    pub phase: Option<String>,
    pub age: i32,
}

/// Representation of daily forecast
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DailyForecast {
    pub air_and_pollen: Vec<AirAndPollen>,
    pub date: String,
    pub day: DayPartForecast,
    pub degree_day_summary: DegreeDaySummary,
    pub epoch_date: i64,
    pub hours_of_sun: f32,
    pub link: String,
    pub mobile_link: String,
    pub moon: Moon,
    pub night: DayPartForecast,
    pub real_feel_temperature: Temperature,
    pub real_feel_temperature_shade: Temperature,
    pub sources: Vec<String>,
    pub sun: Sun,
    pub temperature: Temperature,
}

/// Representation of the Headline part of daily forecast api answer
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Headline {
    pub effective_date: String,
    pub effective_epoch_date: i64,
    pub severity: i32,
    pub text: String,
    pub category: Option<String>,
    pub end_date: Option<String>,
    pub end_epoch_date: Option<i64>,
    pub mobile_link: Option<String>,
    pub link: String,
}

/// Representation of daily forecast api answer
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DailyForecastsAnswer {
    pub headline: Headline,
    pub daily_forecasts: Vec<DailyForecast>,
}

fn air_pollen_default_type() -> String {
    "".to_string()
}

/// Representation of an hourly forecast
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct HourlyForecast {
    pub ceiling: AccuweatherMeasurement,
    pub cloud_cover: i32,
    pub date_time: String,
    pub dew_point: AccuweatherMeasurement,
    pub epoch_date_time: i64,
    pub ice: AccuweatherMeasurement,
    pub ice_probability: i32,
    pub icon_phrase: String,
    pub is_daylight: bool,
    pub link: String,
    pub mobile_link: String,
    pub precipitation_probability: i32,
    pub rain: AccuweatherMeasurement,
    pub rain_probability: i32,
    pub real_feel_temperature: AccuweatherMeasurement,
    pub relative_humidity: i32,
    pub snow: AccuweatherMeasurement,
    pub snow_probability: i32,
    pub temperature: AccuweatherMeasurement,
    pub total_liquid: AccuweatherMeasurement,
    #[serde(rename = "UVIndex")]
    pub uv_index: i32,
    #[serde(rename = "UVIndexText")]
    pub uv_index_text: String,
    pub visibility: AccuweatherMeasurement,
    pub weather_icon: i32,
    pub wet_bulb_temperature: AccuweatherMeasurement,
    pub wind: Wind,
    pub wind_gust: DailyWindGust,
}

/// Representation of LocalSource for current condition api.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LocalSource {
    pub id: i32,
    pub name: String,
    pub weather_code: String,
}

/// Representation of a measurement in current condition api.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ConditionMeasurement {
    pub metric: AccuweatherMeasurement,
    pub imperial: AccuweatherMeasurement,
}

/// Representation of pressure tendency in current condition api.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PressureTendency {
    pub localized_text: String,
    pub code: String,
}

/// Reprensentation of preciipiation summary in current condition api.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PrecipitationSummary {
    pub precipitation: ConditionMeasurement,
    pub past_hour: ConditionMeasurement,
    pub past3_hours: ConditionMeasurement,
    pub past6_hours: ConditionMeasurement,
    pub past9_hours: ConditionMeasurement,
    pub past12_hours: ConditionMeasurement,
    pub past18_hours: ConditionMeasurement,
    pub past24_hours: ConditionMeasurement,
}

/// Representation of temperature summary in current condition api.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TemperatureSummary {
    pub past6_hour_range: TemperatureSummaryRange,
    pub past12_hour_range: TemperatureSummaryRange,
    pub past24_hour_range: TemperatureSummaryRange,
}

/// Representation of temperature summary during a rang of time in current condition api.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TemperatureSummaryRange {
    pub minimum: ConditionMeasurement,
    pub maximum: ConditionMeasurement,
}

/// Representation of wind in current condition api.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct WindCondition {
    pub speed: ConditionMeasurement,
    pub direction: WindDirection,
}

/// Representation of wind gust in current condition api.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ConditionWindGust {
    pub speed: ConditionMeasurement,
}

/// Representation of Current Condion in current condition api.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CurrentCondition {
    pub local_observation_date_time: String,
    pub epoch_time: i64,
    pub weather_text: String,
    pub weather_icon: i32,
    pub local_source: Option<LocalSource>,
    pub is_day_time: bool,
    pub temperature: ConditionMeasurement,
    pub real_feel_temperature: ConditionMeasurement,
    pub real_feel_temperature_shade: ConditionMeasurement,
    pub relative_humidity: i32,
    pub dew_point: ConditionMeasurement,
    pub wind: WindCondition,
    pub wind_gust: ConditionWindGust,
    #[serde(rename = "UVIndex")]
    pub uv_index: i32,
    #[serde(rename = "UVIndexText")]
    pub uv_index_text: String,
    pub visibility: ConditionMeasurement,
    pub obstructions_to_visibility: String,
    pub cloud_cover: i32,
    pub ceiling: ConditionMeasurement,
    pub pressure: ConditionMeasurement,
    pub pressure_tendency: PressureTendency,
    pub past24_hour_temperature_departure: ConditionMeasurement,
    pub apparent_temperature: ConditionMeasurement,
    pub wind_chill_temperature: ConditionMeasurement,
    pub wet_bulb_temperature: ConditionMeasurement,
    pub precip1hr: ConditionMeasurement,
    pub precipitation_summary: PrecipitationSummary,
    pub temperature_summary: TemperatureSummary,
    pub mobile_link: String,
    pub link: String,
    pub has_precipitation: bool,
    pub precipitation_type: Option<String>,
}

extern crate reqwest;
#[macro_use]
extern crate serde_derive;

use crate::types::*;
use reqwest::Client;
use reqwest::Url;
use std::error;
use std::fmt;

pub mod types;

type Result<T> = std::result::Result<T, Box<error::Error>>;

#[derive(Debug, Clone)]
pub struct AccuweatherInvalidParameterError;

impl fmt::Display for AccuweatherInvalidParameterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for AccuweatherInvalidParameterError {
    fn description(&self) -> &str {
        "Invalid Parameter error"
    }
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Debug)]
pub struct Accuweather {
    pub client: Client,
    pub api_key: String,
    pub location: Option<i32>,
    base_url: String,
}

impl Accuweather {
    /// Create an Accuweather client
    ///
    /// It takes as parameters:
    /// * api_key: a String with you api key for Accuweather
    /// * location: An optional id specifying the location to get weather from
    /// #Example
    /// ```
    /// fn main() {
    ///    let api_key = "abcdefg".to_string();
    ///    let client = accuweather::Accuweather::new(api_key, None);
    /// }
    /// ```

    pub fn new(api_key: String, location: Option<i32>) -> Self {
        Accuweather {
            api_key,
            location,
            client: reqwest::Client::builder().build().unwrap(),
            base_url: "http://dataservice.accuweather.com".to_string(),
        }
    }

    /// Set location for an Accuweather client
    ///
    /// Take an Option<i32> to specify location id
    /// # Example
    /// ```
    ///  let api_key = "abcdefg".to_string();
    ///  let mut client = accuweather::Accuweather::new(api_key, None);
    ///  client.set_location(Some(1234));
    ///  assert_eq!(client.location, Some(1234));
    /// ```
    pub fn set_location(&mut self, location: Option<i32>) {
        self.location = location;
    }

    /// Debug with println! a client
    pub fn debug(&self) {
        println!("{:#?}", self);
    }
    /// Get Hourly forecasts for a given period
    ///
    /// Parameters:
    /// * period: A valid accuweather forecasts period in hours as integrer. Can be 1, 12, 24, 72, 120.
    ///
    /// Returns a Result with either a Vec of HourlyForecast or the generated error
    /// # Example
    /// ```
    ///  let api_key = "abcdefg".to_string();
    ///  let client = accuweather::Accuweather::new(api_key, Some(12345));
    ///  client.get_hourly_forecasts(12);
    ///  let forecast_errors = client.get_hourly_forecasts(5);
    ///  assert!(forecast_errors.is_err());
    /// ```

    pub fn get_hourly_forecasts(&self, period: i8) -> Result<Vec<HourlyForecast>> {
        let period = match period {
            1 | 12 | 24 | 72 | 120 => period,
            _ => return Err(AccuweatherInvalidParameterError.into()),
        };
        let url = format!(
            "{}/forecasts/v1/hourly/{}hour/{:?}",
            self.base_url,
            period,
            self.location.unwrap()
        );
        let url = Url::parse_with_params(
            &url,
            &[
                ("apikey", self.api_key.clone()),
                ("details", "true".to_string()),
                ("metric", "true".to_string()),
            ],
        )?;
        match self.client.get(url).send()?.error_for_status()?.json() {
            Ok(x) => Ok(x),
            Err(x) => Err(x.into()),
        }
    }

    /// Get Daily forecasts for a given period
    ///
    /// Parameters:
    /// * period: A valid accuweather forecasts period in hours as integrer. Can be 1, 5, 10, 15.
    ///
    /// Returns a Result with either a DailyForecastAnswer or the generated error
    /// # Example
    /// ```
    ///  let api_key = "abcdefg".to_string();
    ///  let client = accuweather::Accuweather::new(api_key, Some(12345));
    ///  client.get_daily_forecasts(5);
    ///  let forecast_errors = client.get_daily_forecasts(6);
    ///  assert!(forecast_errors.is_err());
    /// ```

    pub fn get_daily_forecasts(&self, period: i8) -> Result<DailyForecastsAnswer> {
        let period = match period {
            1 | 5 | 10 | 15 => period,
            _ => return Err(AccuweatherInvalidParameterError.into()),
        };
        let url = format!(
            "{}/forecasts/v1/daily/{}day/{:?}",
            self.base_url,
            period,
            self.location.unwrap()
        );
        let url = Url::parse_with_params(
            &url,
            &[
                ("apikey", self.api_key.clone()),
                ("details", "true".to_string()),
                ("metric", "true".to_string()),
            ],
        )?;
        match self.client.get(url).send()?.error_for_status()?.json() {
            Ok(x) => Ok(x),
            Err(x) => Err(x.into()),
        }
    }

    /// Get current conditions for location
    ///
    /// Returns a Result with either a Vec of CurrentCondition (with 1 entry) or the generated error
    /// # Example
    /// ```
    ///  let api_key = "abcdefg".to_string();
    ///  let client = accuweather::Accuweather::new(api_key, Some(12345));
    ///  client.get_current_conditions();
    /// ```

    pub fn get_current_conditions(&self) -> Result<Vec<CurrentCondition>> {
        let url = format!(
            "{}/currentconditions/v1/{:?}",
            self.base_url,
            self.location.unwrap()
        );
        let url = Url::parse_with_params(
            &url,
            &[
                ("apikey", self.api_key.clone()),
                ("details", "true".to_string()),
                ("language", "en-us".to_string()),
            ],
        )?;
        match self.client.get(url).send()?.error_for_status()?.json() {
            Ok(x) => Ok(x),
            Err(x) => Err(x.into()),
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

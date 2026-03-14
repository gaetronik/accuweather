//! # Accuweather a crate to interact with accuweather api
//! This crate provides a client to accuweather forecast and current conditions api.
//! At the moment there is only three functions to interact with the api
//!
//! # Example
//! ```
//! extern crate accuweather;
//! #[cfg(doctest)]
//! let _mocks = tests::set_mocks();
//! let api_key = "abcdefg".to_string();
//! let client = accuweather::Accuweather::new(api_key, Some(12345), None);
//! // get next 12 hours of hourly forecasts
//! let hourly_forecasts = client.get_hourly_forecasts(12);
//!
//! let daily_forecasts = client.get_daily_forecasts(5);
//! let conditions = client.get_current_conditions();

extern crate reqwest;
#[macro_use]
extern crate serde_derive;

use crate::types::*;
use reqwest::Client;
use reqwest::Url;
use std::error;
use std::fmt;

pub mod types;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

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
    pub location: Option<String>,
    pub language: String,
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
    ///    let client = accuweather::Accuweather::new(api_key, None, None);
    /// }
    /// ```

    pub fn new(api_key: String, location: Option<String>, language: Option<String>) -> Self {
        #[cfg(not(test))]
        let url = "http://dataservice.accuweather.com";
        #[cfg(test)]
        let url = &mockito::server_url();
        let language = match language {
            Some(l) => l,
            None => "en-us".to_string(),
        };
        Accuweather {
            api_key,
            location,
            language,
            client: reqwest::Client::builder().build().unwrap(),
            base_url: url.to_string(),
        }
    }

    /// Set location for an Accuweather client
    ///
    /// Take an Option<i32> to specify location id
    /// # Example
    /// ```
    ///  let api_key = "abcdefg".to_string();
    ///  let mut client = accuweather::Accuweather::new(api_key, None, None);
    ///  client.set_location(Some(1234));
    ///  assert_eq!(client.location, Some(1234));
    /// ```
    pub fn set_location(&mut self, location: Option<String>) {
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
    ///  let client = accuweather::Accuweather::new(api_key, Some(12345), None);
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
            self.location.as_ref().unwrap()
        );
        let url = Url::parse_with_params(
            &url,
            &[
                ("apikey", self.api_key.clone()),
                ("details", "true".to_string()),
                ("metric", "true".to_string()),
                ("language", self.language.clone()),
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
    ///  let client = accuweather::Accuweather::new(api_key, Some(12345), None);
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
            "{}/forecasts/v1/daily/{}day/{}",
            self.base_url,
            period,
            self.location.as_ref().unwrap()
        );
        let url = Url::parse_with_params(
            &url,
            &[
                ("apikey", self.api_key.clone()),
                ("details", "true".to_string()),
                ("metric", "true".to_string()),
                ("language", self.language.clone()),
            ],
        )?;
        // println!("{:?}", url);
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
    ///  let client = accuweather::Accuweather::new(api_key, Some(12345), Some("fr-fr".to_string()));
    ///  client.get_current_conditions();
    /// ```

    pub fn get_current_conditions(&self) -> Result<Vec<CurrentCondition>> {
        let url = format!(
            "{}/currentconditions/v1/{}",
            self.base_url,
            self.location.as_ref().unwrap()
        );
        let url = Url::parse_with_params(
            &url,
            &[
                ("apikey", self.api_key.clone()),
                ("details", "true".to_string()),
                ("language", self.language.clone()),
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
    use super::*;
    use mockito::{mock, Matcher};
    use std::fs;

    fn set_mocks() -> Vec<mockito::Mock> {
        let mut res = Vec::new();
        let daily5_json = fs::read_to_string("assets/daily5.json").unwrap();
        let _mdnokforbidden = mock("GET", "/forecasts/v1/daily/5day/12345")
            .with_status(403)
            .create();
        res.push(_mdnokforbidden);
        let _mdok = mock("GET", "/forecasts/v1/daily/5day/12345")
            .with_status(200)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("apikey".into(), "abcdefg".into()),
                Matcher::UrlEncoded("details".into(), "true".into()),
                Matcher::UrlEncoded("metric".into(), "true".into()),
                Matcher::UrlEncoded("language".into(), "en-us".into()),
            ]))
            .with_body(&daily5_json)
            .create();
        res.push(_mdok);
        let hourly12_json = fs::read_to_string("assets/hourly12.json").unwrap();
        let _mhnokforbidden = mock("GET", "/forecasts/v1/hourly/12hour/12345")
            .with_status(403)
            .create();
        res.push(_mhnokforbidden);
        let _mhok = mock("GET", "/forecasts/v1/hourly/12hour/12345")
            .with_status(200)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("apikey".into(), "abcdefg".into()),
                Matcher::UrlEncoded("details".into(), "true".into()),
                Matcher::UrlEncoded("metric".into(), "true".into()),
                Matcher::UrlEncoded("language".into(), "en-us".into()),
            ]))
            .with_body(&hourly12_json)
            .create();
        res.push(_mhok);

        let conditions_json = fs::read_to_string("assets/conditions.json").unwrap();
        let _mcnokforbidden = mock("GET", "/currentconditions/v1/12345")
            .with_status(403)
            .create();
        res.push(_mcnokforbidden);
        let _mcok = mock("GET", "/currentconditions/v1/12345")
            .with_status(200)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("apikey".into(), "abcdefg".into()),
                Matcher::UrlEncoded("details".into(), "true".into()),
                Matcher::UrlEncoded("language".into(), "en-us".into()),
            ]))
            .with_body(&conditions_json)
            .create();
        res.push(_mcok);

        res
    }

    #[test]
    fn test_daily_forecast_ok() {
        let _mocks = set_mocks();
        let api_key = "abcdefg".to_string();
        let client = Accuweather::new(api_key, Some(12345), None);
        let res_forecasts = client.get_daily_forecasts(5);
        let forecasts = res_forecasts.unwrap();
        assert_eq!(forecasts.daily_forecasts[0].temperature.minimum.value, 5.4);
    }
    #[test]
    fn test_daily_forecast_nok_forbidden() {
        let _mocks = set_mocks();
        let api_key = "bad_key".to_string();
        let client = Accuweather::new(api_key, Some(12345), None);
        let res_forecasts = client.get_daily_forecasts(5);
        assert!(res_forecasts.is_err());
    }
    #[test]
    fn test_daily_forecast_nok_badlocation() {
        let _mocks = set_mocks();
        let api_key = "bad_key".to_string();
        let client = Accuweather::new(api_key, Some(123456), None);
        let res_forecasts = client.get_daily_forecasts(5);
        assert!(res_forecasts.is_err());
    }

    #[test]
    fn test_hourly_forecast_ok() {
        let _mocks = set_mocks();
        let api_key = "abcdefg".to_string();
        let client = Accuweather::new(api_key, Some(12345), None);
        let res_forecasts = client.get_hourly_forecasts(12);
        let forecasts = res_forecasts.unwrap();
        assert_eq!(forecasts[11].temperature.value, 7.2);
    }
    #[test]
    fn test_hourly_forecast_nok_forbidden() {
        let _mocks = set_mocks();
        let api_key = "bad_key".to_string();
        let client = Accuweather::new(api_key, Some(12345), None);
        let res_forecasts = client.get_hourly_forecasts(12);
        assert!(res_forecasts.is_err());
    }
    #[test]
    fn test_hourly_forecast_nok_badlocation() {
        let _mocks = set_mocks();
        let api_key = "bad_key".to_string();
        let client = Accuweather::new(api_key, Some(123456), None);
        let res_forecasts = client.get_hourly_forecasts(12);
        assert!(res_forecasts.is_err());
    }

    #[test]
    fn test_current_condition_ok() {
        let _mocks = set_mocks();
        let api_key = "abcdefg".to_string();
        let client = Accuweather::new(api_key, Some(12345), None);
        let res_conditions = client.get_current_conditions();
        let conditions = res_conditions.unwrap();
        assert_eq!(conditions[0].temperature.metric.value, 27.9);
    }
    #[test]
    fn test_current_condition_nok_forbidden() {
        let _mocks = set_mocks();
        let api_key = "bad_key".to_string();
        let client = Accuweather::new(api_key, Some(12345), None);
        let res_conditions = client.get_current_conditions();
        assert!(res_conditions.is_err());
    }
    #[test]
    fn test_current_condition_nok_badlocation() {
        let _mocks = set_mocks();
        let api_key = "bad_key".to_string();
        let client = Accuweather::new(api_key, Some(123456), None);
        let res_conditions = client.get_current_conditions();
        assert!(res_conditions.is_err());
    }
}

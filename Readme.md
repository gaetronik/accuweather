# Accuweather crate
[![Build Status](https://travis-ci.org/gaetronik/accuweather.svg)](https://travis-ci.org/gaetronik/accuweather)
[![Crates.io](https://img.shields.io/crates/v/accuweather.svg)](https://crates.io/crates/accuweather)

This crate provides a client to accuweather forecast and current condition api.
At the moment there is only three functions to interact with the api.

## Example
```
extern crate accuweather;

let api_key = "abcdefg".to_string();
let client = accuweather::Accuweather::new(api_key, Some(12345), None);
// get next 12 hours of hourly forecasts
let hourly_forecasts = client.get_hourly_forecasts(12);

let daily_forecasts = client.get_daily_forecasts(5);
let conditions = client.get_current_conditions();
```

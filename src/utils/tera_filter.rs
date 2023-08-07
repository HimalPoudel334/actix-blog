use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_json::{Error, Value};
use tera::Filter;

pub struct Humanizer;

impl Filter for Humanizer {
    fn filter(
        &self,
        value: &serde_json::Value,
        _args: &std::collections::HashMap<String, serde_json::Value>,
    ) -> tera::Result<serde_json::Value> {
        let dt_string: String = serde_json::from_value(value.to_owned()).unwrap();
        let parsed_dt = dt_string.parse::<DateTime<Utc>>().unwrap();
        let datetime_diff = chrono::Utc::now() - parsed_dt;
        Ok(
            serde_json::json!({"humanized_time": chrono_humanize::HumanTime::from(datetime_diff).to_string()}),
        )
    }
}

pub fn humanize(created_on: String) -> String {
    // let dt = chrono::Utc::now() - chrono::Duration::minutes(58);
    let parsed_dt = created_on.parse::<DateTime<Utc>>().unwrap();

    let datetime_diff = chrono::Utc::now() - parsed_dt;
    chrono_humanize::HumanTime::from(datetime_diff).to_string()
}

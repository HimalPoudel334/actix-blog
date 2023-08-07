use std::collections::HashMap;

use chrono::{DateTime, Utc};
use chrono_humanize::Accuracy;
use serde_json::Value;

// Custom filter function for humanizing DateTime<Utc>
// TODO: refactor it so that only exact minutes, hours, days
pub fn humanize_dt_filter(
    value: &Value,
    _args: &HashMap<String, serde_json::Value>,
) -> tera::Result<Value> {
    let dt_string: String = serde_json::from_value(value.to_owned()).unwrap();
    let parsed_dt = dt_string.parse::<DateTime<Utc>>().unwrap();
    let datetime_diff = parsed_dt - chrono::Utc::now();
    let humanized_time = chrono_humanize::HumanTime::from(datetime_diff);

    Ok(Value::String(
        humanized_time.to_text_en(Accuracy::Rough, chrono_humanize::Tense::Past),
    ))
}

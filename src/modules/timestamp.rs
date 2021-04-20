use crate::modules::{Module, TimestampModule};
use chrono::{DateTime, NaiveDateTime, Utc};
use log::info;
use std::error;

impl Module for TimestampModule {
    fn name(&self) -> String {
        String::from("d")
    }

    fn execute(&self, input: String) -> Result<String, Box<dyn error::Error>> {
        info!("Getting date for timestamp {:?}", input);
        let timestamp = input.parse::<i64>();
        match timestamp {
            Ok(timestamp) => {
                let dt =
                    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);
                Result::Ok(dt.to_rfc3339())
            }
            Err(e) => Result::Err(Box::new(e)),
        }
    }
}

#[test]
fn it_should_get_timestamp() {
    let timestamp = TimestampModule {};
    let actual = timestamp.execute(String::from("0")).unwrap();
    assert_eq!(actual, "1970-01-01T00:00:00+00:00");
}

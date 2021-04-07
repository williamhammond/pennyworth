use chrono::{DateTime, NaiveDateTime, Utc};
use log::info;
use std::error;
use std::fmt::Debug;

pub trait Module: Debug {
    fn name(&self) -> String;
    fn execute(&self, input: String) -> Result<String, Box<dyn error::Error>>;

    fn is_match(&self, command_name: String) -> bool {
        command_name == self.name()
    }
}

#[derive(Debug)]
pub struct TimestampModule {}

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

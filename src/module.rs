use crate::errors::ExecutionError;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::fmt::Debug;

pub trait Module: Debug {
    fn name(&self) -> String;

    fn execute(&self, input: String) -> Result<String, ExecutionError>;
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

    fn execute(&self, input: String) -> Result<String, ExecutionError> {
        let timestamp = input.parse::<i64>().unwrap();
        let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);

        Result::Ok(dt.to_rfc3339())
    }
}

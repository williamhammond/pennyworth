pub mod launcher;
pub mod timestamp;

use std::error;
use std::fmt::Debug;

pub trait Module: Debug {
    fn name(&self) -> String;
    fn execute(&self, input: String) -> Result<String, Box<dyn error::Error>>;

    fn is_match(&self, command_name: &str) -> bool {
        command_name == self.name()
    }
}

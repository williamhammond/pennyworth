use crate::Module;
use std::error;

#[derive(Debug)]
pub struct LauncherModule {}

impl Module for LauncherModule {
    fn name(&self) -> String {
        String::from("d")
    }

    fn execute(&self, input: String) -> Result<String, Box<dyn error::Error>> {
        unimplemented!();
    }
}

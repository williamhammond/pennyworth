use crate::sql_client::SQLClient;
use crate::Module;
use freedesktop_entry_parser::desktop_entry::DesktopEntry;
use std::error;

#[derive(Debug)]
pub struct LauncherModule {
    sql_client: SQLClient,
}

impl LauncherModule {
    fn new() -> LauncherModule {
        LauncherModule {
            sql_client: SQLClient::new(),
        }
    }

    fn index_applications() {
        let current_desktop =
            std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| String::from(""));
    }

    fn should_index(entry: DesktopEntry) -> bool {
        return false;
    }
}

impl Module for LauncherModule {
    fn name(&self) -> String {
        String::from("d")
    }

    fn execute(&self, input: String) -> Result<String, Box<dyn error::Error>> {
        unimplemented!();
    }
}

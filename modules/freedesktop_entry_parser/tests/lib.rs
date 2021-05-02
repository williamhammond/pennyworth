#[cfg(test)]
mod tests {

    #[test]
    fn it_parses_dirs() {
        let paths: &'static [&'static str] = &["/usr/share/applications/"];

        let desktop_entries = freedesktop_entry_parser::get_entries_in_dirs(paths);
        assert!(desktop_entries.len() > 0, true);
    }

    #[test]
    fn it_parses_only_valid_dirs() {
        let paths: &'static [&'static str] = &["/usr/share/applications/", "/usr/donesnt/exists/"];

        let desktop_entries = freedesktop_entry_parser::get_entries_in_dirs(paths);
        assert!(desktop_entries.len() > 0, true);
    }

    #[test]
    fn it_gets_all_entries() {
        let paths: &'static [&'static str] = &["/usr/share/applications/"];

        let desktop_entries = freedesktop_entry_parser::get_entries_in_dirs(paths);

        for desktop_entry in desktop_entries {
            //println!("{}", desktop_entry.name);
        }
    }

    #[test]
    fn it_gets_applications() {
        let paths: &'static [&'static str] = &["/usr/share/applications/"];

        let application_entries = freedesktop_entry_parser::get_entries_in_dirs_filtered_by(
            paths,
            freedesktop_entry_parser::EntryType::Application,
        );

        for desktop_entry in application_entries {
            //print!("{:?}", desktop_entry);
        }
    }
}

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
}

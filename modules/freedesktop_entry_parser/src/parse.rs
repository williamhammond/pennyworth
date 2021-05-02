use desktop_entry::EntryType;

/// Parses a `&str` and returns the corresponding `Result<EntryType, &'static str>`.
pub fn parse_entry_type(s: &str) -> Result<EntryType, &'static str> {
    match s {
        "Application" => Ok(EntryType::Application),
        "Link" => Ok(EntryType::Link),
        "Directory" => Ok(EntryType::Directory),
        _ => Err("Unknown entry type")
    }
}

/// This function sets the `result` to the result of the parsing operation if the `line` starts
/// with `starts_with`. It also returns a `bool` indicating whether or not there was a change in
/// the `result` variable.
pub fn parse_if_starts_with<'a>(line: &'a str, result: &mut Option<String>, starts_with: &'a str) -> bool {
    if line.starts_with(starts_with) {
        if let Some(val) = line.get(starts_with.len()..) {
            *result = Some(val.to_string());
            return true;
        }
    };

    false
}

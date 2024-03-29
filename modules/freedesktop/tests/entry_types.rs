extern crate freedesktop;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    const TEST_FILES_DIR: &str = "./test_files/entries";

    #[test]
    fn it_parses_application() {
        let path_buf: PathBuf = [TEST_FILES_DIR, "test_app.desktop"].iter().collect();
        let file = freedesktop::parse_file(&path_buf).unwrap();

        assert_eq!(file.entry_type, freedesktop::EntryType::Application);
        assert_eq!(file.name, "Test App");
        assert_eq!(file.exec.unwrap(), "test-app");
        assert_eq!(file.comment.unwrap(), "A test application comment");
    }

    #[test]
    fn it_parses_link() {
        let path_buf: PathBuf = [TEST_FILES_DIR, "test_link.desktop"].iter().collect();
        let file = freedesktop::parse_file(&path_buf).unwrap();

        assert_eq!(file.entry_type, freedesktop::EntryType::Link);
        assert_eq!(file.name, "Test Link");
        assert_eq!(file.comment.unwrap(), "A test link comment");
    }

    #[test]
    fn it_parses_directory() {
        let path_buf: PathBuf = [TEST_FILES_DIR, "test_directory.desktop"].iter().collect();
        let file = freedesktop::parse_file(&path_buf).unwrap();

        assert_eq!(file.entry_type, freedesktop::EntryType::Directory);
        assert_eq!(file.name, "Test Directory");
        assert_eq!(file.comment.unwrap(), "A test directory comment");
    }

    #[test]
    fn it_fails_if_invalid_entry_type() {
        let path_buf: PathBuf = [TEST_FILES_DIR, "test_invalid_entry_type.desktop"]
            .iter()
            .collect();
        let file = freedesktop::parse_file(&path_buf);

        assert!(file.is_err());
    }
}

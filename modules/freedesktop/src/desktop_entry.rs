use std::fmt;

#[derive(PartialEq, Debug)]
pub enum EntryType {
    Application,
    Link,
    Directory
}

pub struct DesktopEntry {
    pub name: String,
    pub exec: Option<String>,
    pub comment: Option<String>,
    pub entry_type: EntryType,
}

impl fmt::Debug for DesktopEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = writeln!(f, "Name: {}", &self.name);
        res = res.and(print_if_some(f, "Exec:", &self.exec));
        res.and(print_if_some(f, "Comment:", &self.comment))
    }
}

fn print_if_some<'a>(f: &mut fmt::Formatter, attribute_name: &'static str, value: &Option<String>) -> fmt::Result {
    match *value {
        Some(ref val) => writeln!(f, "{} {}", attribute_name, val),
        None => Ok(())
    }
}

use rusqlite::{Connection, Result};

static CREATE_TABLE_QUERY: &str =
    "CREATE TABLE IF NOT EXISTS applications (name TEXT PRIMARY KEY);";
static ADD_ENTRY_QUERY: &str = "INSERT INTO applications (name) VALUES (?)";
static SELECT_APP_ENTRIES: &str = "SELECT * FROM applications";

#[derive(Debug)]
pub struct SQLClient {
    conn: Connection,
}

#[derive(Debug)]
struct ApplicationEntry {
    name: String,
}

impl SQLClient {
    pub fn new() -> SQLClient {
        SQLClient {
            conn: Connection::open_in_memory().unwrap(),
        }
    }
    pub fn add_application_entry(&self, entry: &ApplicationEntry) {
        self.conn.execute(CREATE_TABLE_QUERY, []).unwrap();
        self.conn.execute(ADD_ENTRY_QUERY, [&entry.name]).unwrap();
    }

    pub fn get_applications(&self) -> Vec<Result<ApplicationEntry>> {
        let mut select_statement = self.conn.prepare(SELECT_APP_ENTRIES).unwrap();
        return select_statement
            .query_map([], |row| {
                Ok(ApplicationEntry {
                    name: row.get(0).unwrap(),
                })
            })
            .unwrap()
            .collect();
    }
}

#[test]
fn it_inserts_app() {
    let entry = ApplicationEntry {
        name: String::from("name"),
    };
    let sql_client = SQLClient::new();
    sql_client.add_application_entry(&entry);
    let applications = sql_client.get_applications();
    assert_eq!(applications.len(), 1);
}

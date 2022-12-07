use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct SQLClient {
    conn: Connection,
}

#[derive(Debug)]
pub struct ApplicationEntry {
    name: String,
}

impl SQLClient {
    const CREATE_TABLE_QUERY: &'static str =
        "CREATE TABLE IF NOT EXISTS applications (name TEXT PRIMARY KEY);";
    const SELECT_APPLICATIONS_QUERY: &'static str = "SELECT * FROM APPLICATIONS";
    const ADD_APPLICATION_QUERY: &'static str = "INSERT INTO applications (name) VALUES (?)";

    pub fn new() -> SQLClient {
        SQLClient {
            conn: Connection::open_in_memory().unwrap(),
        }
    }
    pub fn add_application_entry(&self, entry: &ApplicationEntry) {
        self.conn
            .execute(SQLClient::CREATE_TABLE_QUERY, [])
            .unwrap();
        self.conn
            .execute(SQLClient::ADD_APPLICATION_QUERY, [&entry.name])
            .unwrap();
    }

    pub fn get_applications(&self) -> Vec<Result<ApplicationEntry>> {
        let mut select_statement = self
            .conn
            .prepare(SQLClient::SELECT_APPLICATIONS_QUERY)
            .unwrap();
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

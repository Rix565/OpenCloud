use crate::lib::db::conn::conn;

pub fn create() {
    let conn = conn();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS User (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  password        TEXT NOT NULL,
                  token           TEXT,
                  email           TEXT
                  )",
        "".bytes(),
    )
    .expect("Error");
}

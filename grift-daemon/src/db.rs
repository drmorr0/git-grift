use std::sync::{
    Arc,
    Mutex,
};

use grift_core::prelude::*;
use rusqlite::Connection;
use tracing::*;

const TRACKED_REPOS_TABLE: &str = "tracked_repos";

#[derive(Clone)]
pub struct GriftDB {
    conn: Arc<Mutex<Connection>>,
    db_path: String,
}

impl GriftDB {
    pub fn open() -> eyre::Result<GriftDB> {
        let db_path = XDG_DIRS.place_data_file("grift.sqlite")?;
        let pp_db_path = db_path.to_string_lossy().to_string();

        info!("Connecting to database at {pp_db_path}");

        let conn = Connection::open(db_path)?;
        conn.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS {TRACKED_REPOS_TABLE} (
                id   INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL,
                UNIQUE(path)
            )"
            ),
            (),
        )?;

        Ok(GriftDB {
            conn: Arc::new(Mutex::new(conn)),
            db_path: pp_db_path,
        })
    }

    pub fn track_repo(&self, repo_path: &str) -> Empty {
        self.conn
            .lock()
            .unwrap()
            .execute(&format!("INSERT INTO {TRACKED_REPOS_TABLE} (path) VALUES (?1)"), (repo_path,))?;

        Ok(())
    }

    pub fn path(&self) -> &str {
        &self.db_path
    }
}

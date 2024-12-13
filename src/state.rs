use rusqlite::Connection;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub conn: Arc<Connection>,
}

impl AppState {
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Arc::new(conn),
        }
    }
}

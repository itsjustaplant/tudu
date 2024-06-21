use std::io::{Error, ErrorKind};

use rusqlite::{Connection, Result};

use crate::constants;
use crate::filesystem::get_app_config_path;
use crate::task::Task;

#[derive(Debug, Default)]
pub struct Client {
    pub connection: Option<Connection>,
}

impl Client {
    pub fn get_connection(&self) -> &Connection {
        self.connection.as_ref().expect("Could not find connection")
    }

    pub fn open_connection(&mut self) -> Result<(), Error> {
        let mut app_config_path = get_app_config_path()?;
        app_config_path.push(constants::DB_NAME);

        match Connection::open(app_config_path) {
            Ok(connection) => {
                self.connection = Some(connection);
                Ok(())
            }
            Err(e) => Err(Error::new(
                ErrorKind::Other,
                format!("Could not close connection, e: {}", e),
            )),
        }
    }

    pub fn close_connection(&mut self) -> Result<(), Error> {
        match self.connection.take() {
            Some(connection) => connection
                .close()
                .map_err(|_| Error::new(ErrorKind::Other, "Could not close connection")),
            None => Err(Error::new(ErrorKind::Other, "Could not find connection")),
        }
    }

    pub fn crete_todos_table(&self) -> Result<usize, Error> {
        let query = "CREATE TABLE IF NOT EXISTS todos (
                     title TEXT,
                     status TEXT
                    );";
        self.get_connection().execute(query, []).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("Could not create todos table, e: {}", e),
            )
        })
    }

    pub fn get_tasks(&self) -> Result<Vec<Task>> {
        let mut stmt = self.get_connection().prepare("SELECT * FROM todos")?;
        let rows = stmt.query_map([], |row| {
            Ok(Task {
                title: row.get(0)?,
                status: row.get(1)?,
            })
        })?;

        let mut tasks = Vec::new();
        for task_result in rows {
            tasks.push(task_result?);
        }

        Ok(tasks)
    }

    pub fn create_task(&self, title: &str) -> Result<usize, Error> {
        self.get_connection()
            .execute(
                format!(
                    "INSERT INTO todos (title, status) VALUES('{}', 'in-progress')",
                    title
                )
                .as_str(),
                [],
            )
            .map_err(|e| Error::new(ErrorKind::Other, format!("Could not insert task, e: {}", e)))
    }

    pub fn test(&self) {
        self.get_connection()
            .execute(
                "INSERT INTO todos (title, status) VALUES('delete this', 'in-progress')",
                [],
            )
            .unwrap();
    }
}

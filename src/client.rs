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
                     id INTEGER NOT NULL PRIMARY KEY,
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
                id: row.get(0)?,
                title: row.get(1)?,
                status: row.get(2)?,
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
                "INSERT INTO todos (title, status) VALUES(?1, 'in-progress')",
                [title],
            )
            .map_err(|e| Error::new(ErrorKind::Other, format!("Could not insert task, e: {}", e)))
    }

    pub fn remove_task(&self, id: i32) -> Result<usize, Error> {
        self.get_connection()
            .execute("DELETE FROM todos where id=?1", [id])
            .map_err(|e| Error::new(ErrorKind::Other, format!("Could not remove task, e: {}", e)))
    }

    pub fn update_task(&self, id: i32, current_status: &str) -> Result<usize, Error> {
        let new_status = if current_status == "in-progress" {
            "completed"
        } else {
            "in-progress"
        };
        self.get_connection()
            .execute("UPDATE todos SET status=?1 WHERE id=?2", (new_status, id))
            .map_err(|e| Error::new(ErrorKind::Other, format!("Could not update task, e: {}", e)))
    }
}

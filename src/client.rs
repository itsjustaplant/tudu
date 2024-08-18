use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use rusqlite::{Connection, Result};

use crate::task::Task;
use crate::user::User;

#[derive(Debug, Default)]
pub struct Client {
    pub connection: Option<Connection>,
}

impl Client {
    pub fn get_connection(&self) -> Result<&Connection, Error> {
        match &self.connection {
            Some(connection) => Ok(&connection),
            None => Err(Error::new(
                ErrorKind::Other,
                String::from("Could not open connection"),
            )),
        }
    }

    pub fn open_connection(&mut self, mut app_config_path: PathBuf, db_name: &str) -> Result<(), Error> {
        app_config_path.push(db_name);

        match Connection::open(app_config_path) {
            Ok(connection) => {
                self.connection = Some(connection);
                Ok(())
            }
            Err(e) => Err(Error::new(
                ErrorKind::Other,
                format!("Could not open connection, e: {}", e),
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

    pub fn create_todos_table(&self) -> Result<usize, Error> {
        let query = "CREATE TABLE IF NOT EXISTS todos (
                     id INTEGER NOT NULL PRIMARY KEY,
                     title TEXT,
                     status TEXT
                    );";
        let connection = self.get_connection();

        match connection {
            Ok(c) => {
                let result = c.execute(query, []);
                match result {
                    Ok(r) => Ok(r),
                    // How do i test here
                    Err(e) => Err(Error::new(
                        ErrorKind::Other,
                        format!("Could not create todos table, e: {}", e),
                    )),
                }
            }
            Err(e) => Err(Error::new(
                ErrorKind::Other,
                format!("Could not get connection, e: {}", e),
            )),
        }
    }

    pub fn create_user_table(&self) -> Result<usize, Error> {
        let query = "CREATE TABLE IF NOT EXISTS user (
                   id INTEGER NOT NULL PRIMARY KEY,
                   secret TEXT
                  );";
        let connection = self.get_connection();

        match connection {
            Ok(c) => {
                let result = c.execute(query, []);
                match result {
                    Ok(r) => Ok(r),
                    Err(e) => Err(Error::new(
                        ErrorKind::Other,
                        format!("Could not create user table, e: {}", e),
                    )),
                }
            }
            Err(e) => Err(Error::new(
                ErrorKind::Other,
                format!("Could not get connection, e: {}", e),
            )),
        }
    }

    pub fn get_tasks(&self) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        let mut stmt = self.get_connection()?.prepare("SELECT * FROM todos")?;
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

    pub fn create_task(&self, title: String) -> Result<usize, Error> {
        self.get_connection()?
            .execute(
                "INSERT INTO todos (title, status) VALUES(?1, 'in-progress')",
                [format!("{:?}", title)],
            )
            .map_err(|e| Error::new(ErrorKind::Other, format!("Could not insert task, e: {}", e)))
    }

    pub fn create_user(&self, secret: String) -> Result<usize, Error> {
        self.get_connection()?
            .execute(
                "INSERT INTO user (secret) VALUES(?1)",
                [format!("{:?}", secret)],
            )
            .map_err(|e| Error::new(ErrorKind::Other, format!("Could not insert user, e: {}", e)))
    }

    pub fn get_user(&self) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        let mut stmt = self
            .get_connection()?
            .prepare("SELECT * FROM user where id=1")?;
        let rows = stmt.query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                secret: row.get(1)?,
            })
        })?;

        let mut user = Vec::new();
        for user_result in rows {
            user.push(user_result?);
        }

        Ok(user)
    }

    pub fn remove_task(&self, id: i32) -> Result<usize, Error> {
        self.get_connection()?
            .execute("DELETE FROM todos where id=?1", [id])
            .map_err(|e| Error::new(ErrorKind::Other, format!("Could not remove task, e: {}", e)))
    }

    pub fn update_task(&self, id: i32, current_status: &str) -> Result<usize, Error> {
        let new_status = if current_status == "in-progress" {
            "completed"
        } else {
            "in-progress"
        };
        self.get_connection()?
            .execute("UPDATE todos SET status=?1 WHERE id=?2", (new_status, id))
            .map_err(|e| Error::new(ErrorKind::Other, format!("Could not update task, e: {}", e)))
    }

    pub fn remove_user(&self) -> Result<usize, Error> {
        self.get_connection()?
            .execute("DELETE FROM user", [])
            .map_err(|e| Error::new(ErrorKind::Other, format!("Could not remove user, e: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::{self, DB_NAME};

    use super::*;

    #[test]
    fn test_connection_operations() {
        let mut client = Client::default();
        let mut path = PathBuf::new();

        let title = String::from("Test client module");
        let secret = String::from("SECRET");

        path.push("./test/client/");

        client
            .open_connection(path, constants::DB_NAME)
            .expect("Could not open connection");
        client
            .create_todos_table()
            .expect("Could not create todos table");
        client
            .create_user_table()
            .expect("Could not create user table");

        let mut tasks = client.get_tasks().expect("Could not get tasks");

        assert_eq!(tasks.len(), 0);

        client.create_task(title).expect("Could not create task");
        client.create_user(secret).expect("Could not create user");

        tasks = client.get_tasks().expect("Could not get tasks");
        let users = client.get_user().expect("Could not get user");

        assert_eq!(tasks.len(), 1);
        assert_eq!(users.len(), 1);

        client
            .update_task(1, "in-progress")
            .expect("Could not update task");
        tasks = client.get_tasks().expect("Could not get tasks");
        let task = tasks.get(0).expect("Could not get task 0");

        assert_eq!(task.status, "completed");

        client
            .update_task(1, "completed")
            .expect("Could not update task");
        tasks = client.get_tasks().expect("Could not get tasks");
        let task = tasks.get(0).expect("Could not get task 0");

        assert_eq!(task.status, "in-progress");

        client.remove_task(1).expect("Could not remove connection");
        client.remove_user().expect("Could not remove user");
        client
            .close_connection()
            .expect("Could not close connection");
    }

    #[test]
    fn test_open_connection_error() {
        let mut client = Client::default();
        let mut path = PathBuf::new();

        path.push("./bad_test_path");
        let result = client.open_connection(path, DB_NAME);
        assert!(result.is_err());
    }

    #[test]
    fn test_close_connection_error() {
        let mut client = Client::default();

        let result = client.close_connection();
        assert!(result.is_err());
    }

    #[test]
    fn test_todos_table_creation_error() {
        let mut client = Client::default();
        let mut path = PathBuf::new();
        
        let mut result = client.create_todos_table();
        assert!(result.is_err());

        path.push("./test/bad_db/");
        client.open_connection(path, "tudu.txt").expect("Could not create connection");
        result = client.create_todos_table();
        assert!(result.is_err());
    }

    #[test]
    fn test_user_table_creation_error() {
        let mut client = Client::default();
        let mut path = PathBuf::new();
        let mut result = client.create_user_table();

        assert!(result.is_err());

        path.push("./test/bad_db/");
        client.open_connection(path, "tudu.txt").expect("Could not create connection");
        result = client.create_user_table();
        assert!(result.is_err());
    }
}

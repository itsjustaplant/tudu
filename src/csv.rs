use std::{error::Error, path::PathBuf};

use csv::Writer;

use crate::task::Task;

pub fn write_tasks_into_csv_file(task_list: &Vec<Task>, path: &PathBuf) -> Result<(), Box<dyn Error>> {
  let mut writer = Writer::from_path(path)?;
  let mut index = 0;
  
  for task in task_list.iter() {
    let title = task.title.as_str();
    let status = task.status.as_str();
    let record_index = format!("{}", index);
    let record = &[record_index.as_str(), title, status];
    
    writer.write_record(record)?;
    index += 1;
  }
  writer.flush()?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_write_file() {
    let task = Task {
      id: 0,
      title: String::from("Title"),
      status: String::from("completed")
    };

    let task_list = vec![task];

    let mut path = PathBuf::new();
    path.push("./test/csv/tudu.csv");

    let result = write_tasks_into_csv_file(&task_list, &path);
    assert!(result.is_ok());
  }
}
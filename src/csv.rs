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

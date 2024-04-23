extern crate uuid;
use std::fmt;
use uuid::Uuid;
use std::str::FromStr;
#[derive(Debug)]
pub enum TaskStatus {
    NotStartedYet,
    InProgress,
    Done
}
impl FromStr for TaskStatus{
    type Err = String;
    #[inline]
    fn from_str(s: &str) -> Result<TaskStatus,String> {
        match s {
            "done" => Ok(TaskStatus::Done),
            "inprogress" => Ok(TaskStatus::InProgress),
            "notstarted" => Ok(TaskStatus::NotStartedYet),
            _ => Err("NOT FOUND STATUS".to_string()),
        }
    }
}
pub struct Task {
    id: Uuid,
    name: String,
    status: TaskStatus
}    
impl Task {
    /// Constructer for Task structure, returns a task with the given name, auto-generated id, NotStartedYet status
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the task
    pub fn new(name: &str) -> Self {
        Task {
            id: Uuid::new_v4(),
            name: String::from(name),
            status: TaskStatus::NotStartedYet,
        }
    }
    /// Returns the name of the task
    pub fn get_name(&self) -> &String {
        &self.name
    }
    /// Returns the id of the task
    pub fn get_id(&self) -> &Uuid {
        &self.id
    }
    /// Returns the status of the task
    pub fn get_status(&self) -> &TaskStatus {
        &self.status
    }
    /// Sets the name of the task
    /// 
    /// # Arguments
    /// 
    /// * `name` - A string slice that holds the name of the task
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    /// Sets the status of the task
    /// 
    /// # Arguments
    /// 
    /// * `status` - A TaskStatus value that holds the status of the task
    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
    }
}
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Id:{} Name:{} - Status:{:?}", self.get_id(), self.get_name(), self.get_status())
    }
}

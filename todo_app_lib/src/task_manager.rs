use crate::task::{TaskStatus, Task};

pub struct TaskManager {
    todo_tasks: Vec<Task>,
}

impl TaskManager {
    /// Constructer for TaskManager structure, returns a TaskManager instance with the given todo_tasks
    ///
    /// # Arguments
    ///
    /// * `todo_tasks` - A vector of tasks that holds the todo list 
    pub fn new(todo_tasks:Vec<Task>) -> Self {
        Self {
            todo_tasks
        }
    }

    /// Prints all the tasks with details for each task if there are any tasks, otherwise it prints 'Empty todo list'
    fn show_all_tasks(&mut self) {    
        let num_of_tasks = self.todo_tasks.len();
        match num_of_tasks {
            0=>{
                println!("Empty todo list.");
            },
            _=>{
                for task in &self.todo_tasks {
                    println!("{}",task);
                }
            }
        }
    }
    
    /// Adds a task to the todo tasks list, returns a Result enum with error type: &'static str.
    /// 
    /// # Arguments
    /// 
    /// * `name` - A string slice that holds the name of the new task 
    fn add_task(&mut self, name:&str)->Result<(),&'static str>{
        self.todo_tasks.push(Task::new(name));
        println!("Task added successfully");
        Ok(())
    }

    /// Delets a task from the todo tasks list, returns a Result enum with error type: &'static str.
    /// 
    /// # Arguments
    /// 
    /// * `id` - A string slice that holds the id of the wanted task 
    fn delete_task(&mut self, id:&str)->Result<(),&'static str>{
        let index= self.todo_tasks.iter().position(|t| t.get_id().to_string() == id).ok_or("Task id not found!")?;
        self.todo_tasks.remove(index);
        println!("Task with ID {} deleted successfully.", id);
        Ok(())
    }

    /// Updates the name for the passed task, returns a Result enum with error type: &'static str.
    /// 
    /// # Arguments
    /// 
    /// * `id` - A string slice that holds the id of the wanted task
    /// * `name` - A string slice that holds the new name   
    fn update_name(&mut self, id:&str,name:&str)->Result<(),&'static str>{
        let task = self.todo_tasks.iter_mut().find(|t| t.get_id().to_string() == id).ok_or("Task id not found!")?;
        task.set_name(name.to_string());
        println!("Name changed successfully");
        Ok(())
    }

    /// Moves the status for the given task to the given status, returns a Result enum with error type: &'static str.
    /// 
    /// # Arguments
    /// 
    /// * `id` - A string slice that holds the id of the wanted task
    /// * `status` - A string slice that holds the new status 
    fn move_status(&mut self, id:&str,status:&str)->Result<(),&'static str>{
        let task=self.todo_tasks.iter_mut().find(|t| t.get_id().to_string() == id).ok_or("Task id not found!")?;
        let new_status=status.to_lowercase().parse::<TaskStatus>().ok().ok_or("Status not found, the avaliable statuses are NotStartedYet,InProgress, Done.")?;
        task.set_status(new_status);
        

        Ok(())
    }

    /// Parses the CLIs and matchs them with the correct function.
    /// 
    /// # Arguments
    /// 
    /// * `args` - A vector of string slices that holds the terminal command arguments 
    pub fn parse_arguments(&mut self, args:Vec<&str>)->Result<(),&'static str>{
        let length =args.len();
        if length == 0 {
            return Err("No command found");
        }
        match args[0]{
            "show"=>{
                if length!=1 {
                    return Err("Syntax error, refer to 'help' to see the correct syntax");
                }
                self.show_all_tasks();
                Ok(())
            },
            "add"=>{
                if length!=2 {
                    return Err("Syntax error, refer to 'help' to see the correct syntax");
                }
                self.add_task(args[1])
            },
            "delete"=>{
                if length!=2 {
                    return Err("Syntax error, refer to 'help' to see the correct syntax");
                }
                self.delete_task(args[1])
            },
            "update_name"=>{
                if length!=3 {
                    return Err("Syntax error, refer to 'help' to see the correct syntax");
                }
                self.update_name(args[1],args[2])
            },
            "move"=>{
                if length!=3 {
                    return Err("Syntax error, refer to 'help' to see the correct syntax");
                }
                self.move_status(args[1],args[2])
            },
            "help"=>{
                if length!=1 {
                    return Err("Syntax error, refer to 'help' to see the correct syntax");
                }
                println!("the avaliable commands and thier syntaxs are:\n\t1.show - to show all the tasks - syntax: show\n\t2.add - to add a task - syntax: add <task_name>\n\t3.delete - to delete a task - syntax: delete <task-id>\n\t4.move - to change the status for a task - syntax: move <task_id> <new-status>\n\t The avaliable statuses are: NotStartedYet, InProgress, Done\n\t5.update_name - to change the name for a task - syntax: update_name <task_id> <new_name>\n\t6.close - to close the app - syntax: close\n\t7.help - to see all the avaliable commands - syntax: help");
                Ok(())
            },
            "close"=>{
                Err("Closing...")
            }
            _=>{
                println!("command not found. command 'help' shows all avaliable commands.");
                Ok(())
            }
        }
    }
}

extern crate todo_app_lib;
use std::io;
use std::io::Write;
use todo_app_lib::task::Task;
use todo_app_lib::task_manager::TaskManager;
use crossterm::{
    execute,
    style::{Color, Print, SetForegroundColor}
};

fn main() -> std::io::Result<()>{
    execute!(
        io::stdout(),
        SetForegroundColor(Color::DarkBlue),
        Print("\t\t***Welcome to TODO app***\n"),
    )?;
    let tasks: Vec<Task> = Vec::new();
    let mut task_manager = TaskManager::new(tasks);
    loop{
        let mut stdout = io::stdout();
        print!("(todo app) > ");
        stdout.flush().expect("can't flush the stdout");
        let mut input =String::new();
        io::stdin().read_line(&mut input).expect("could not read from stdin"); 
        let args: Vec<&str> = input.trim().split_whitespace().collect();
        
        match task_manager.parse_arguments(args){
            Ok(_) => {}
            Err(err) => {
                println!("{}", err);
                if err=="Closing..."{
                    break;
                }
            }
        }
    }
    Ok(())
}
  

#![allow(unused)]
use std::io::{stdin,stdout,Write};
use std::{ env, fs};
use std::process::exit;
use glob::glob_with;
use std::path::PathBuf;
use std::fs::File;
use colored::Colorize;

/* enum KeyWords {
    Cd,
    Mkdir,
    Ls,
    Pwd,
    Exit
} */
#[derive(Debug)]
struct Command {
    keyword: String,
    arguments: Vec<String>
}

fn main() {
    loop{
        let mut input = String::new();
        print!("> ");
        stdout().flush().expect("Failed to flush stdout");
        stdin().read_line(&mut input).expect("Did not enter correct string");
        let input_commads: Vec<String> = input.trim().split('|').map(String::from).collect();
        for command_str in input_commads {
            let mut parts = command_str.trim().split_whitespace();
            let keyword = parts.next().unwrap_or("").to_string();
            let arguments = parts.map(String::from).collect();
            let command = Command { keyword, arguments };
            execute_command(command);
        }
    }
}

fn execute_command(command: Command) {
    match &*command.keyword {
        "pwd" => {
            if let Err(e) = execute_pwd() {
                eprintln!("Error executing pwd: {}", e);
            }
        },
        "ls" => {
            if let Err(e) = execute_ls() {
                eprintln!("Error executing ls: {}", e);
            }
        },
        "cd" => {
            if let Err(e) = execute_cd(command.arguments) {
                eprintln!("Error executing cd: {}", e);
            }
        },
        "mkdir" => {
            if let Err(e) = execute_mkdir(command.arguments) {
                eprintln!("Error executing mkdir: {}", e);
            }
        },
        "touch" => {
            if let Err(e) = execute_touch(command.arguments) {
                eprintln!("Error executing touch: {}", e);
            }
        },
        "rmdir" => {
            if let Err(e) = execute_rmdir(command.arguments) {
                eprintln!("Error executing rmdir: {}", e);
            }
        },
        "rm" => {
            if let Err(e) = execute_rm(command.arguments) {
                eprintln!("Error executing rm: {}", e);
            }
        },
        "exit" => {
            exit(0)
        },
        &_ => println!("Command not found"),
    }
}

fn execute_pwd() -> Result<(), Box<dyn std::error::Error>>{
    let path = env::current_dir()?;
    println!("{}", path.display());
    
    Ok(())
}

fn execute_ls() -> Result<(), Box<dyn std::error::Error>>{
    let path = env::current_dir()?;
    let mut location_vector: Vec<PathBuf> = Vec::new();
    for element in glob::glob("*").unwrap(){
        if let Ok(path) = element {
            location_vector.push(path);
            //println!("{:#?}", path.display());
        }
    }
    for file in location_vector{
        if file.is_dir() {
            println!("{}/",  file.display().to_string().blue());
        } else {
            println!("{}",  file.display());
        }
    }
    Ok(())
}

fn execute_cd(arguments: Vec<String>) -> Result<(), Box<dyn std::error::Error>>{
    //println!("{arguments:#?}");
    let current_path = env::current_dir()?;
    if arguments.is_empty(){
        env::set_current_dir("/").is_ok();
    } else {
        let mut next_path = PathBuf::from(current_path);
        next_path.push(&arguments[0]);
        env::set_current_dir(next_path)?;
    }
    Ok(())
}

fn execute_mkdir(arguments: Vec<String>) -> Result<(), Box<dyn std::error::Error>>{
     let current_path = env::current_dir()?;
     if arguments.is_empty(){
        eprintln!("Provide directory name");
     }
     let mut dir_path = current_path;
     dir_path.push(&arguments[0]);
     fs::create_dir(dir_path)?;
     Ok(())
}

fn execute_touch(arguments: Vec<String>) -> Result<(), Box<dyn std::error::Error>>{
     let current_path = env::current_dir()?;
     if arguments.is_empty(){
        eprintln!("Provide file name");
     }
     let mut file_path = current_path;
     file_path.push(&arguments[0]);
     File::create(file_path);
     Ok(())
}

fn execute_rmdir(arguments: Vec<String>) -> Result<(), Box<dyn std::error::Error>>{
    let current_path = env::current_dir()?;
    if arguments.is_empty() {
        eprintln!("Provide directory name");
    } 
    let mut dir_path = current_path;
    dir_path.push(&arguments[0]);
    let entries = fs::read_dir(&dir_path)?;
    if entries.count() == 0 {
            fs::remove_dir(dir_path)?;
        } else {
            eprintln!("Directory is not empty!");
        }
    Ok(())
}

fn execute_rm(arguments: Vec<String>) -> Result<(), Box<dyn std::error::Error>>{
    let current_path = env::current_dir()?;
    if arguments.is_empty() {
        eprintln!("Provide file name");
    } 
    let mut file_path = current_path;
    file_path.push(&arguments[0]);
    fs::remove_file(file_path)?;
    Ok(())
}
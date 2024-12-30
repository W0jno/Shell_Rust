#![allow(unused)]
use std::io::{stdin,stdout,Write};
use std::{ env, fs};
use std::process::exit;
use glob::glob_with;
 use std::path::PathBuf;
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
        //let mut vecInput: Vec<String> = Vec::new();
        print!("> ");
        stdout().flush().expect("Failed to flush stdout");
        stdin().read_line(&mut input).expect("Did not enter correct string");
        let mut vec_input: Vec<String> = input.split_whitespace().map(String::from).collect();
        let command = Command {
            keyword: vec_input.remove(0),
            arguments: vec_input.clone(),
        };
        execute_command(command)
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

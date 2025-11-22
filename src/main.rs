use std::io::stdin;
use std::str::FromStr;

mod command;
mod error;
mod folder;

use crate::command::{Argument, Command};
use crate::error::Result;
use crate::folder::Folder;

fn main() {
    println!("Welcome to Doky file simulator");
    let mut folder = Folder::new("C:".to_string());
    loop {
        println!("{}", folder.get_path().unwrap());

        let mut input = String::new();
        stdin().read_line(&mut input).expect("fail...");
        let input = input.trim();

        let result: Result<Command> = Command::from_str(input);

        match result {
            Ok(cmd) => {
                match cmd {
                    Command::Make(name) => {
                        // let _ = Folder::add(&cursor, name);
                        let _ = &folder.create(name);
                    }
                    Command::Open(path) => {
                        let result = folder.open(path);
                        if let Err(e) = result {
                            println!("Error during open: {:?}", e);
                        }
                    }
                    Command::Remove(name, arguments) => {
                        if let Err(e) = folder.remove(name, arguments) {
                            println!("Error during remove: {:?}", e);
                        }
                    }
                    Command::Help => {
                        println!("Command's list:");
                        println!("help: list the commands");
                        println!("exit: close the application");
                    }
                    Command::Exit => {
                        println!("Closing...");
                        break;
                    }
                    _ => {
                        println!("Not implemented...")
                    }
                };
            }
            Err(_) => println!("Command not recognized"),
        };
    }
}

pub trait Node {
    fn open(&mut self, name: String) -> Result<&Folder>;

    fn move_to(&self, path: String) -> Result<()>;

    fn remove(&self, name: String, arguments: Vec<Argument>) -> Result<()>;
}

use std::cell::RefCell;
use std::io::stdin;
use std::rc::Rc;
use std::str::FromStr;

mod error;
mod folder;

use crate::folder::Folder;
use crate::error::{Result, Error};

fn main() {
    println!("Welcome to Doky file simulator");
    let root = Folder::new("C:".to_string());
    let mut cursor = Rc::clone( &root);
    loop {
        println!("{}", cursor.borrow().get_path().unwrap());

        let mut input = String::new();
        stdin().read_line(&mut input).expect("fail...");
        let input = input.trim();

        let result:Result<Command> = Command::from_str(input);

        match result {
            Ok(cmd) => {
                match cmd {
                    Command::Make(name) => { let _ = Folder::add(&cursor, name); },
                    Command::Open(path) => { let result = cursor.borrow().open(&path);
                        match result {
                            Ok(folder) => { cursor = Rc::clone(&folder); },
                            Err(e) => { println!("Error during open: {:?}", e); }
                        }
                    },
                    Command::Help => {
                        println!("Command's list:");
                        println!("help: list the commands");
                        println!("exit: close the application");
                    },
                    Command::Exit => {
                        println!("Closing...");
                        break;
                    }
                    _ => { println!("Not implemented...") } 
                };
            }
            Err(_) => println!("Command not recognized")
        };
    }
}

enum Command {
    Help,
    Display,
    Open(String),
    Make(String),
    Move,
    Remove,
    Exit,
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let args:Vec<&str> = s.split_whitespace().collect();
        match args.as_slice() {
            ["help"] => Ok(Command::Help),
            ["ls"] => Ok(Command::Display),
            ["cd", path] => Ok(Command::Open(path.to_string())),
            ["mk", name] => Ok(Command::Make(name.to_string())),
            ["mv"] => Ok(Command::Move),
            ["rm"] => Ok(Command::Remove),
            ["exit"] => Ok(Command::Exit),
            _ => Err(Error::CommandNotRecognized("Command not recognized"))
        }
        
    }
}

pub trait Node {
    fn open(&self, name:&str) -> Result<Rc<RefCell<Folder>>>;

    fn get_path(&self) -> Result<String>;

    fn move_to(&self, path:&str) -> Result<()>;

    fn remove(&self, name:&str) -> Result<()>;
}
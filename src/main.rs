use std::cell::RefCell;
use std::io::stdin;
use std::rc::Rc;

mod error;
mod folder;

use crate::folder::Folder;
use crate::error::{Result, Error};

fn main() {
    println!("Welcome to Doky file simulator");
    let root = Folder::new("C:".to_string());
    let cursor = &root;
    loop {
        println!("{}", cursor.borrow().get_path().unwrap());

        let mut input = String::new();
        stdin().read_line(&mut input).expect("echec...");
        let input = input.trim();

        match parse_command(&input) {
            Ok(()) => {},
            Err(e) => println!("{:?}", e)
        }

        if input == "exit" {
            println!("Fermeture de l'application...");
            break;
        }
    }
}

fn parse_command(input: &str) -> Result<()> {
    let args:Vec<&str> = input.split_whitespace().collect();
    // println!("{:?} -> {}", args, args[0]);
    let cmd = match args.get(0)  {
        Some(arg) => arg,
        None => return Err(Error::CommandNotRecognized("Command not recognized..."))
    };
    if cmd.to_string() == "mk".to_string() {
        println!("Création");
    }

    if args[0] == "help" {
        println!("Liste des commandes disponible:");
        println!("mk    => Créer un dossier.");
        println!("exit  => Ferme l'application");
    }
    Err(Error::CommandNotRecognized("Command not recognized..."))
}

pub trait Node {
    fn open(&self, name:&str) -> Result<Rc<RefCell<Folder>>>;

    fn get_path(&self) -> Result<String>;
}
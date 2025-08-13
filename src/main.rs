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

        if input == "help" {
            println!("Liste des commandes disponible:")
        }

        if input == "exit" {
            println!("Fermeture de l'application...");
            break;
        }
    }
}

pub trait Node {
    fn open(&self) -> Result<Rc<RefCell<Folder>>>;

    fn get_path(&self) -> Result<String>;
}
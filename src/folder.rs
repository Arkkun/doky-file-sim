use std::rc::{Rc, Weak};
use std::cell::RefCell;

use crate::Node;
use crate::error::{Error, Result};

#[derive(Debug)]
pub struct Folder {
    name: String,
    parent: Option<Weak<RefCell<Folder>>>,
    childs: Vec<Rc<RefCell<Folder>>>
}

impl Folder {
    pub fn new(name:String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Folder { name, parent:None, childs: Vec::new() }))
    }

    fn add(parent: &Rc<RefCell<Folder>>, name:String) -> Result<Rc<RefCell<Folder>>> {
        if !is_valid_name(&name) {
            return Err(Error::InvalidName("Invalid folder name"));
        }
        let child = Folder::new(name);
        child.borrow_mut().parent = Some(Rc::downgrade(parent));
        parent.borrow_mut().childs.push(Rc::clone(&child));
        Ok(Rc::clone(&child))
    }
}

impl Node for Folder {
    fn open(&self, name:&str) -> Result<Rc<RefCell<Folder>>> {
        for child in &self.childs {
            if child.borrow().name == name {
                return Ok(Rc::clone(&child));
            }
        }
        Err(Error::InvalidPath)
    }
    
    fn get_path(&self) -> Result<String> {
        let mut path = "".to_string();
        match &self.parent {
            Some(p) => {
                path += &p.upgrade().unwrap().borrow().get_path().unwrap();
                path += "\\";
                path += &self.name.clone();
                println!("test : {}", path);
             },
            None => path += &self.name.clone(),
        }
        Ok(path.to_string())
    }
}

fn is_valid_name(name:&String) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_sub_folder_success() {
        let root = Folder::new("C:".to_string());
        let sub_name = "Sub";
        let _ = Folder::add(&root, sub_name.to_string());
        let result = root.borrow().open(sub_name);
        assert!(result.is_ok());
        assert_eq!(sub_name.to_string(), result.unwrap().borrow().name);
    }

        #[test]
    fn open_sub_folder_that_not_exist() {
        let root = Folder::new("C:".to_string());
        let sub_name = "Sub";
        let _ = Folder::add(&root, sub_name.to_string());
        let result = root.borrow().open("unknown");
        assert!(result.is_err());
    }

    #[test]
    fn add_sub_folder() {
        let root = Folder::new("C:".to_string());
        let sub_name = "Sub".to_string();
        let result = Folder::add(&root, sub_name.clone());
        assert!(result.is_ok());
        assert_eq!(1, root.borrow().childs.len()); 

        let borrowed_root = &root.borrow();
        let root_child = borrowed_root.childs.get(0).unwrap();

        //Check if sub folder name is correct
        let result_child = result.unwrap();
        assert_eq!(sub_name, result_child.borrow().name.clone());

        //Check if child is correct
        assert_eq!(Rc::as_ptr(&result_child), Rc::as_ptr(&root_child));

        //Check if parent is correct
        let result_parent = result_child.borrow().parent.as_ref().unwrap().upgrade().unwrap();
        assert_eq!(Rc::as_ptr(&root), Rc::as_ptr(&result_parent));
    }

    #[test]
    fn get_folder_path() {
        let root = Folder::new("C:".to_string());
        let result = "C:";
        assert_eq!(result, root.borrow().get_path().unwrap());
    }

    #[test]
    fn get_sub_folder_path() {
        let root = Folder::new("C:".to_string());
        let cursor = Folder::add(&root, "sub".to_string()).unwrap();
        let result = cursor.borrow().get_path();
        assert_eq!("C:\\sub".to_string(), result.unwrap());
    }
}
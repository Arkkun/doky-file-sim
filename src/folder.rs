use std::rc::{Rc, Weak};
use std::cell::RefCell;

use crate::Node;
use crate::error::{Error, Result};

pub struct Folder {
    name: String,
    parent: Option<Weak<RefCell<Folder>>>,
    childs: Vec<Rc<RefCell<Folder>>>
}

impl Folder {
    pub fn new(name:String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Folder { name, parent:None, childs: Vec::new() }))
    }

    fn add(parent: &Rc<RefCell<Folder>>, name:String) -> Result<()> {
        if !is_valid_name(&name) {
            return Err(Error::InvalidName("Invalid folder name"));
        }
        let child = Folder::new(name);
        child.borrow_mut().parent = Some(Rc::downgrade(parent));
        parent.borrow_mut().childs.push(child);
        Ok(())
    }
}

impl Node for Folder {
    fn open(&self) -> Result<Rc<RefCell<Folder>>> {
        todo!()
    }
    
    fn get_path(&self) -> Result<String> {
        Ok(self.name.to_string())
    }
}

fn is_valid_name(name:&String) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_sub_folder() {
        let root = Folder::new("C:".to_string());
        let sub_name = "Sub".to_string();
        let sub = Folder::add(&root, sub_name.clone());
        assert!(sub.is_ok());
        assert_eq!(1, root.borrow().childs.len()); 

        let t = root.borrow();
        let child = &t.childs.get(0).unwrap().borrow();

        //Check if sub folder name is correct
        assert_eq!(sub_name, child.name.clone());

        //Check if parent is correct
        let test_cinq = child.parent.as_ref().unwrap();
        let test_six = test_cinq.upgrade().unwrap();
        assert_eq!(Rc::as_ptr(&root), Rc::as_ptr(&test_six));
    }

    #[test]
    fn get_folder_path() {
        let root = Folder::new("C:".to_string());
        let result = "C:";
        assert_eq!(result, root.borrow().get_path().unwrap());
    }

    // #[test]
    // fn get_sub_folder_path() {
    //     let root = Folder::new("C:".to_string());
    //     let cursor = Folder::add(&root, "sub".to_string()).unwrap();

    //     assert_eq!("C:\\sub".to_string(), &cursor.get_path());
    // }
}
use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

use crate::Node;
use crate::command::Argument;
use crate::error::{Error, Result};

#[derive(Debug)]
pub struct Folder {
    root: Rc<RefCell<FolderCursor>>,
    item: Rc<RefCell<FolderCursor>>,
}

impl Folder {}

#[derive(Debug)]
struct FolderCursor {
    name: String,
    parent: Option<Weak<RefCell<FolderCursor>>>,
    childs: Vec<Rc<RefCell<FolderCursor>>>,
}

impl FolderCursor {
    pub fn new(name: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(FolderCursor {
            name,
            parent: None,
            childs: Vec::new(),
        }))
    }

    pub fn add(
        parent: &Rc<RefCell<FolderCursor>>,
        name: String,
    ) -> Result<Rc<RefCell<FolderCursor>>> {
        if !is_valid_name(&name) {
            return Err(Error::InvalidName("Invalid folder name"));
        }
        let child = FolderCursor::new(name);
        child.borrow_mut().parent = Some(Rc::downgrade(parent));
        parent.borrow_mut().childs.push(Rc::clone(&child));
        Ok(Rc::clone(&child))
    }

    pub fn get_path(&self) -> Result<String> {
        let mut path = "".to_string();
        match &self.parent {
            Some(p) => {
                path += &p.upgrade().unwrap().borrow().get_path().unwrap();
                path += "\\";
                path += &self.name.clone();
            }
            None => path += &self.name.clone(),
        }
        Ok(path.to_string())
    }

    fn open(
        current: &Rc<RefCell<FolderCursor>>,
        path: String,
    ) -> Result<Rc<RefCell<FolderCursor>>> {
        let parts: Vec<&str> = path.split('/').collect();
        let mut cursor: Rc<RefCell<FolderCursor>> = Rc::clone(current);
        for name in parts {
            let mut found = false;
            if name.eq("..") {
                let parent_opt = cursor.borrow().parent.clone();
                match parent_opt {
                    Some(p) => match p.upgrade() {
                        Some(parent_rc) => {
                            cursor = parent_rc;
                            found = true;
                        }
                        None => return Err(Error::InvalidParent),
                    },
                    None => return Err(Error::InvalidParent),
                }
            }

            // search for a child with the given name
            let children = cursor.borrow().childs.clone();
            for child in children.iter() {
                if child.borrow().name == name {
                    cursor = Rc::clone(child);
                    found = true;
                    break;
                }
            }

            if !found {
                return Err(Error::InvalidPath);
            }
        }
        Ok(cursor)
    }

    fn remove(
        current: &Rc<RefCell<FolderCursor>>,
        name: String,
        argments: Vec<Argument>,
    ) -> Result<()> {
        let mut folder = current.borrow_mut();
        if let Some(index) = folder.childs.iter().position(|f| f.borrow().name == name) {
            if !folder.childs[index].borrow().childs.is_empty() {
                let message = format!("Folder {} is not empty", name);
                return Err(Error::FolderNotEmpty(message));
            }
            folder.childs.remove(index);
            Ok(())
        } else {
            Err(Error::InvalidName("Folder not found"))
        }
    }
}

impl Folder {
    pub fn new(name: String) -> Self {
        let folder = FolderCursor::new(name);
        Folder {
            root: Rc::clone(&folder.clone()),
            item: Rc::clone(&folder.clone()),
        }
    }

    pub fn create(&mut self, name: String) -> Result<&Folder> {
        match FolderCursor::add(&self.item, name) {
            Ok(folder) => {
                self.item = folder;
                Ok(self)
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_path(&self) -> Result<String> {
        match self.item.borrow().get_path() {
            Ok(path) => Ok(path),
            Err(e) => Err(e),
        }
    }
}

impl Node for Folder {
    fn open(&mut self, path: String) -> Result<&Folder> {
        match FolderCursor::open(&self.item, path) {
            Ok(cursor) => {
                self.item = cursor;
                Ok(self)
            }
            Err(e) => Err(e),
        }
    }

    fn move_to(&self, path: String) -> Result<()> {
        todo!()
    }

    fn remove(&self, name: String, argments: Vec<Argument>) -> Result<()> {
        FolderCursor::remove(&self.item, name, argments)
    }
}

fn is_valid_name(name: &str) -> bool {
    if name.contains('/') || name.contains('\\') {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use std::result;

    use crate::folder;

    use super::*;

    fn setup() -> Folder {
        let root = FolderCursor::new("C:".to_string());
        let first = FolderCursor::add(&root, "first".to_string());
        let _ = FolderCursor::add(&root, "second".to_string());
        let _ = FolderCursor::add(&first.unwrap(), "sub_first".to_string());
        Folder {
            root: Rc::clone(&root.clone()),
            item: Rc::clone(&root.clone()),
        }
    }

    #[test]
    fn open_folder_with_multiple_level() {
        let mut folder = setup();
        let result = folder.open("first/sub_first".to_string());
        assert!(result.is_ok());
        assert_eq!("sub_first".to_string(), result.unwrap().item.borrow().name);
    }

    #[test]
    fn open_folder_with_multiple_parent_level() {
        let mut folder = setup();
        let result = folder.open("first/..".to_string());
        assert!(result.is_ok());
        assert_eq!("C:".to_string(), result.unwrap().item.borrow().name);
    }

    #[test]
    fn open_sub_folder_success() {
        let mut folder = setup();
        let result = folder.open("first".to_string());
        assert!(result.is_ok());
        assert_eq!("first", result.unwrap().item.borrow().name);
    }

    #[test]
    fn open_parent_folder_success() {
        let mut folder = setup();
        let _ = folder.open("first".to_string());
        let result = folder.open("..".to_string());
        assert!(result.is_ok());
        assert_eq!("C:", result.unwrap().item.borrow().name);
    }

    #[test]
    fn open_invalid_parent_folder() {
        let mut folder = setup();
        let result = folder.open("..".to_string());
        assert!(result.is_err());
        assert_eq!(Error::InvalidParent, result.unwrap_err());
    }

    #[test]
    fn open_sub_folder_that_not_exist() {
        let mut folder = setup();
        let result = folder.open("unknown".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn add_sub_folder() {
        let root = FolderCursor::new("C:".to_string());
        let sub_name = "Sub".to_string();
        let result = FolderCursor::add(&root, sub_name.clone());
        assert!(result.is_ok());
        assert_eq!(1, root.borrow().childs.len());

        let borrowed_root = &root.borrow();
        let root_child = borrowed_root.childs.first().unwrap();

        //Check if sub folder name is correct
        let result_child = result.unwrap();
        assert_eq!(sub_name, result_child.borrow().name.clone());

        //Check if child is correct
        assert_eq!(Rc::as_ptr(&result_child), Rc::as_ptr(root_child));

        //Check if parent is correct
        let result_parent = result_child
            .borrow()
            .parent
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap();
        assert_eq!(Rc::as_ptr(&root), Rc::as_ptr(&result_parent));
    }

    #[test]
    fn get_folder_path() {
        let root = FolderCursor::new("C:".to_string());
        let result = "C:";
        assert_eq!(result, root.borrow().get_path().unwrap());
    }

    #[test]
    fn get_sub_folder_path() {
        let root = FolderCursor::new("C:".to_string());
        let cursor = FolderCursor::add(&root, "sub".to_string()).unwrap();
        let result = cursor.borrow().get_path();
        assert_eq!("C:\\sub".to_string(), result.unwrap());
    }

    #[test]
    fn is_valid_name_success() {
        let name = "ValidName".to_string();
        assert!(is_valid_name(&name));
    }

    #[test]
    fn name_contains_slash_is_invalid() {
        let name = "Invalid/Name".to_string();
        assert!(!is_valid_name(&name));
    }

    #[test]
    fn name_contains_back_slash_is_invalid() {
        let name = "Invalid\\Name".to_string();
        assert!(!is_valid_name(&name));
    }

    #[test]
    fn remove_folder_success() {
        let mut folder = setup();
        let result = folder.remove("second".to_string(), Vec::new());
        assert!(result.is_ok());
        let result = folder.open("second".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn remove_folder_that_not_exist() {
        let folder = setup();
        let result = folder.remove("unknown".to_string(), Vec::new());
        assert!(result.is_err());
        assert_eq!(Error::InvalidName("Folder not found"), result.unwrap_err());
    }

    #[test]
    fn remove_folder_not_empty() {
        let folder = setup();
        let result = folder.remove("first".to_string(), Vec::new());
        assert!(result.is_err());
        assert_eq!(
            Error::FolderNotEmpty("Folder first is not empty".to_string()),
            result.unwrap_err()
        );
    }
}

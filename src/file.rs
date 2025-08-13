struct File {
    name: String,
    extension: String,
    size: usize,
    parent: Option<Box<Folder>>,
}

// impl Root for File {
//     fn open(&self) {
//         todo!()
//     }
    
//     fn get_path(&self) -> String {
//         todo!()
//     }
// }
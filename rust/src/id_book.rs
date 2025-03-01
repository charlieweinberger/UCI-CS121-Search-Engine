use std::path::PathBuf;

pub struct IDBookElement {
    pub id: u16,
    pub url: String,
    pub path: PathBuf,
}

impl IDBookElement {
    pub fn line_to_id_book_element(id: u16, line: &str) -> IDBookElement {
        let mut split = line.split('|');
        let url = split.next().unwrap().trim().to_string();
        let path = PathBuf::from(split.next().unwrap().trim());
        IDBookElement { id, url, path }
    }
}


pub fn main() {
    
}
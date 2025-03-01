use std::{io::BufRead, path::PathBuf};

pub struct IDBookElement {
    pub id: u16,
    pub url: String,
    pub path: PathBuf,
}

impl IDBookElement {
    pub fn new(id: u16, url: String, path: PathBuf) -> Self {
        Self { id, url, path }
    }

    pub fn get_domain(&self) -> String {
        let parts: Vec<&str> = self.url.splitn(4, '/').collect();
        if parts.len() >= 3 {
            format!("{}//{}", parts[0], parts[2])
        } else {
            self.url.to_string()
        }
    }
}

pub fn main() {
    // Given the idbook file,
    let path = PathBuf::from("inverted_index/id_book.txt");
    // read line by line, keep track of the bytes, the lines (id starting with 1), the current domain, and the path to the file as well
    // for every new domain, add the domain to a IDBookElement, as a skip list to write to a file and efficiently search for the domain

    let mut id_book: Vec<IDBookElement> = Vec::new();
    let mut id = 0;
    let mut domain = String::new();
    let file = std::fs::File::open(&path).expect("Could not open the id_book file");
    let mut reader = std::io::BufReader::new(file);
    let mut line = String::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        id += 1;
        let mut split = line.split('|');
        let url = split.next().unwrap().trim();
        let path = PathBuf::from(split.next().unwrap().trim());
        // Extract the scheme and domain (everything before the path)
        let parts: Vec<&str> = url.splitn(4, '/').collect();
        let current_domain = if parts.len() >= 3 {
            format!("{}//{}", parts[0], parts[2])
        } else {
            url.to_string()
        };

        if domain != current_domain {
            id_book.push(IDBookElement::new(id, current_domain.to_string(), path));
            domain = current_domain.to_string();
        }
        line.clear(); // clear the line for the next iteration
    }
}

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
pub struct FileHolder {
    f: File,
}

impl FileHolder {
    pub fn build(file_name: &str) -> Option<Self> {
        let default_path = Path::new("./").join(file_name);

        match File::open(default_path) {
            Ok(fp) => Some(Self { f: fp }),
            Err(_) => None,
        }
    }

    pub fn list_lines(self) -> Vec<u32> {
        BufReader::new(self.f)
            .lines()
            .map(|l| l.unwrap())
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
    }
}

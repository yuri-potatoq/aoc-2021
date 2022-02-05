use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    default::Default,
};

#[derive(Debug)]
pub struct FileHolder {
    f: Option<File>,
}

impl Default for FileHolder {
    fn default() -> Self {
        let path: &'static str = "default.txt";

        match File::create(path) {
            Ok(fp) => Self { f: Some(fp) },
            Err(_) => Self { f: None }
        }
    }
}

impl FileHolder {    
    pub fn build(file_name: &str) -> Option<Self> {
        let default_path = Path::new("./").join(file_name);

        match File::open(default_path) {
            Ok(fp) => Some(Self { f: Some(fp) }),
            Err(_) => None,
        }
    }

    pub fn list_lines_by_int(self) -> Vec<u32> {
        if let Some(f) = self.f {
            BufReader::new(f)
            .lines()
            .map(|l| l.unwrap())
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
        } else { Vec::<u32>::new() }        
    }

    pub fn list_lines(self) -> Vec<String> {
        if let Some(f) = self.f {
            BufReader::new(f)
                .lines()
                .map(|l| l.unwrap())
                .collect::<Vec<String>>()
        } else { Vec::<String>::new() }        
    }
}

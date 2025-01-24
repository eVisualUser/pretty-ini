use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use crate::ini::Ini;

#[derive(Default)]
pub struct IniFile {
    path: PathBuf,
    buffer: Option<Vec<String>>,
}

impl IniFile {
    /// Empty the buffer
    pub fn clear_buffer(&mut self) {
        if let Some(mut buffer) = self.buffer.take() {
            buffer.clear();
        }
    }

    /// Set the buffer storing each line as a string in a Vec<String>
    pub fn set_buffer(&mut self, new_buffer: Vec<String>) {
        self.buffer = Some(new_buffer);
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    /// Set the path, and return if exists
    pub fn set_path(&mut self, path: &str) -> bool {
        self.path = path.parse().unwrap();

        self.exist()
    }

    pub fn exist(&self) -> bool {
        self.path.exists()
    }

    /// Load the file from disk
    pub fn load(&mut self) {
        let mut result = Vec::<String>::new();

        let file = File::open(&self.path).unwrap();
        let reader = BufReader::new(file);
        for (_, line) in reader.lines().enumerate() {
            result.push(line.unwrap());
        }

        self.buffer = Some(result);
    }

    /// Save the file on disk
    pub fn save(&mut self, ini: &mut Ini, new_buffer: Option<Vec<String>>) -> Result<(), std::io::Error> {
        if let Some(new_Buffer) = new_buffer {
            self.buffer = Some(new_Buffer);
        } else {
            self.buffer = Some(ini.make_ini_file_buffer());
        }

        if !self.exist() {
            File::create(&self.path)?;
        }

        match File::options().write(true).open(&self.path) {
            Ok(mut file) => {
                let buffer = self.buffer.clone().unwrap();
                for line in 0..buffer.len() {
                    file.write(format!("{}", buffer[line]).into_bytes().as_ref())?;

                    if line != buffer.len() - 1 {
                        file.write("\n".to_string().into_bytes().as_ref())?;
                    }
                }

                file.flush()
            }
            Err(error) => {
                Err(error)
            }
        }
    }

    pub fn get_buffer(&mut self) -> &mut Option<Vec<String>> {
        &mut self.buffer
    }

    pub fn copy_buffer(&self) -> Option<Vec<String>> { self.buffer.clone() }
}

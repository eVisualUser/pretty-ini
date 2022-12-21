use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;

use crate::ini::Ini;

pub type CryptAction = Option<Box<dyn Fn(Vec<String>) -> Vec<String>>>;

#[derive(Default)]
pub struct IniFile {
    path: String,
    buffer: Option<Vec<String>>,
    uncrypt: CryptAction,
    encrypt: CryptAction,
}

impl IniFile {
    pub fn clear_buffer(&mut self) {
        self.buffer = None;
    }

    pub fn set_buffer(&mut self, new_buffer: Vec<String>) {
        self.buffer = Some(new_buffer);
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn set_path(&mut self, path: &str) {
        self.path = String::from(path);
    }

    pub fn exist(&self) -> bool {
        std::path::Path::new(&self.path).exists()
    }

    pub fn add_uncrypt_action(&mut self, action: CryptAction) {
        self.uncrypt = action;
    }

    pub fn add_encrypt_action(&mut self, action: CryptAction) {
        self.encrypt = action;
    }

    pub fn load(&mut self) {
        let mut result = Vec::<String>::new();

        let file = File::open(&self.path).unwrap();
        let reader = BufReader::new(file);
        for (_, line) in reader.lines().enumerate() {
            result.push(line.unwrap());
        }

        self.buffer = Some(self.crypt(result));
    }

    pub fn save(&mut self, ini: &mut Ini) {
        self.buffer = Some(ini.make_ini_file_buffer());

        std::fs::remove_file(&self.path).unwrap();
        std::fs::File::create(&self.path).unwrap();
        let mut file = File::options().write(true).open(&self.path).unwrap();

        let buffer = self.buffer.clone().unwrap();
        for line in 0..buffer.len() {
            file.write(format!("{}", buffer[line]).into_bytes().as_ref()).unwrap();

            if line != buffer.len() - 1 {
                file.write("\n".to_string().into_bytes().as_ref()).unwrap();
            }
        }

        file.flush().unwrap();
    }

    pub fn decrypt(&self, content: Vec<String>) -> Vec<String> {
        match &self.uncrypt {
            Some(uncrypt) => {
                (uncrypt)(content)
            }
            None => content,
        }
    }

    pub fn crypt(&mut self, content: Vec<String>) -> Vec<String> {
        match &self.encrypt {
            Some(encrypt) => {
                (encrypt)(content)
            }
            None => content,
        }
    }

    pub fn get_content(&self) -> Option<Vec<String>> {
        self.buffer.clone()
    }
}

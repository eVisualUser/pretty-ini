use crate::ini_file::IniFile;
use crate::parser::{ElementType, Parser};
use crate::parser_config::ParserConfig;
use crate::table::Table;
use crate::variable::Variable;

pub const TABLE_NAME_ROOT: &str = "root";
pub const TABLE_ERROR_ROOT: &str = "The table [root] already exist";

#[derive(Default, Debug, Clone)]
pub struct Ini {
    content: Vec<Table>,
    unknow_elements: Vec<String>,
    pub parser: Parser,
}

impl Ini {
    pub fn make_ini_file_buffer(&mut self) -> Vec<String> {
        let mut result = Vec::<String>::new();

        for table in self.content.iter() {
            result.append(&mut table.as_vec_string(&self.parser.config));
        }
        result.append(&mut self.unknow_elements);

        result
    }
}

impl Ini {
    pub fn get_all_table_names(&self) -> Vec<String> {
        let mut result = Vec::new();

        for table in self.content.iter() {
            result.push(table.name.clone());
        }

        result
    }

    pub fn get_all_keys_in_table(&self, table: &str) -> Result<Vec<String>, String> {
        let mut key_list = Vec::<String>::new();

        let table = self.get_table_ref(table)?;

        for var in table.content.iter() {
            key_list.push(var.key.clone());
        }

        if !key_list.is_empty() {
            return Ok(key_list);
        }

        Err(format!("No key found in table: {:?}", table))
    }
}

impl Ini {
    pub fn get_parser_config(&self) -> ParserConfig {
        self.parser.config.clone()
    }

    pub fn get_table_ref_mut(&mut self, name: &str) -> Result<&mut Table, String> {
        for i in 0..self.content.len() {
            if self.content[i].name == name {
                return Ok(&mut self.content[i]);
            }
        }
        Err(format!("Table [{}] not found", name))
    }

    pub fn get_table_ref(&self, name: &str) -> Result<&Table, String> {
        for i in 0..self.content.len() {
            if self.content[i].name == name {
                return Ok(&self.content[i]);
            }
        }
        Err(format!("Table [{}] not found", name))
    }

    pub fn get_table(&self, name: &str) -> Result<Table, String> {
        for i in 0..self.content.len() {
            if self.content[i].name == name {
                return Ok(self.content[i].clone());
            }
        }
        Err(format!("Table [{}] not found", name))
    }

    pub fn set_parser_config(&mut self, config: ParserConfig) {
        self.parser.config = config;
    }

    pub fn get_ref_mut(&mut self, table: &str, var: &str) -> Result<&mut Variable, String> {
        #[allow(unused)]
        let mut result: Result<&mut Variable, String> = Err(String::from("unknow error"));

        let table = self.get_table_ref_mut(table);
        match table {
            Ok(table) => {
                result = table.get_variable_ref_mut(var);
            }
            Err(error) => {
                return Err(error);
            }
        }

        return result;
    }

    pub fn get_ref(&self, table: &str, var: &str) -> Result<&Variable, String> {
        let result: Result<&Variable, String>;

        let table = self.get_table_ref(table);
        match table {
            Ok(table) => {
                result = table.get_variable_ref(var);
            }
            Err(error) => {
                return Err(error);
            }
        }

        return result;
    }

    pub fn get(&self, table: &str, var: &str) -> Result<Variable, String> {
        let result: Result<Variable, String>;

        let table = self.get_table_ref(table);
        match table {
            Ok(table) => {
                result = table.get_variable(var).clone();
            }
            Err(error) => {
                return Err(error);
            }
        }

        return result;
    }
}

impl Ini {
    pub fn load(&mut self, file: &mut IniFile) -> Result<(), String> {
        let lines;

        match file.get_content() {
            Some(content) => {
                lines = content;
            }
            None => {
                if file.exist() {
                    file.load();
                } else {
                    let error = format!("File {:?} not found", file.get_path());
                    return Err(error);
                }
                return self.load(file);
            }
        }

        let mut table = Table::default();
        table.name = String::from(TABLE_NAME_ROOT);

        for line in lines.iter() {
            match self.parser.get_element_type(line) {
                ElementType::Unknow => {
                    if line.trim() != "" {
                        let mut variable = Variable::default();
                        variable.unknow_element = Some(line.to_string());
                        table.content.push(variable);
                    }
                }
                ElementType::Table => {
                    self.content.push(table);

                    let table_name = self.parser.parse_table_title(line);

                    if table_name != TABLE_NAME_ROOT {
                        table = Table::default();
                        table.name = table_name;
                    } else {
                        return Err(String::from(TABLE_ERROR_ROOT));
                    }
                }
                ElementType::Variable => {
                    table.content.push(self.parser.parse_variable(line));
                }
            }
        }
        self.content.push(table);

        file.clear_buffer();

        Ok(())
    }
}

impl Ini {
    pub fn create_table(&mut self, name: &str) {
        let mut table = Table::default();
        table.name = String::from(name);
        self.content.push(table);
    }
}

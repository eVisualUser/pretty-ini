use crate::table::Table;
use crate::parser::{Parser, ElementType};
use crate::parser_config::ParserConfig;
use crate::ini_file::IniFile;
use crate::variable::Variable;

pub const TABLE_NAME_ROOT: &str = "root";
pub const TABLE_ERROR_ROOT: &str = "The table [root] already exist";

#[derive(Default, Debug)]
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

    pub fn get_parser_config(&self) -> ParserConfig {
        self.parser.config.clone()
    }

    pub fn get_table_mut(&mut self, name: &str) -> Result<&mut Table, String> {
        for i in 0..self.content.len() {
            if self.content[i].name == name {
                return Ok(&mut self.content[i]);
            }
        }
        Err(format!("Table [{}] not found", name))
    }

    pub fn set_parser_config(&mut self, config: ParserConfig) {
        self.parser.config = config;
    }

    pub fn get_refmut(&mut self, table: &str, var: &str) -> Result<&mut Variable, String> {
        #[allow(unused)]
        let mut result: Result<&mut Variable, String> = Err(String::from("unknow error"));

        let table = self.get_table_mut(table);
        match table {
            Ok(table) => {
                result = table.get_variable_refmut(var);
            }
            Err(error) => {
                return Err(error);
            }
        }

        return result;
    }

    pub fn load(&mut self, file: &mut IniFile) -> Result<(), String>{
        #[allow(unused)]
        let mut lines = Vec::<String>::new();

        match file.get_content() {
            Some(content) => {
                lines = content;
            }
            None => {
                if file.exist() {
                    file.load();
                }
                else {
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

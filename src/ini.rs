use crate::table::Table;
use crate::parser::{Parser, ElementType};
use crate::parser_config::ParserConfig;
use crate::ini_file::IniFile;
use crate::variable::Variable;

pub const TABLE_NAME_ROOT: &str = "root";

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

    pub fn get_table_mut(&mut self, name: &str) -> Option<&mut Table> {
        for i in 0..self.content.len() {
            if self.content[i].name == name {
                return Some(&mut self.content[i]);
            }
        }
        None
    }

    pub fn set_parser_config(&mut self, config: ParserConfig) {
        self.parser.config = config;
    }

    pub fn load(&mut self, file: &IniFile) {
        let lines = file.get_content().unwrap();

        let mut table = Table::default();
        table.name = String::from(TABLE_NAME_ROOT);

        for line in lines.iter() {
            match self.parser.get_element_type(line) {
                ElementType::Unknow => {
                    if line.trim() != "" {
                        let mut variable = Variable::default();
                        variable.uknow_element = Some(line.to_string());
                        table.content.push(variable);
                    }
                }
                ElementType::Table => {
                    self.content.push(table);
                    table = Table::default();
                    table.name = self.parser.parse_table_title(line);
                }
                ElementType::Variable => {
                    table.content.push(self.parser.parse_variable(line));
                }
            }
        }
        self.content.push(table);
    }
}
